#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct Geofence(::windows::core::IUnknown);
impl Geofence {
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DwellTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn MonitoredStates(&self) -> ::windows::core::Result<MonitoredGeofenceStates> {
        let this = self;
        unsafe {
            let mut result__: MonitoredGeofenceStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MonitoredGeofenceStates>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Geoshape(&self) -> ::windows::core::Result<super::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IGeoshape>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn SingleUse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn CreateWithMonitorStates<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithMonitorStatesAndDwellTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: Param4) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::IGeoshape>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(id: Param0, geoshape: Param1, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: Param4, starttime: Param5, duration: Param6) -> ::windows::core::Result<Geofence> {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), id.into_param().abi(), geoshape.into_param().abi(), monitoredstates, singleuse, dwelltime.into_param().abi(), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<Geofence>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeofenceFactory<R, F: FnOnce(&IGeofenceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geofence, IGeofenceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geofence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geofence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geofence {}
impl ::core::fmt::Debug for Geofence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geofence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geofence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.Geofence;{9c090823-edb8-47e0-8245-5bf61d321f2d})");
}
unsafe impl ::windows::core::Interface for Geofence {
    type Vtable = IGeofenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c090823_edb8_47e0_8245_5bf61d321f2d);
}
impl ::windows::core::RuntimeName for Geofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.Geofence";
}
impl ::core::convert::From<Geofence> for ::windows::core::IUnknown {
    fn from(value: Geofence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geofence> for ::windows::core::IUnknown {
    fn from(value: &Geofence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geofence> for ::windows::core::IInspectable {
    fn from(value: Geofence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geofence> for ::windows::core::IInspectable {
    fn from(value: &Geofence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Geofence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Geofence {}
unsafe impl ::core::marker::Sync for Geofence {}
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct GeofenceMonitor(::windows::core::IUnknown);
impl GeofenceMonitor {
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Status(&self) -> ::windows::core::Result<GeofenceMonitorStatus> {
        let this = self;
        unsafe {
            let mut result__: GeofenceMonitorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceMonitorStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Geofences(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Geofence>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<Geofence>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn LastKnownGeoposition(&self) -> ::windows::core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Geoposition>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GeofenceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGeofenceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Current() -> ::windows::core::Result<GeofenceMonitor> {
        Self::IGeofenceMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceMonitor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeofenceMonitorStatics<R, F: FnOnce(&IGeofenceMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeofenceMonitor, IGeofenceMonitorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GeofenceMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeofenceMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceMonitor {}
impl ::core::fmt::Debug for GeofenceMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceMonitor;{4c0f5f78-1c1f-4621-bbbd-833b92247226})");
}
unsafe impl ::windows::core::Interface for GeofenceMonitor {
    type Vtable = IGeofenceMonitorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c0f5f78_1c1f_4621_bbbd_833b92247226);
}
impl ::windows::core::RuntimeName for GeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
}
impl ::core::convert::From<GeofenceMonitor> for ::windows::core::IUnknown {
    fn from(value: GeofenceMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceMonitor> for ::windows::core::IUnknown {
    fn from(value: &GeofenceMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeofenceMonitor> for ::windows::core::IInspectable {
    fn from(value: GeofenceMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceMonitor> for ::windows::core::IInspectable {
    fn from(value: &GeofenceMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GeofenceMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeofenceMonitor {}
unsafe impl ::core::marker::Sync for GeofenceMonitor {}
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct GeofenceMonitorStatus(pub i32);
impl GeofenceMonitorStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for GeofenceMonitorStatus {}
impl ::core::clone::Clone for GeofenceMonitorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GeofenceMonitorStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GeofenceMonitorStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceMonitorStatus {}
impl ::core::fmt::Debug for GeofenceMonitorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceMonitorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceMonitorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus;i4)");
}
impl ::windows::core::DefaultType for GeofenceMonitorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: Self = Self(0i32);
    pub const Expired: Self = Self(1i32);
}
impl ::core::marker::Copy for GeofenceRemovalReason {}
impl ::core::clone::Clone for GeofenceRemovalReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GeofenceRemovalReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GeofenceRemovalReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceRemovalReason {}
impl ::core::fmt::Debug for GeofenceRemovalReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceRemovalReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceRemovalReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason;i4)");
}
impl ::windows::core::DefaultType for GeofenceRemovalReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl ::core::marker::Copy for GeofenceState {}
impl ::core::clone::Clone for GeofenceState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GeofenceState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GeofenceState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceState {}
impl ::core::fmt::Debug for GeofenceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceState").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GeofenceState {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GeofenceState {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GeofenceState {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GeofenceState {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GeofenceState {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceState;u4)");
}
impl ::windows::core::DefaultType for GeofenceState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct GeofenceStateChangeReport(::windows::core::IUnknown);
impl GeofenceStateChangeReport {
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn NewState(&self) -> ::windows::core::Result<GeofenceState> {
        let this = self;
        unsafe {
            let mut result__: GeofenceState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceState>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Geofence(&self) -> ::windows::core::Result<Geofence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geofence>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn Geoposition(&self) -> ::windows::core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Geoposition>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
    pub fn RemovalReason(&self) -> ::windows::core::Result<GeofenceRemovalReason> {
        let this = self;
        unsafe {
            let mut result__: GeofenceRemovalReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeofenceRemovalReason>(result__)
        }
    }
}
impl ::core::clone::Clone for GeofenceStateChangeReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeofenceStateChangeReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeofenceStateChangeReport {}
impl ::core::fmt::Debug for GeofenceStateChangeReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeofenceStateChangeReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeofenceStateChangeReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport;{9a243c18-2464-4c89-be05-b3ffff5babc5})");
}
unsafe impl ::windows::core::Interface for GeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a243c18_2464_4c89_be05_b3ffff5babc5);
}
impl ::windows::core::RuntimeName for GeofenceStateChangeReport {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
}
impl ::core::convert::From<GeofenceStateChangeReport> for ::windows::core::IUnknown {
    fn from(value: GeofenceStateChangeReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceStateChangeReport> for ::windows::core::IUnknown {
    fn from(value: &GeofenceStateChangeReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeofenceStateChangeReport> for ::windows::core::IInspectable {
    fn from(value: GeofenceStateChangeReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeofenceStateChangeReport> for ::windows::core::IInspectable {
    fn from(value: &GeofenceStateChangeReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GeofenceStateChangeReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeofenceStateChangeReport {}
unsafe impl ::core::marker::Sync for GeofenceStateChangeReport {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofence(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeofence {
    type Vtable = IGeofenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c090823_edb8_47e0_8245_5bf61d321f2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MonitoredGeofenceStates) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeofenceFactory {
    type Vtable = IGeofenceFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x841f624b_325f_4b90_bca7_2b8022a93796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeofenceMonitor {
    type Vtable = IGeofenceMonitorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c0f5f78_1c1f_4621_bbbd_833b92247226);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeofenceMonitorStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceMonitorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeofenceMonitorStatics {
    type Vtable = IGeofenceMonitorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd32fcf_7e75_4899_ace3_2bd0a65cce06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceMonitorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeofenceStateChangeReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a243c18_2464_4c89_be05_b3ffff5babc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeofenceStateChangeReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeofenceState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeofenceRemovalReason) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Devices_Geolocation_Geofencing'*"]
#[repr(transparent)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl ::core::marker::Copy for MonitoredGeofenceStates {}
impl ::core::clone::Clone for MonitoredGeofenceStates {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MonitoredGeofenceStates {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MonitoredGeofenceStates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MonitoredGeofenceStates {}
impl ::core::fmt::Debug for MonitoredGeofenceStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MonitoredGeofenceStates").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MonitoredGeofenceStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MonitoredGeofenceStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MonitoredGeofenceStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MonitoredGeofenceStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MonitoredGeofenceStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for MonitoredGeofenceStates {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates;u4)");
}
impl ::windows::core::DefaultType for MonitoredGeofenceStates {
    type DefaultType = Self;
}
