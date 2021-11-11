#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geofence(pub ::windows::core::IInspectable);
impl Geofence {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn DwellTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn MonitoredStates(&self) -> ::windows::core::Result<MonitoredGeofenceStates> {
        let this = self;
        unsafe {
            let mut result__: MonitoredGeofenceStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MonitoredGeofenceStates>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Geoshape(&self) -> ::windows::core::Result<super::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IGeoshape>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn SingleUse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn CreateWithMonitorStates<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn CreateWithMonitorStatesAndDwellTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: Param4) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        id: Param0,
        geoshape: Param1,
        monitoredstates: MonitoredGeofenceStates,
        singleuse: bool,
        dwelltime: Param4,
        starttime: Param5,
        duration: Param6,
    ) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    pub fn IGeofenceFactory<R, F: FnOnce(&IGeofenceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geofence, IGeofenceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Geofence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.Geofence;{9c090823-edb8-47e0-8245-5bf61d321f2d})");
}
unsafe impl ::windows::core::Interface for Geofence {
    type Vtable = IGeofence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c090823_edb8_47e0_8245_5bf61d321f2d);
}
impl ::windows::core::RuntimeName for Geofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.Geofence";
}
impl ::core::convert::From<Geofence> for ::windows::core::IUnknown {
    fn from(value: Geofence) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geofence> for ::windows::core::IUnknown {
    fn from(value: &Geofence) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geofence> for ::windows::core::IInspectable {
    fn from(value: Geofence) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geofence> for ::windows::core::IInspectable {
    fn from(value: &Geofence) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Geofence {}
unsafe impl ::core::marker::Sync for Geofence {}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeofenceMonitor(pub ::windows::core::IInspectable);
impl GeofenceMonitor {
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Status(&self) -> ::windows::core::Result<GeofenceMonitorStatus> {
        let this = self;
        unsafe {
            let mut result__: GeofenceMonitorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceMonitorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation_Collections`*"]
    pub fn Geofences(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Geofence>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<Geofence>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn LastKnownGeoposition(&self) -> ::windows::core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Geoposition>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn GeofenceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn RemoveGeofenceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation_Collections`*"]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Current() -> ::windows::core::Result<GeofenceMonitor> {
        Self::IGeofenceMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceMonitor>(result__)
        })
    }
    pub fn IGeofenceMonitorStatics<R, F: FnOnce(&IGeofenceMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeofenceMonitor, IGeofenceMonitorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceMonitor;{4c0f5f78-1c1f-4621-bbbd-833b92247226})");
}
unsafe impl ::windows::core::Interface for GeofenceMonitor {
    type Vtable = IGeofenceMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c0f5f78_1c1f_4621_bbbd_833b92247226);
}
impl ::windows::core::RuntimeName for GeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
}
impl ::core::convert::From<GeofenceMonitor> for ::windows::core::IUnknown {
    fn from(value: GeofenceMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeofenceMonitor> for ::windows::core::IUnknown {
    fn from(value: &GeofenceMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeofenceMonitor> for ::windows::core::IInspectable {
    fn from(value: GeofenceMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeofenceMonitor> for ::windows::core::IInspectable {
    fn from(value: &GeofenceMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeofenceMonitor {}
unsafe impl ::core::marker::Sync for GeofenceMonitor {}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeofenceMonitorStatus(pub i32);
impl GeofenceMonitorStatus {
    pub const Ready: GeofenceMonitorStatus = GeofenceMonitorStatus(0i32);
    pub const Initializing: GeofenceMonitorStatus = GeofenceMonitorStatus(1i32);
    pub const NoData: GeofenceMonitorStatus = GeofenceMonitorStatus(2i32);
    pub const Disabled: GeofenceMonitorStatus = GeofenceMonitorStatus(3i32);
    pub const NotInitialized: GeofenceMonitorStatus = GeofenceMonitorStatus(4i32);
    pub const NotAvailable: GeofenceMonitorStatus = GeofenceMonitorStatus(5i32);
}
impl ::core::convert::From<i32> for GeofenceMonitorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeofenceMonitorStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeofenceMonitorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus;i4)");
}
impl ::windows::core::DefaultType for GeofenceMonitorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: GeofenceRemovalReason = GeofenceRemovalReason(0i32);
    pub const Expired: GeofenceRemovalReason = GeofenceRemovalReason(1i32);
}
impl ::core::convert::From<i32> for GeofenceRemovalReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeofenceRemovalReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeofenceRemovalReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason;i4)");
}
impl ::windows::core::DefaultType for GeofenceRemovalReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: GeofenceState = GeofenceState(0u32);
    pub const Entered: GeofenceState = GeofenceState(1u32);
    pub const Exited: GeofenceState = GeofenceState(2u32);
    pub const Removed: GeofenceState = GeofenceState(4u32);
}
impl ::core::convert::From<u32> for GeofenceState {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeofenceState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeofenceState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceState;u4)");
}
impl ::windows::core::DefaultType for GeofenceState {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for GeofenceState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GeofenceState {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GeofenceState {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GeofenceState {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GeofenceState {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeofenceStateChangeReport(pub ::windows::core::IInspectable);
impl GeofenceStateChangeReport {
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn NewState(&self) -> ::windows::core::Result<GeofenceState> {
        let this = self;
        unsafe {
            let mut result__: GeofenceState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceState>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Geofence(&self) -> ::windows::core::Result<Geofence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geofence>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Geoposition(&self) -> ::windows::core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Geoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn RemovalReason(&self) -> ::windows::core::Result<GeofenceRemovalReason> {
        let this = self;
        unsafe {
            let mut result__: GeofenceRemovalReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceRemovalReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceStateChangeReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport;{9a243c18-2464-4c89-be05-b3ffff5babc5})");
}
unsafe impl ::windows::core::Interface for GeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a243c18_2464_4c89_be05_b3ffff5babc5);
}
impl ::windows::core::RuntimeName for GeofenceStateChangeReport {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
}
impl ::core::convert::From<GeofenceStateChangeReport> for ::windows::core::IUnknown {
    fn from(value: GeofenceStateChangeReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeofenceStateChangeReport> for ::windows::core::IUnknown {
    fn from(value: &GeofenceStateChangeReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeofenceStateChangeReport> for ::windows::core::IInspectable {
    fn from(value: GeofenceStateChangeReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeofenceStateChangeReport> for ::windows::core::IInspectable {
    fn from(value: &GeofenceStateChangeReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeofenceStateChangeReport {}
unsafe impl ::core::marker::Sync for GeofenceStateChangeReport {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofence(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeofence {
    type Vtable = IGeofence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c090823_edb8_47e0_8245_5bf61d321f2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofence_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MonitoredGeofenceStates) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofenceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeofenceFactory {
    type Vtable = IGeofenceFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x841f624b_325f_4b90_bca7_2b8022a93796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofenceMonitor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeofenceMonitor {
    type Vtable = IGeofenceMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c0f5f78_1c1f_4621_bbbd_833b92247226);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GeofenceMonitorStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofenceMonitorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeofenceMonitorStatics {
    type Vtable = IGeofenceMonitorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd32fcf_7e75_4899_ace3_2bd0a65cce06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitorStatics_abi(
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
pub struct IGeofenceStateChangeReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a243c18_2464_4c89_be05_b3ffff5babc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceStateChangeReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GeofenceState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GeofenceRemovalReason) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: MonitoredGeofenceStates = MonitoredGeofenceStates(0u32);
    pub const Entered: MonitoredGeofenceStates = MonitoredGeofenceStates(1u32);
    pub const Exited: MonitoredGeofenceStates = MonitoredGeofenceStates(2u32);
    pub const Removed: MonitoredGeofenceStates = MonitoredGeofenceStates(4u32);
}
impl ::core::convert::From<u32> for MonitoredGeofenceStates {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MonitoredGeofenceStates {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MonitoredGeofenceStates {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates;u4)");
}
impl ::windows::core::DefaultType for MonitoredGeofenceStates {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for MonitoredGeofenceStates {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MonitoredGeofenceStates {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MonitoredGeofenceStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MonitoredGeofenceStates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MonitoredGeofenceStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
