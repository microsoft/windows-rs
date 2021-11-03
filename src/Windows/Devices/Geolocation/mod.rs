#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: AltitudeReferenceSystem = AltitudeReferenceSystem(0i32);
    pub const Terrain: AltitudeReferenceSystem = AltitudeReferenceSystem(1i32);
    pub const Ellipsoid: AltitudeReferenceSystem = AltitudeReferenceSystem(2i32);
    pub const Geoid: AltitudeReferenceSystem = AltitudeReferenceSystem(3i32);
    pub const Surface: AltitudeReferenceSystem = AltitudeReferenceSystem(4i32);
}
impl ::std::convert::From<i32> for AltitudeReferenceSystem {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AltitudeReferenceSystem {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AltitudeReferenceSystem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.AltitudeReferenceSystem;i4)");
}
impl ::windows::runtime::DefaultType for AltitudeReferenceSystem {
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Devices_Geolocation`*"]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl BasicGeoposition {}
impl ::std::default::Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BasicGeoposition {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BasicGeoposition").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).finish()
    }
}
impl ::std::cmp::PartialEq for BasicGeoposition {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude
    }
}
impl ::std::cmp::Eq for BasicGeoposition {}
unsafe impl ::windows::runtime::Abi for BasicGeoposition {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BasicGeoposition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Devices.Geolocation.BasicGeoposition;f8;f8;f8)");
}
impl ::windows::runtime::DefaultType for BasicGeoposition {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CivicAddress(::windows::runtime::IInspectable);
impl CivicAddress {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Country(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn State(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn City(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn PostalCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CivicAddress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.CivicAddress;{a8567a1a-64f4-4d48-bcea-f6b008eca34c})");
}
unsafe impl ::windows::runtime::Interface for CivicAddress {
    type Vtable = ICivicAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2824239642, 25844, 19784, [188, 234, 246, 176, 8, 236, 163, 76]);
}
impl ::windows::runtime::RuntimeName for CivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.CivicAddress";
}
impl ::std::convert::From<CivicAddress> for ::windows::runtime::IUnknown {
    fn from(value: CivicAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CivicAddress> for ::windows::runtime::IUnknown {
    fn from(value: &CivicAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CivicAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CivicAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CivicAddress> for ::windows::runtime::IInspectable {
    fn from(value: CivicAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CivicAddress> for ::windows::runtime::IInspectable {
    fn from(value: &CivicAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CivicAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CivicAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CivicAddress {}
unsafe impl ::std::marker::Sync for CivicAddress {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GeoboundingBox(::windows::runtime::IInspectable);
impl GeoboundingBox {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn NorthwestCorner(&self) -> ::windows::runtime::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SoutheastCorner(&self) -> ::windows::runtime::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Center(&self) -> ::windows::runtime::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn MinAltitude(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn MaxAltitude(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn GeoshapeType(&self) -> ::windows::runtime::Result<GeoshapeType> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SpatialReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn AltitudeReferenceSystem(&self) -> ::windows::runtime::Result<AltitudeReferenceSystem> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>, Param1: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1) -> ::windows::runtime::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CreateWithAltitudeReference<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>, Param1: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::runtime::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), altitudereferencesystem, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>, Param1: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::runtime::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn TryCompute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0) -> ::windows::runtime::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), positions.into_param().abi(), &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn TryComputeWithAltitudeReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altituderefsystem: AltitudeReferenceSystem) -> ::windows::runtime::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), positions.into_param().abi(), altituderefsystem, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn TryComputeWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::runtime::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), positions.into_param().abi(), altituderefsystem, spatialreferenceid, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn IGeoboundingBoxFactory<R, F: FnOnce(&IGeoboundingBoxFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GeoboundingBox, IGeoboundingBoxFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGeoboundingBoxStatics<R, F: FnOnce(&IGeoboundingBoxStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GeoboundingBox, IGeoboundingBoxStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeoboundingBox {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeoboundingBox;{0896c80b-274f-43da-9a06-cbfcdaeb4ec2})");
}
unsafe impl ::windows::runtime::Interface for GeoboundingBox {
    type Vtable = IGeoboundingBox_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(144099339, 10063, 17370, [154, 6, 203, 252, 218, 235, 78, 194]);
}
impl ::windows::runtime::RuntimeName for GeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeoboundingBox";
}
impl ::std::convert::From<GeoboundingBox> for ::windows::runtime::IUnknown {
    fn from(value: GeoboundingBox) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GeoboundingBox> for ::windows::runtime::IUnknown {
    fn from(value: &GeoboundingBox) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GeoboundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GeoboundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GeoboundingBox> for ::windows::runtime::IInspectable {
    fn from(value: GeoboundingBox) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GeoboundingBox> for ::windows::runtime::IInspectable {
    fn from(value: &GeoboundingBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GeoboundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GeoboundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<GeoboundingBox> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GeoboundingBox) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&GeoboundingBox> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GeoboundingBox) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for GeoboundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for &GeoboundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::std::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GeoboundingBox {}
unsafe impl ::std::marker::Sync for GeoboundingBox {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geocircle(::windows::runtime::IInspectable);
impl Geocircle {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Center(&self) -> ::windows::runtime::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Radius(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn GeoshapeType(&self) -> ::windows::runtime::Result<GeoshapeType> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SpatialReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn AltitudeReferenceSystem(&self) -> ::windows::runtime::Result<AltitudeReferenceSystem> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64) -> ::windows::runtime::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), position.into_param().abi(), radius, &mut result__).from_abi::<Geocircle>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CreateWithAltitudeReferenceSystem<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::runtime::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), position.into_param().abi(), radius, altitudereferencesystem, &mut result__).from_abi::<Geocircle>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::runtime::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position.into_param().abi(), radius, altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<Geocircle>(result__)
        })
    }
    pub fn IGeocircleFactory<R, F: FnOnce(&IGeocircleFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geocircle, IGeocircleFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geocircle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocircle;{39e45843-a7f9-4e63-92a7-ba0c28d124b1})");
}
unsafe impl ::windows::runtime::Interface for Geocircle {
    type Vtable = IGeocircle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(971266115, 43001, 20067, [146, 167, 186, 12, 40, 209, 36, 177]);
}
impl ::windows::runtime::RuntimeName for Geocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocircle";
}
impl ::std::convert::From<Geocircle> for ::windows::runtime::IUnknown {
    fn from(value: Geocircle) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geocircle> for ::windows::runtime::IUnknown {
    fn from(value: &Geocircle) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geocircle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geocircle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geocircle> for ::windows::runtime::IInspectable {
    fn from(value: Geocircle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geocircle> for ::windows::runtime::IInspectable {
    fn from(value: &Geocircle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geocircle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geocircle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<Geocircle> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Geocircle) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Geocircle> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Geocircle) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for Geocircle {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for &Geocircle {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::std::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Geocircle {}
unsafe impl ::std::marker::Sync for Geocircle {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geocoordinate(::windows::runtime::IInspectable);
impl Geocoordinate {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Latitude(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Longitude(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn Altitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Accuracy(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn AltitudeAccuracy(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn Heading(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn Speed(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Point(&self) -> ::windows::runtime::Result<Geopoint> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateWithPoint>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn PositionSource(&self) -> ::windows::runtime::Result<PositionSource> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__: PositionSource = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PositionSource>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SatelliteData(&self) -> ::windows::runtime::Result<GeocoordinateSatelliteData> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeocoordinateSatelliteData>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn PositionSourceTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateWithPositionSourceTimestamp>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn IsRemoteSource(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateWithRemoteSource>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geocoordinate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocoordinate;{ee21a3aa-976a-4c70-803d-083ea55bcbc4})");
}
unsafe impl ::windows::runtime::Interface for Geocoordinate {
    type Vtable = IGeocoordinate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3995181994, 38762, 19568, [128, 61, 8, 62, 165, 91, 203, 196]);
}
impl ::windows::runtime::RuntimeName for Geocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocoordinate";
}
impl ::std::convert::From<Geocoordinate> for ::windows::runtime::IUnknown {
    fn from(value: Geocoordinate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geocoordinate> for ::windows::runtime::IUnknown {
    fn from(value: &Geocoordinate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geocoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geocoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geocoordinate> for ::windows::runtime::IInspectable {
    fn from(value: Geocoordinate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geocoordinate> for ::windows::runtime::IInspectable {
    fn from(value: &Geocoordinate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geocoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geocoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Geocoordinate {}
unsafe impl ::std::marker::Sync for Geocoordinate {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GeocoordinateSatelliteData(::windows::runtime::IInspectable);
impl GeocoordinateSatelliteData {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn PositionDilutionOfPrecision(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn HorizontalDilutionOfPrecision(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn VerticalDilutionOfPrecision(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn GeometricDilutionOfPrecision(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn TimeDilutionOfPrecision(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeocoordinateSatelliteData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeocoordinateSatelliteData;{c32a74d9-2608-474c-912c-06dd490f4af7})");
}
unsafe impl ::windows::runtime::Interface for GeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3274339545, 9736, 18252, [145, 44, 6, 221, 73, 15, 74, 247]);
}
impl ::windows::runtime::RuntimeName for GeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeocoordinateSatelliteData";
}
impl ::std::convert::From<GeocoordinateSatelliteData> for ::windows::runtime::IUnknown {
    fn from(value: GeocoordinateSatelliteData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GeocoordinateSatelliteData> for ::windows::runtime::IUnknown {
    fn from(value: &GeocoordinateSatelliteData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GeocoordinateSatelliteData> for ::windows::runtime::IInspectable {
    fn from(value: GeocoordinateSatelliteData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GeocoordinateSatelliteData> for ::windows::runtime::IInspectable {
    fn from(value: &GeocoordinateSatelliteData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GeocoordinateSatelliteData {}
unsafe impl ::std::marker::Sync for GeocoordinateSatelliteData {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: GeolocationAccessStatus = GeolocationAccessStatus(0i32);
    pub const Allowed: GeolocationAccessStatus = GeolocationAccessStatus(1i32);
    pub const Denied: GeolocationAccessStatus = GeolocationAccessStatus(2i32);
}
impl ::std::convert::From<i32> for GeolocationAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GeolocationAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GeolocationAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeolocationAccessStatus;i4)");
}
impl ::windows::runtime::DefaultType for GeolocationAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geolocator(::windows::runtime::IInspectable);
impl Geolocator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geolocator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn DesiredAccuracy(&self) -> ::windows::runtime::Result<PositionAccuracy> {
        let this = self;
        unsafe {
            let mut result__: PositionAccuracy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PositionAccuracy>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn MovementThreshold(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SetMovementThreshold(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn ReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SetReportInterval(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn LocationStatus(&self) -> ::windows::runtime::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__: PositionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PositionStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn GetGeopositionAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Geoposition>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn GetGeopositionAsyncWithAgeAndTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, maximumage: Param0, timeout: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), maximumage.into_param().abi(), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Geoposition>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn PositionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn RemovePositionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn DesiredAccuracyInMeters(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::runtime::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn SetDesiredAccuracyInMeters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetGeopositionHistoryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(starttime: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), starttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetGeopositionHistoryWithDurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(starttime: Param0, duration: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn AllowFallbackToConsentlessPositions(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGeolocator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn IsDefaultGeopositionRecommended() -> ::windows::runtime::Result<bool> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn SetDefaultGeoposition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<BasicGeoposition>>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IGeolocatorStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn DefaultGeoposition() -> ::windows::runtime::Result<super::super::Foundation::IReference<BasicGeoposition>> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<BasicGeoposition>>(result__)
        })
    }
    pub fn IGeolocatorStatics<R, F: FnOnce(&IGeolocatorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geolocator, IGeolocatorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGeolocatorStatics2<R, F: FnOnce(&IGeolocatorStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geolocator, IGeolocatorStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geolocator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geolocator;{a9c3bf62-4524-4989-8aa9-de019d2e551f})");
}
unsafe impl ::windows::runtime::Interface for Geolocator {
    type Vtable = IGeolocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2848178018, 17700, 18825, [138, 169, 222, 1, 157, 46, 85, 31]);
}
impl ::windows::runtime::RuntimeName for Geolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geolocator";
}
impl ::std::convert::From<Geolocator> for ::windows::runtime::IUnknown {
    fn from(value: Geolocator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geolocator> for ::windows::runtime::IUnknown {
    fn from(value: &Geolocator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geolocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geolocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geolocator> for ::windows::runtime::IInspectable {
    fn from(value: Geolocator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geolocator> for ::windows::runtime::IInspectable {
    fn from(value: &Geolocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geolocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geolocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Geolocator {}
unsafe impl ::std::marker::Sync for Geolocator {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geopath(::windows::runtime::IInspectable);
impl Geopath {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn Positions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<BasicGeoposition>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn GeoshapeType(&self) -> ::windows::runtime::Result<GeoshapeType> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SpatialReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn AltitudeReferenceSystem(&self) -> ::windows::runtime::Result<AltitudeReferenceSystem> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0) -> ::windows::runtime::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), positions.into_param().abi(), &mut result__).from_abi::<Geopath>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn CreateWithAltitudeReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::runtime::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), positions.into_param().abi(), altitudereferencesystem, &mut result__).from_abi::<Geopath>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::runtime::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), positions.into_param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<Geopath>(result__)
        })
    }
    pub fn IGeopathFactory<R, F: FnOnce(&IGeopathFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geopath, IGeopathFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geopath {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopath;{e53fd7b9-2da4-4714-a652-de8593289898})");
}
unsafe impl ::windows::runtime::Interface for Geopath {
    type Vtable = IGeopath_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3846166457, 11684, 18196, [166, 82, 222, 133, 147, 40, 152, 152]);
}
impl ::windows::runtime::RuntimeName for Geopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopath";
}
impl ::std::convert::From<Geopath> for ::windows::runtime::IUnknown {
    fn from(value: Geopath) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geopath> for ::windows::runtime::IUnknown {
    fn from(value: &Geopath) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geopath {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geopath {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geopath> for ::windows::runtime::IInspectable {
    fn from(value: Geopath) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geopath> for ::windows::runtime::IInspectable {
    fn from(value: &Geopath) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geopath {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geopath {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<Geopath> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Geopath) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Geopath> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Geopath) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for Geopath {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for &Geopath {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::std::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Geopath {}
unsafe impl ::std::marker::Sync for Geopath {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geopoint(::windows::runtime::IInspectable);
impl Geopoint {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn GeoshapeType(&self) -> ::windows::runtime::Result<GeoshapeType> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SpatialReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn AltitudeReferenceSystem(&self) -> ::windows::runtime::Result<AltitudeReferenceSystem> {
        let this = &::windows::runtime::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(position: Param0) -> ::windows::runtime::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<Geopoint>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CreateWithAltitudeReferenceSystem<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(position: Param0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::runtime::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), position.into_param().abi(), altitudereferencesystem, &mut result__).from_abi::<Geopoint>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<'a, Param0: ::windows::runtime::IntoParam<'a, BasicGeoposition>>(position: Param0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::runtime::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position.into_param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<Geopoint>(result__)
        })
    }
    pub fn IGeopointFactory<R, F: FnOnce(&IGeopointFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geopoint, IGeopointFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geopoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopoint;{6bfa00eb-e56e-49bb-9caf-cbaa78a8bcef})");
}
unsafe impl ::windows::runtime::Interface for Geopoint {
    type Vtable = IGeopoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1811546347, 58734, 18875, [156, 175, 203, 170, 120, 168, 188, 239]);
}
impl ::windows::runtime::RuntimeName for Geopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopoint";
}
impl ::std::convert::From<Geopoint> for ::windows::runtime::IUnknown {
    fn from(value: Geopoint) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geopoint> for ::windows::runtime::IUnknown {
    fn from(value: &Geopoint) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geopoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geopoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geopoint> for ::windows::runtime::IInspectable {
    fn from(value: Geopoint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geopoint> for ::windows::runtime::IInspectable {
    fn from(value: &Geopoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geopoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geopoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<Geopoint> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Geopoint) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Geopoint> for IGeoshape {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Geopoint) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for Geopoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGeoshape> for &Geopoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGeoshape> {
        ::std::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Geopoint {}
unsafe impl ::std::marker::Sync for Geopoint {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geoposition(::windows::runtime::IInspectable);
impl Geoposition {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Coordinate(&self) -> ::windows::runtime::Result<Geocoordinate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Geocoordinate>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn CivicAddress(&self) -> ::windows::runtime::Result<CivicAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CivicAddress>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn VenueData(&self) -> ::windows::runtime::Result<VenueData> {
        let this = &::windows::runtime::Interface::cast::<IGeoposition2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VenueData>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geoposition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geoposition;{c18d0454-7d41-4ff7-a957-9dffb4ef7f5b})");
}
unsafe impl ::windows::runtime::Interface for Geoposition {
    type Vtable = IGeoposition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3247244372, 32065, 20471, [169, 87, 157, 255, 180, 239, 127, 91]);
}
impl ::windows::runtime::RuntimeName for Geoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geoposition";
}
impl ::std::convert::From<Geoposition> for ::windows::runtime::IUnknown {
    fn from(value: Geoposition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geoposition> for ::windows::runtime::IUnknown {
    fn from(value: &Geoposition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geoposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geoposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geoposition> for ::windows::runtime::IInspectable {
    fn from(value: Geoposition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geoposition> for ::windows::runtime::IInspectable {
    fn from(value: &Geoposition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geoposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geoposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Geoposition {}
unsafe impl ::std::marker::Sync for Geoposition {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: GeoshapeType = GeoshapeType(0i32);
    pub const Geocircle: GeoshapeType = GeoshapeType(1i32);
    pub const Geopath: GeoshapeType = GeoshapeType(2i32);
    pub const GeoboundingBox: GeoshapeType = GeoshapeType(3i32);
}
impl ::std::convert::From<i32> for GeoshapeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GeoshapeType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GeoshapeType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeoshapeType;i4)");
}
impl ::windows::runtime::DefaultType for GeoshapeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Geovisit(::windows::runtime::IInspectable);
impl Geovisit {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Geoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn StateChange(&self) -> ::windows::runtime::Result<VisitStateChange> {
        let this = self;
        unsafe {
            let mut result__: VisitStateChange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VisitStateChange>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geovisit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geovisit;{b1877a76-9ef6-41ab-a0dd-793ece76e2de})");
}
unsafe impl ::windows::runtime::Interface for Geovisit {
    type Vtable = IGeovisit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2978445942, 40694, 16811, [160, 221, 121, 62, 206, 118, 226, 222]);
}
impl ::windows::runtime::RuntimeName for Geovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geovisit";
}
impl ::std::convert::From<Geovisit> for ::windows::runtime::IUnknown {
    fn from(value: Geovisit) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Geovisit> for ::windows::runtime::IUnknown {
    fn from(value: &Geovisit) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Geovisit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Geovisit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Geovisit> for ::windows::runtime::IInspectable {
    fn from(value: Geovisit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Geovisit> for ::windows::runtime::IInspectable {
    fn from(value: &Geovisit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Geovisit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Geovisit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Geovisit {}
unsafe impl ::std::marker::Sync for Geovisit {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GeovisitMonitor(::windows::runtime::IInspectable);
impl GeovisitMonitor {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GeovisitMonitor, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn MonitoringScope(&self) -> ::windows::runtime::Result<VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__: VisitMonitoringScope = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VisitMonitoringScope>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Start(&self, value: VisitMonitoringScope) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn VisitStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn RemoveVisitStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation`*"]
    pub fn GetLastReportAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Geovisit>> {
        Self::IGeovisitMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Geovisit>>(result__)
        })
    }
    pub fn IGeovisitMonitorStatics<R, F: FnOnce(&IGeovisitMonitorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GeovisitMonitor, IGeovisitMonitorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeovisitMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitMonitor;{80118aaf-5944-4591-83c1-396647f54f2c})");
}
unsafe impl ::windows::runtime::Interface for GeovisitMonitor {
    type Vtable = IGeovisitMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2148633263, 22852, 17809, [131, 193, 57, 102, 71, 245, 79, 44]);
}
impl ::windows::runtime::RuntimeName for GeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitMonitor";
}
impl ::std::convert::From<GeovisitMonitor> for ::windows::runtime::IUnknown {
    fn from(value: GeovisitMonitor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GeovisitMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &GeovisitMonitor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GeovisitMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GeovisitMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GeovisitMonitor> for ::windows::runtime::IInspectable {
    fn from(value: GeovisitMonitor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GeovisitMonitor> for ::windows::runtime::IInspectable {
    fn from(value: &GeovisitMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GeovisitMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GeovisitMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GeovisitMonitor {}
unsafe impl ::std::marker::Sync for GeovisitMonitor {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GeovisitStateChangedEventArgs(::windows::runtime::IInspectable);
impl GeovisitStateChangedEventArgs {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Visit(&self) -> ::windows::runtime::Result<Geovisit> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Geovisit>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeovisitStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitStateChangedEventArgs;{ceb4d1ff-8b53-4968-beed-4cecd029ce15})");
}
unsafe impl ::windows::runtime::Interface for GeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3467956735, 35667, 18792, [190, 237, 76, 236, 208, 41, 206, 21]);
}
impl ::windows::runtime::RuntimeName for GeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
}
impl ::std::convert::From<GeovisitStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GeovisitStateChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GeovisitStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GeovisitStateChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GeovisitStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GeovisitStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GeovisitStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GeovisitStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GeovisitStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for GeovisitStateChangedEventArgs {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GeovisitTriggerDetails(::windows::runtime::IInspectable);
impl GeovisitTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation`, `Foundation_Collections`*"]
    pub fn ReadReports(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<Geovisit>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Geovisit>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeovisitTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitTriggerDetails;{ea770d9e-d1c9-454b-99b7-b2f8cdd2482f})");
}
unsafe impl ::windows::runtime::Interface for GeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3933670814, 53705, 17739, [153, 183, 178, 248, 205, 210, 72, 47]);
}
impl ::windows::runtime::RuntimeName for GeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitTriggerDetails";
}
impl ::std::convert::From<GeovisitTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: GeovisitTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GeovisitTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &GeovisitTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GeovisitTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: GeovisitTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GeovisitTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &GeovisitTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GeovisitTriggerDetails {}
unsafe impl ::std::marker::Sync for GeovisitTriggerDetails {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICivicAddress(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICivicAddress {
    type Vtable = ICivicAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2824239642, 25844, 19784, [188, 234, 246, 176, 8, 236, 163, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoboundingBox(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeoboundingBox {
    type Vtable = IGeoboundingBox_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(144099339, 10063, 17370, [154, 6, 203, 252, 218, 235, 78, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBox_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BasicGeoposition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BasicGeoposition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BasicGeoposition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeoboundingBoxFactory {
    type Vtable = IGeoboundingBoxFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1308337545, 1041, 19132, [179, 181, 91, 188, 203, 87, 217, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeoboundingBoxStatics {
    type Vtable = IGeoboundingBoxStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1740113672, 58906, 19664, [132, 27, 147, 35, 55, 146, 181, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positions: ::windows::runtime::RawPtr, altituderefsystem: AltitudeReferenceSystem, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positions: ::windows::runtime::RawPtr, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocircle(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocircle {
    type Vtable = IGeocircle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(971266115, 43001, 20067, [146, 167, 186, 12, 40, 209, 36, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BasicGeoposition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocircleFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocircleFactory {
    type Vtable = IGeocircleFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2950058783, 29361, 20349, [135, 204, 78, 212, 201, 132, 156, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircleFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: BasicGeoposition, radius: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinate(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinate {
    type Vtable = IGeocoordinate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3995181994, 38762, 19568, [128, 61, 8, 62, 165, 91, 203, 196]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3274339545, 9736, 18252, [145, 44, 6, 221, 73, 15, 74, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinateSatelliteData2 {
    type Vtable = IGeocoordinateSatelliteData2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1981582589, 41373, 23121, [128, 245, 113, 103, 97, 21, 72, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinateWithPoint {
    type Vtable = IGeocoordinateWithPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4276749605, 53804, 19782, [181, 39, 11, 150, 6, 111, 199, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinateWithPositionData {
    type Vtable = IGeocoordinateWithPositionData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2514891966, 56278, 16556, [184, 242, 166, 92, 3, 64, 217, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PositionSource) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinateWithPositionSourceTimestamp {
    type Vtable = IGeocoordinateWithPositionSourceTimestamp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2235825154, 51697, 17936, [175, 224, 139, 195, 166, 168, 112, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeocoordinateWithRemoteSource {
    type Vtable = IGeocoordinateWithRemoteSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(964488151, 60984, 24379, [137, 0, 196, 167, 188, 156, 249, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeolocator {
    type Vtable = IGeolocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2848178018, 17700, 18825, [138, 169, 222, 1, 157, 46, 85, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PositionAccuracy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PositionAccuracy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PositionStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocator2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeolocator2 {
    type Vtable = IGeolocator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3518246509, 34961, 17332, [173, 54, 39, 198, 254, 154, 151, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocatorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeolocatorStatics {
    type Vtable = IGeolocatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2593027441, 11765, 17809, [159, 135, 235, 95, 216, 148, 233, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocatorStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeolocatorStatics2 {
    type Vtable = IGeolocatorStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2570064290, 64028, 17969, [167, 29, 13, 190, 177, 37, 13, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeolocatorWithScalarAccuracy {
    type Vtable = IGeolocatorWithScalarAccuracy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2532692929, 47119, 17930, [153, 77, 169, 108, 71, 165, 26, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopath(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeopath {
    type Vtable = IGeopath_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3846166457, 11684, 18196, [166, 82, 222, 133, 147, 40, 152, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopath_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopathFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeopathFactory {
    type Vtable = IGeopathFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(666806728, 51175, 17241, [155, 155, 252, 163, 224, 94, 245, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopathFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positions: ::windows::runtime::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positions: ::windows::runtime::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopoint(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeopoint {
    type Vtable = IGeopoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1811546347, 58734, 18875, [156, 175, 203, 170, 120, 168, 188, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BasicGeoposition) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopointFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeopointFactory {
    type Vtable = IGeopointFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3681258803, 30397, 20016, [138, 247, 168, 68, 220, 55, 183, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopointFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: BasicGeoposition, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoposition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeoposition {
    type Vtable = IGeoposition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3247244372, 32065, 20471, [169, 87, 157, 255, 180, 239, 127, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoposition2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeoposition2 {
    type Vtable = IGeoposition2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2137192087, 34417, 19213, [134, 248, 71, 74, 132, 150, 24, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Geolocation`*"]
pub struct IGeoshape(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeoshape {
    type Vtable = IGeoshape_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3382485679, 50985, 17345, [143, 171, 214, 222, 201, 20, 223, 126]);
}
impl IGeoshape {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn GeoshapeType(&self) -> ::windows::runtime::Result<GeoshapeType> {
        let this = self;
        unsafe {
            let mut result__: GeoshapeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn SpatialReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn AltitudeReferenceSystem(&self) -> ::windows::runtime::Result<AltitudeReferenceSystem> {
        let this = self;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGeoshape {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c99ca2af-c729-43c1-8fab-d6dec914df7e}");
}
impl ::std::convert::From<IGeoshape> for ::windows::runtime::IUnknown {
    fn from(value: IGeoshape) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGeoshape> for ::windows::runtime::IUnknown {
    fn from(value: &IGeoshape) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGeoshape {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGeoshape {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IGeoshape> for ::windows::runtime::IInspectable {
    fn from(value: IGeoshape) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGeoshape> for ::windows::runtime::IInspectable {
    fn from(value: &IGeoshape) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IGeoshape {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IGeoshape {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoshape_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GeoshapeType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AltitudeReferenceSystem) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisit(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeovisit {
    type Vtable = IGeovisit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2978445942, 40694, 16811, [160, 221, 121, 62, 206, 118, 226, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VisitStateChange) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitMonitor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeovisitMonitor {
    type Vtable = IGeovisitMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2148633263, 22852, 17809, [131, 193, 57, 102, 71, 245, 79, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VisitMonitoringScope) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: VisitMonitoringScope) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeovisitMonitorStatics {
    type Vtable = IGeovisitMonitorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3170465447, 48114, 19677, [149, 207, 85, 76, 130, 237, 251, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3467956735, 35667, 18792, [190, 237, 76, 236, 208, 41, 206, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3933670814, 53705, 17739, [153, 183, 178, 248, 205, 210, 72, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(931503333, 40222, 18117, [191, 59, 106, 216, 202, 193, 160, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(877908698, 35987, 16657, [162, 5, 154, 236, 252, 155, 229, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PositionStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVenueData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVenueData {
    type Vtable = IVenueData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1727238535, 24803, 19247, [181, 39, 79, 83, 241, 195, 198, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVenueData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: PositionAccuracy = PositionAccuracy(0i32);
    pub const High: PositionAccuracy = PositionAccuracy(1i32);
}
impl ::std::convert::From<i32> for PositionAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PositionAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PositionAccuracy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionAccuracy;i4)");
}
impl ::windows::runtime::DefaultType for PositionAccuracy {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PositionChangedEventArgs(::windows::runtime::IInspectable);
impl PositionChangedEventArgs {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Geoposition>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PositionChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.PositionChangedEventArgs;{37859ce5-9d1e-46c5-bf3b-6ad8cac1a093})");
}
unsafe impl ::windows::runtime::Interface for PositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(931503333, 40222, 18117, [191, 59, 106, 216, 202, 193, 160, 147]);
}
impl ::windows::runtime::RuntimeName for PositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.PositionChangedEventArgs";
}
impl ::std::convert::From<PositionChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PositionChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PositionChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PositionChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PositionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PositionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PositionChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PositionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PositionChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PositionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PositionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PositionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PositionChangedEventArgs {}
unsafe impl ::std::marker::Sync for PositionChangedEventArgs {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PositionSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PositionSource {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PositionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionSource;i4)");
}
impl ::windows::runtime::DefaultType for PositionSource {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PositionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PositionStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PositionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionStatus;i4)");
}
impl ::windows::runtime::DefaultType for PositionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct StatusChangedEventArgs(::windows::runtime::IInspectable);
impl StatusChangedEventArgs {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__: PositionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PositionStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StatusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.StatusChangedEventArgs;{3453d2da-8c93-4111-a205-9aecfc9be5c0})");
}
unsafe impl ::windows::runtime::Interface for StatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(877908698, 35987, 16657, [162, 5, 154, 236, 252, 155, 229, 192]);
}
impl ::windows::runtime::RuntimeName for StatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.StatusChangedEventArgs";
}
impl ::std::convert::From<StatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: StatusChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&StatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &StatusChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &StatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<StatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: StatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &StatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for StatusChangedEventArgs {}
unsafe impl ::std::marker::Sync for StatusChangedEventArgs {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VenueData(::windows::runtime::IInspectable);
impl VenueData {
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation`*"]
    pub fn Level(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VenueData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.VenueData;{66f39187-60e3-4b2f-b527-4f53f1c3c677})");
}
unsafe impl ::windows::runtime::Interface for VenueData {
    type Vtable = IVenueData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1727238535, 24803, 19247, [181, 39, 79, 83, 241, 195, 198, 119]);
}
impl ::windows::runtime::RuntimeName for VenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.VenueData";
}
impl ::std::convert::From<VenueData> for ::windows::runtime::IUnknown {
    fn from(value: VenueData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VenueData> for ::windows::runtime::IUnknown {
    fn from(value: &VenueData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VenueData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VenueData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VenueData> for ::windows::runtime::IInspectable {
    fn from(value: VenueData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VenueData> for ::windows::runtime::IInspectable {
    fn from(value: &VenueData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VenueData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VenueData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VenueData {}
unsafe impl ::std::marker::Sync for VenueData {}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: VisitMonitoringScope = VisitMonitoringScope(0i32);
    pub const City: VisitMonitoringScope = VisitMonitoringScope(1i32);
}
impl ::std::convert::From<i32> for VisitMonitoringScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisitMonitoringScope {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VisitMonitoringScope {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitMonitoringScope;i4)");
}
impl ::windows::runtime::DefaultType for VisitMonitoringScope {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: VisitStateChange = VisitStateChange(0i32);
    pub const Arrived: VisitStateChange = VisitStateChange(1i32);
    pub const Departed: VisitStateChange = VisitStateChange(2i32);
    pub const OtherMovement: VisitStateChange = VisitStateChange(3i32);
}
impl ::std::convert::From<i32> for VisitStateChange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisitStateChange {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VisitStateChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitStateChange;i4)");
}
impl ::windows::runtime::DefaultType for VisitStateChange {
    type DefaultType = Self;
}
