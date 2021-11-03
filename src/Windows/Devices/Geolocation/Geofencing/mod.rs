#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct Geofence(::windows::runtime::IInspectable);
impl Geofence {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn DwellTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn MonitoredStates(&self) -> ::windows::runtime::Result<MonitoredGeofenceStates> {
        let this = self;
        unsafe {
            let mut result__: MonitoredGeofenceStates = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MonitoredGeofenceStates>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Geoshape(&self) -> ::windows::runtime::Result<super::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::IGeoshape>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn SingleUse(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1) -> ::windows::runtime::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn CreateWithMonitorStates<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows::runtime::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn CreateWithMonitorStatesAndDwellTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::IGeoshape>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: Param4) -> ::windows::runtime::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::IGeoshape>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>, Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        id: Param0,
        geoshape: Param1,
        monitoredstates: MonitoredGeofenceStates,
        singleuse: bool,
        dwelltime: Param4,
        starttime: Param5,
        duration: Param6,
    ) -> ::windows::runtime::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    pub fn IGeofenceFactory<R, F: FnOnce(&IGeofenceFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Geofence, IGeofenceFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Geofence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.Geofence;{9c090823-edb8-47e0-8245-5bf61d321f2d})");
}
unsafe impl ::windows::runtime::Interface for Geofence {
    type Vtable = IGeofence_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2617837603, 60856, 18400, [130, 69, 91, 246, 29, 50, 31, 45]);
}
impl ::windows::runtime::RuntimeName for Geofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.Geofence";
}
unsafe impl ::std::marker::Send for Geofence {}
unsafe impl ::std::marker::Sync for Geofence {}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GeofenceMonitor(::windows::runtime::IInspectable);
impl GeofenceMonitor {
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<GeofenceMonitorStatus> {
        let this = self;
        unsafe {
            let mut result__: GeofenceMonitorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceMonitorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation_Collections`*"]
    pub fn Geofences(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<Geofence>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<Geofence>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn LastKnownGeoposition(&self) -> ::windows::runtime::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Geoposition>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn GeofenceStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn RemoveGeofenceStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation_Collections`*"]
    pub fn ReadReports(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Current() -> ::windows::runtime::Result<GeofenceMonitor> {
        Self::IGeofenceMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceMonitor>(result__)
        })
    }
    pub fn IGeofenceMonitorStatics<R, F: FnOnce(&IGeofenceMonitorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GeofenceMonitor, IGeofenceMonitorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeofenceMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceMonitor;{4c0f5f78-1c1f-4621-bbbd-833b92247226})");
}
unsafe impl ::windows::runtime::Interface for GeofenceMonitor {
    type Vtable = IGeofenceMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1276075896, 7199, 17953, [187, 189, 131, 59, 146, 36, 114, 38]);
}
impl ::windows::runtime::RuntimeName for GeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
}
unsafe impl ::std::marker::Send for GeofenceMonitor {}
unsafe impl ::std::marker::Sync for GeofenceMonitor {}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for GeofenceMonitorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GeofenceMonitorStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GeofenceMonitorStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus;i4)");
}
impl ::windows::runtime::DefaultType for GeofenceMonitorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: GeofenceRemovalReason = GeofenceRemovalReason(0i32);
    pub const Expired: GeofenceRemovalReason = GeofenceRemovalReason(1i32);
}
impl ::std::convert::From<i32> for GeofenceRemovalReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GeofenceRemovalReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GeofenceRemovalReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason;i4)");
}
impl ::windows::runtime::DefaultType for GeofenceRemovalReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: GeofenceState = GeofenceState(0u32);
    pub const Entered: GeofenceState = GeofenceState(1u32);
    pub const Exited: GeofenceState = GeofenceState(2u32);
    pub const Removed: GeofenceState = GeofenceState(4u32);
}
impl ::std::convert::From<u32> for GeofenceState {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GeofenceState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GeofenceState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceState;u4)");
}
impl ::windows::runtime::DefaultType for GeofenceState {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GeofenceState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GeofenceState {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GeofenceState {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GeofenceState {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GeofenceState {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GeofenceStateChangeReport(::windows::runtime::IInspectable);
impl GeofenceStateChangeReport {
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn NewState(&self) -> ::windows::runtime::Result<GeofenceState> {
        let this = self;
        unsafe {
            let mut result__: GeofenceState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceState>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Geofence(&self) -> ::windows::runtime::Result<Geofence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Geofence>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn Geoposition(&self) -> ::windows::runtime::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Geoposition>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
    pub fn RemovalReason(&self) -> ::windows::runtime::Result<GeofenceRemovalReason> {
        let this = self;
        unsafe {
            let mut result__: GeofenceRemovalReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceRemovalReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GeofenceStateChangeReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport;{9a243c18-2464-4c89-be05-b3ffff5babc5})");
}
unsafe impl ::windows::runtime::Interface for GeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2586065944, 9316, 19593, [190, 5, 179, 255, 255, 91, 171, 197]);
}
impl ::windows::runtime::RuntimeName for GeofenceStateChangeReport {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
}
unsafe impl ::std::marker::Send for GeofenceStateChangeReport {}
unsafe impl ::std::marker::Sync for GeofenceStateChangeReport {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofence(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeofence {
    type Vtable = IGeofence_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2617837603, 60856, 18400, [130, 69, 91, 246, 29, 50, 31, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofence_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MonitoredGeofenceStates) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofenceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeofenceFactory {
    type Vtable = IGeofenceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2216649291, 12895, 19344, [188, 167, 43, 128, 34, 169, 55, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, geoshape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, geoshape: ::windows::runtime::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, geoshape: ::windows::runtime::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, geoshape: ::windows::runtime::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofenceMonitor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeofenceMonitor {
    type Vtable = IGeofenceMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1276075896, 7199, 17953, [187, 189, 131, 59, 146, 36, 114, 38]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GeofenceMonitorStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeofenceMonitorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeofenceMonitorStatics {
    type Vtable = IGeofenceMonitorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(768815055, 32373, 18585, [172, 227, 43, 208, 166, 92, 206, 6]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitorStatics_abi(
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
pub struct IGeofenceStateChangeReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2586065944, 9316, 19593, [190, 5, 179, 255, 255, 91, 171, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceStateChangeReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GeofenceState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GeofenceRemovalReason) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Geolocation_Geofencing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: MonitoredGeofenceStates = MonitoredGeofenceStates(0u32);
    pub const Entered: MonitoredGeofenceStates = MonitoredGeofenceStates(1u32);
    pub const Exited: MonitoredGeofenceStates = MonitoredGeofenceStates(2u32);
    pub const Removed: MonitoredGeofenceStates = MonitoredGeofenceStates(4u32);
}
impl ::std::convert::From<u32> for MonitoredGeofenceStates {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MonitoredGeofenceStates {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MonitoredGeofenceStates {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates;u4)");
}
impl ::windows::runtime::DefaultType for MonitoredGeofenceStates {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MonitoredGeofenceStates {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MonitoredGeofenceStates {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MonitoredGeofenceStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MonitoredGeofenceStates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MonitoredGeofenceStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
