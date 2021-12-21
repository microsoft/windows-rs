#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Networking_Sockets'*"]
pub struct BandwidthStatistics {
    pub OutboundBitsPerSecond: u64,
    pub InboundBitsPerSecond: u64,
    pub OutboundBitsPerSecondInstability: u64,
    pub InboundBitsPerSecondInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
impl ::core::marker::Copy for BandwidthStatistics {}
impl ::core::clone::Clone for BandwidthStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BandwidthStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BandwidthStatistics").field("OutboundBitsPerSecond", &self.OutboundBitsPerSecond).field("InboundBitsPerSecond", &self.InboundBitsPerSecond).field("OutboundBitsPerSecondInstability", &self.OutboundBitsPerSecondInstability).field("InboundBitsPerSecondInstability", &self.InboundBitsPerSecondInstability).field("OutboundBandwidthPeaked", &self.OutboundBandwidthPeaked).field("InboundBandwidthPeaked", &self.InboundBandwidthPeaked).finish()
    }
}
unsafe impl ::windows::core::Abi for BandwidthStatistics {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BandwidthStatistics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.Sockets.BandwidthStatistics;u8;u8;u8;u8;b1;b1)");
}
impl ::windows::core::DefaultType for BandwidthStatistics {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for BandwidthStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BandwidthStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for BandwidthStatistics {}
impl ::core::default::Default for BandwidthStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ControlChannelTrigger(::windows::core::IUnknown);
impl ControlChannelTrigger {
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ControlChannelTriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ServerKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetServerKeepAliveIntervalInMinutes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CurrentKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransportObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'ApplicationModel_Background'*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn KeepAliveTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Background::IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'ApplicationModel_Background'*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn PushNotificationTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Background::IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn UsingTransport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, transport: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), transport.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn WaitForPushEnabled(&self) -> ::windows::core::Result<ControlChannelTriggerStatus> {
        let this = self;
        unsafe {
            let mut result__: ControlChannelTriggerStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ControlChannelTriggerStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn DecreaseNetworkKeepAliveInterval(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn FlushTransport(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IControlChannelTrigger2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CreateControlChannelTrigger<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(channelid: Param0, serverkeepaliveintervalinminutes: u32) -> ::windows::core::Result<ControlChannelTrigger> {
        Self::IControlChannelTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channelid.into_param().abi(), serverkeepaliveintervalinminutes, &mut result__).from_abi::<ControlChannelTrigger>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CreateControlChannelTriggerEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(channelid: Param0, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType) -> ::windows::core::Result<ControlChannelTrigger> {
        Self::IControlChannelTriggerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), channelid.into_param().abi(), serverkeepaliveintervalinminutes, resourcerequesttype, &mut result__).from_abi::<ControlChannelTrigger>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IControlChannelTriggerFactory<R, F: FnOnce(&IControlChannelTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ControlChannelTrigger, IControlChannelTriggerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ControlChannelTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ControlChannelTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ControlChannelTrigger {}
impl ::core::fmt::Debug for ControlChannelTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ControlChannelTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ControlChannelTrigger;{7d1431a7-ee96-40e8-a199-8703cd969ec3})");
}
unsafe impl ::windows::core::Interface for ControlChannelTrigger {
    type Vtable = IControlChannelTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1431a7_ee96_40e8_a199_8703cd969ec3);
}
impl ::windows::core::RuntimeName for ControlChannelTrigger {
    const NAME: &'static str = "Windows.Networking.Sockets.ControlChannelTrigger";
}
impl ::core::convert::From<ControlChannelTrigger> for ::windows::core::IUnknown {
    fn from(value: ControlChannelTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ControlChannelTrigger> for ::windows::core::IUnknown {
    fn from(value: &ControlChannelTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ControlChannelTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ControlChannelTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ControlChannelTrigger> for ::windows::core::IInspectable {
    fn from(value: ControlChannelTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ControlChannelTrigger> for ::windows::core::IInspectable {
    fn from(value: &ControlChannelTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ControlChannelTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ControlChannelTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ControlChannelTrigger> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ControlChannelTrigger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ControlChannelTrigger> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ControlChannelTrigger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ControlChannelTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ControlChannelTrigger {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ControlChannelTrigger {}
unsafe impl ::core::marker::Sync for ControlChannelTrigger {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ControlChannelTriggerResetReason(pub i32);
impl ControlChannelTriggerResetReason {
    pub const FastUserSwitched: Self = Self(0i32);
    pub const LowPowerExit: Self = Self(1i32);
    pub const QuietHoursExit: Self = Self(2i32);
    pub const ApplicationRestart: Self = Self(3i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResetReason {}
impl ::core::clone::Clone for ControlChannelTriggerResetReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ControlChannelTriggerResetReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ControlChannelTriggerResetReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ControlChannelTriggerResetReason {}
impl ::core::fmt::Debug for ControlChannelTriggerResetReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerResetReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ControlChannelTriggerResetReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerResetReason;i4)");
}
impl ::windows::core::DefaultType for ControlChannelTriggerResetReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ControlChannelTriggerResourceType(pub i32);
impl ControlChannelTriggerResourceType {
    pub const RequestSoftwareSlot: Self = Self(0i32);
    pub const RequestHardwareSlot: Self = Self(1i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResourceType {}
impl ::core::clone::Clone for ControlChannelTriggerResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ControlChannelTriggerResourceType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ControlChannelTriggerResourceType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ControlChannelTriggerResourceType {}
impl ::core::fmt::Debug for ControlChannelTriggerResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerResourceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ControlChannelTriggerResourceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerResourceType;i4)");
}
impl ::windows::core::DefaultType for ControlChannelTriggerResourceType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ControlChannelTriggerStatus(pub i32);
impl ControlChannelTriggerStatus {
    pub const HardwareSlotRequested: Self = Self(0i32);
    pub const SoftwareSlotAllocated: Self = Self(1i32);
    pub const HardwareSlotAllocated: Self = Self(2i32);
    pub const PolicyError: Self = Self(3i32);
    pub const SystemError: Self = Self(4i32);
    pub const TransportDisconnected: Self = Self(5i32);
    pub const ServiceUnavailable: Self = Self(6i32);
}
impl ::core::marker::Copy for ControlChannelTriggerStatus {}
impl ::core::clone::Clone for ControlChannelTriggerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ControlChannelTriggerStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ControlChannelTriggerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ControlChannelTriggerStatus {}
impl ::core::fmt::Debug for ControlChannelTriggerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ControlChannelTriggerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerStatus;i4)");
}
impl ::windows::core::DefaultType for ControlChannelTriggerStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct DatagramSocket(::windows::core::IUnknown);
impl DatagramSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DatagramSocket, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Control(&self) -> ::windows::core::Result<DatagramSocketControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DatagramSocketControl>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<DatagramSocketInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DatagramSocketInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, remotehostname: Param0, remoteservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithEndpointPairAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EndpointPair>>(&self, endpointpair: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), endpointpair.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BindServiceNameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localservicename: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), localservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BindEndpointAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localhostname: Param0, localservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), localhostname.into_param().abi(), localservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn JoinMulticastGroup<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>>(&self, host: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), host.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetOutputStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, remotehostname: Param0, remoteservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetOutputStreamWithEndpointPairAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EndpointPair>>(&self, endpointpair: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), endpointpair.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Networking_Connectivity'*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub fn BindServiceNameAndAdapterAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Connectivity::NetworkAdapter>>(&self, localservicename: Param0, adapter: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), localservicename.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn EnableTransferOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskid: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), taskid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn EnableTransferOwnershipWithConnectedStandbyAction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskid: Param0, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), taskid.into_param().abi(), connectedstandbyaction).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransferOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, socketid: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), socketid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransferOwnershipWithContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SocketActivityContext>>(&self, socketid: Param0, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), socketid.into_param().abi(), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TransferOwnershipWithContextAndKeepAliveTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SocketActivityContext>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, socketid: Param0, data: Param1, keepalivetime: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), socketid.into_param().abi(), data.into_param().abi(), keepalivetime.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetEndpointPairsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(remotehostname: Param0, remoteservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IDatagramSocketStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetEndpointPairsWithSortOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(remotehostname: Param0, remoteservicename: Param1, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IDatagramSocketStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), sortoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDatagramSocketStatics<R, F: FnOnce(&IDatagramSocketStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DatagramSocket, IDatagramSocketStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DatagramSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DatagramSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocket {}
impl ::core::fmt::Debug for DatagramSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DatagramSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocket;{7fe25bbb-c3bc-4677-8446-ca28a465a3af})");
}
unsafe impl ::windows::core::Interface for DatagramSocket {
    type Vtable = IDatagramSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe25bbb_c3bc_4677_8446_ca28a465a3af);
}
impl ::windows::core::RuntimeName for DatagramSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocket";
}
impl ::core::convert::From<DatagramSocket> for ::windows::core::IUnknown {
    fn from(value: DatagramSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocket> for ::windows::core::IUnknown {
    fn from(value: &DatagramSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DatagramSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DatagramSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DatagramSocket> for ::windows::core::IInspectable {
    fn from(value: DatagramSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocket> for ::windows::core::IInspectable {
    fn from(value: &DatagramSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DatagramSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DatagramSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DatagramSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DatagramSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DatagramSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DatagramSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for DatagramSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &DatagramSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DatagramSocket {}
unsafe impl ::core::marker::Sync for DatagramSocket {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct DatagramSocketControl(::windows::core::IUnknown);
impl DatagramSocketControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__: SocketQualityOfService = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketQualityOfService>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn InboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetInboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn DontFragment(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetDontFragment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn MulticastOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDatagramSocketControl3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetMulticastOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDatagramSocketControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for DatagramSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DatagramSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocketControl {}
impl ::core::fmt::Debug for DatagramSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocketControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DatagramSocketControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocketControl;{52ac3f2e-349a-4135-bb58-b79b2647d390})");
}
unsafe impl ::windows::core::Interface for DatagramSocketControl {
    type Vtable = IDatagramSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52ac3f2e_349a_4135_bb58_b79b2647d390);
}
impl ::windows::core::RuntimeName for DatagramSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketControl";
}
impl ::core::convert::From<DatagramSocketControl> for ::windows::core::IUnknown {
    fn from(value: DatagramSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocketControl> for ::windows::core::IUnknown {
    fn from(value: &DatagramSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DatagramSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DatagramSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DatagramSocketControl> for ::windows::core::IInspectable {
    fn from(value: DatagramSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocketControl> for ::windows::core::IInspectable {
    fn from(value: &DatagramSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DatagramSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DatagramSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DatagramSocketControl {}
unsafe impl ::core::marker::Sync for DatagramSocketControl {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct DatagramSocketInformation(::windows::core::IUnknown);
impl DatagramSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DatagramSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DatagramSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocketInformation {}
impl ::core::fmt::Debug for DatagramSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DatagramSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocketInformation;{5f1a569a-55fb-48cd-9706-7a974f7b1585})");
}
unsafe impl ::windows::core::Interface for DatagramSocketInformation {
    type Vtable = IDatagramSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f1a569a_55fb_48cd_9706_7a974f7b1585);
}
impl ::windows::core::RuntimeName for DatagramSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketInformation";
}
impl ::core::convert::From<DatagramSocketInformation> for ::windows::core::IUnknown {
    fn from(value: DatagramSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &DatagramSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DatagramSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DatagramSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DatagramSocketInformation> for ::windows::core::IInspectable {
    fn from(value: DatagramSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &DatagramSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DatagramSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DatagramSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DatagramSocketInformation {}
unsafe impl ::core::marker::Sync for DatagramSocketInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct DatagramSocketMessageReceivedEventArgs(::windows::core::IUnknown);
impl DatagramSocketMessageReceivedEventArgs {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::DataReader>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for DatagramSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DatagramSocketMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocketMessageReceivedEventArgs {}
impl ::core::fmt::Debug for DatagramSocketMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocketMessageReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DatagramSocketMessageReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs;{9e2ddca2-1712-4ce4-b179-8c652c6d107e})");
}
unsafe impl ::windows::core::Interface for DatagramSocketMessageReceivedEventArgs {
    type Vtable = IDatagramSocketMessageReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d107e);
}
impl ::windows::core::RuntimeName for DatagramSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs";
}
impl ::core::convert::From<DatagramSocketMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DatagramSocketMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocketMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DatagramSocketMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DatagramSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DatagramSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DatagramSocketMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DatagramSocketMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DatagramSocketMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DatagramSocketMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DatagramSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DatagramSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DatagramSocketMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for DatagramSocketMessageReceivedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IControlChannelTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IControlChannelTrigger {
    type Vtable = IControlChannelTriggerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1431a7_ee96_40e8_a199_8703cd969ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IControlChannelTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IControlChannelTrigger2 {
    type Vtable = IControlChannelTrigger2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf00d237_51be_4514_9725_3556e1879580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTrigger2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IControlChannelTriggerEventDetails(::windows::core::IUnknown);
impl IControlChannelTriggerEventDetails {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ControlChannelTrigger(&self) -> ::windows::core::Result<ControlChannelTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ControlChannelTrigger>(result__)
        }
    }
}
impl ::core::convert::From<IControlChannelTriggerEventDetails> for ::windows::core::IInspectable {
    fn from(value: IControlChannelTriggerEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlChannelTriggerEventDetails> for ::windows::core::IInspectable {
    fn from(value: &IControlChannelTriggerEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IControlChannelTriggerEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IControlChannelTriggerEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IControlChannelTriggerEventDetails> for ::windows::core::IUnknown {
    fn from(value: IControlChannelTriggerEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlChannelTriggerEventDetails> for ::windows::core::IUnknown {
    fn from(value: &IControlChannelTriggerEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlChannelTriggerEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IControlChannelTriggerEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IControlChannelTriggerEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IControlChannelTriggerEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChannelTriggerEventDetails {}
impl ::core::fmt::Debug for IControlChannelTriggerEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChannelTriggerEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IControlChannelTriggerEventDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1b36e047-89bb-4236-96ac-71d012bb4869}");
}
unsafe impl ::windows::core::Interface for IControlChannelTriggerEventDetails {
    type Vtable = IControlChannelTriggerEventDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b36e047_89bb_4236_96ac_71d012bb4869);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerEventDetailsVtbl(
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
pub struct IControlChannelTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IControlChannelTriggerFactory {
    type Vtable = IControlChannelTriggerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda4b7cf0_8d71_446f_88c3_b95184a2d6cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IControlChannelTriggerResetEventDetails(::windows::core::IUnknown);
impl IControlChannelTriggerResetEventDetails {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ResetReason(&self) -> ::windows::core::Result<ControlChannelTriggerResetReason> {
        let this = self;
        unsafe {
            let mut result__: ControlChannelTriggerResetReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ControlChannelTriggerResetReason>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn HardwareSlotReset(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SoftwareSlotReset(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IControlChannelTriggerResetEventDetails> for ::windows::core::IInspectable {
    fn from(value: IControlChannelTriggerResetEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlChannelTriggerResetEventDetails> for ::windows::core::IInspectable {
    fn from(value: &IControlChannelTriggerResetEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IControlChannelTriggerResetEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IControlChannelTriggerResetEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IControlChannelTriggerResetEventDetails> for ::windows::core::IUnknown {
    fn from(value: IControlChannelTriggerResetEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlChannelTriggerResetEventDetails> for ::windows::core::IUnknown {
    fn from(value: &IControlChannelTriggerResetEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlChannelTriggerResetEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IControlChannelTriggerResetEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IControlChannelTriggerResetEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IControlChannelTriggerResetEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChannelTriggerResetEventDetails {}
impl ::core::fmt::Debug for IControlChannelTriggerResetEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChannelTriggerResetEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IControlChannelTriggerResetEventDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6851038e-8ec4-42fe-9bb2-21e91b7bfcb1}");
}
unsafe impl ::windows::core::Interface for IControlChannelTriggerResetEventDetails {
    type Vtable = IControlChannelTriggerResetEventDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6851038e_8ec4_42fe_9bb2_21e91b7bfcb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerResetEventDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocket {
    type Vtable = IDatagramSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe25bbb_c3bc_4677_8446_ca28a465a3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocket2 {
    type Vtable = IDatagramSocket2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd83ba354_9a9d_4185_a20a_1424c9c2a7cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocket2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocket3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocket3 {
    type Vtable = IDatagramSocket3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37544f09_ab92_4306_9ac1_0c381283d9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocket3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketControl {
    type Vtable = IDatagramSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52ac3f2e_349a_4135_bb58_b79b2647d390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketControl2 {
    type Vtable = IDatagramSocketControl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ead5c2_979c_4415_82a1_3cfaf646c192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketControl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketControl3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketControl3 {
    type Vtable = IDatagramSocketControl3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4eb8256_1f6d_4598_9b57_d42a001df349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketControl3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketInformation {
    type Vtable = IDatagramSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f1a569a_55fb_48cd_9706_7a974f7b1585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketMessageReceivedEventArgs {
    type Vtable = IDatagramSocketMessageReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d107e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketMessageReceivedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketStatics {
    type Vtable = IDatagramSocketStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9c62aee_1494_4a21_bb7e_8589fc751d9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocket {
    type Vtable = IMessageWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33727d08_34d5_4746_ad7b_8dde5bc2ef88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocket2 {
    type Vtable = IMessageWebSocket2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed0cee7_f9c8_440a_9ad5_737281d9742e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocket2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocket3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocket3 {
    type Vtable = IMessageWebSocket3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59d9defb_71af_4349_8487_911fcf681597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocket3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketControl {
    type Vtable = IMessageWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8118388a_c629_4f0a_80fb_81fc05538862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketControl2 {
    type Vtable = IMessageWebSocketControl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe30fd791_080c_400a_a712_27dfa9e744d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketControl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MessageWebSocketReceiveMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MessageWebSocketReceiveMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketMessageReceivedEventArgs {
    type Vtable = IMessageWebSocketMessageReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x478c22ac_4c4b_42ed_9ed7_1ef9f94fa3d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketMessageReceivedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketMessageReceivedEventArgs2 {
    type Vtable = IMessageWebSocketMessageReceivedEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89ce06fd_dd6f_4a07_87f9_f9eb4d89d83d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketMessageReceivedEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerMessageWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerMessageWebSocket {
    type Vtable = IServerMessageWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3ac9240_813b_5efd_7e11_ae2305fc77f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerMessageWebSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerMessageWebSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerMessageWebSocketControl {
    type Vtable = IServerMessageWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c2f051_1c1f_587a_4519_2181610192b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerMessageWebSocketControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerMessageWebSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerMessageWebSocketInformation {
    type Vtable = IServerMessageWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc32b45f_4448_5505_6cc9_09afa8915f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerMessageWebSocketInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerStreamWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerStreamWebSocket {
    type Vtable = IServerStreamWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ced5bbf_74f6_55e4_79df_9132680dfee8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerStreamWebSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerStreamWebSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerStreamWebSocketInformation {
    type Vtable = IServerStreamWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc32b45f_4448_5505_6cc9_09aba8915f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerStreamWebSocketInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityContext {
    type Vtable = ISocketActivityContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43b04d64_4c85_4396_a637_1d973f6ebd49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityContextFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityContextFactory {
    type Vtable = ISocketActivityContextFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb99fc3c3_088c_4388_83ae_2525138e049a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityContextFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityInformation {
    type Vtable = ISocketActivityInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d8a42e4_a87e_4b74_9968_185b2511defe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityInformationStatics {
    type Vtable = ISocketActivityInformationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8570b47a_7e7d_4736_8041_1327a6543c56);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityInformationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityTriggerDetails {
    type Vtable = ISocketActivityTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45f406a7_fc9f_4f81_acad_355fef51e67b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityTriggerReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketErrorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketErrorStatics {
    type Vtable = ISocketErrorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x828337f4_7d56_4d8e_b7b4_a07dd7c1bca9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketErrorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SocketErrorStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocket {
    type Vtable = IStreamSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a22cf3_fc7b_4857_af38_f6e7de6a5b49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectionlevel: SocketProtectionLevel, validationhostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocket2 {
    type Vtable = IStreamSocket2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29d0e575_f314_4d09_adf0_0fbd967fbd9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocket2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocket3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocket3 {
    type Vtable = IStreamSocket3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f430b00_9d28_4854_bac3_2301941ec223);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocket3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl {
    type Vtable = IStreamSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe25adf1_92ab_4af3_9992_0f4c85e36cc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl2 {
    type Vtable = IStreamSocketControl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2d09a56_060f_44c1_b8e2_1fbf60bd62c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl3 {
    type Vtable = IStreamSocketControl3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc56a444c_4e74_403e_894c_b31cae5c7342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl4 {
    type Vtable = IStreamSocketControl4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x964e2b3d_ec27_4888_b3ce_c74b418423ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketProtectionLevel) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketInformation {
    type Vtable = IStreamSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b80ae30_5e68_4205_88f0_dc85d2e25ded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundTripTimeStatistics) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketInformation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketInformation2 {
    type Vtable = IStreamSocketInformation2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12c28452_4bdc_4ee4_976a_cf130e9d92e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketInformation2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListener {
    type Vtable = IStreamSocketListenerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff513437_df9f_4df0_bf82_0ec5d7b35aae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListener2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListener2 {
    type Vtable = IStreamSocketListener2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x658dc13e_bb3e_4458_b232_ed1088694b98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListener2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListener3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListener3 {
    type Vtable = IStreamSocketListener3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4798201c_bdf8_4919_8542_28d450e74507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListener3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerConnectionReceivedEventArgs {
    type Vtable = IStreamSocketListenerConnectionReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c472ea9_373f_447b_85b1_ddd4548803ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerConnectionReceivedEventArgsVtbl(
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
pub struct IStreamSocketListenerControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerControl {
    type Vtable = IStreamSocketListenerControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20d8c576_8d8a_4dba_9722_a16c4d984980);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerControl2 {
    type Vtable = IStreamSocketListenerControl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x948bb665_2c3e_404b_b8b0_8eb249a2b0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerControl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerInformation {
    type Vtable = IStreamSocketListenerInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62ba82f_a63a_430b_bf62_29e93e5633b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketStatics {
    type Vtable = IStreamSocketStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa420bc4a_6e2e_4af5_b556_355ae0cd4f29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocket {
    type Vtable = IStreamWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4a49d8_b289_45bb_97eb_c7525205a843);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocket2 {
    type Vtable = IStreamWebSocket2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa4d08cb_93f5_4678_8236_57cce5417ed5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocket2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocketControl {
    type Vtable = IStreamWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4f478b1_a45a_48db_953a_645b7d964c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocketControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocketControl2 {
    type Vtable = IStreamWebSocketControl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x215d9f7e_fa58_40da_9f11_a48dafe95037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocketControl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IWebSocket(::windows::core::IUnknown);
impl IWebSocket {
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CloseWithStatus<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, code: u16, reason: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), code, reason.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IWebSocket> for ::windows::core::IInspectable {
    fn from(value: IWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocket> for ::windows::core::IInspectable {
    fn from(value: &IWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebSocket> for ::windows::core::IUnknown {
    fn from(value: IWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocket> for ::windows::core::IUnknown {
    fn from(value: &IWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocket {}
impl ::core::fmt::Debug for IWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f877396f-99b1-4e18-bc08-850c9adf156e}");
}
unsafe impl ::windows::core::Interface for IWebSocket {
    type Vtable = IWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf877396f_99b1_4e18_bc08_850c9adf156e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebSocketClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebSocketClosedEventArgs {
    type Vtable = IWebSocketClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb78d07_d0a8_4703_a091_c8c2c0915bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketClosedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IWebSocketControl(::windows::core::IUnknown);
impl IWebSocketControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::convert::From<IWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: IWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: &IWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: IWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: &IWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketControl {}
impl ::core::fmt::Debug for IWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebSocketControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2ec4bdc3-d9a5-455a-9811-de24d45337e9}");
}
unsafe impl ::windows::core::Interface for IWebSocketControl {
    type Vtable = IWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec4bdc3_d9a5_455a_9811_de24d45337e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IWebSocketControl2(::windows::core::IUnknown);
impl IWebSocketControl2 {
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::convert::From<IWebSocketControl2> for ::windows::core::IInspectable {
    fn from(value: IWebSocketControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketControl2> for ::windows::core::IInspectable {
    fn from(value: &IWebSocketControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebSocketControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebSocketControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebSocketControl2> for ::windows::core::IUnknown {
    fn from(value: IWebSocketControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketControl2> for ::windows::core::IUnknown {
    fn from(value: &IWebSocketControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebSocketControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebSocketControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebSocketControl2> for IWebSocketControl {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebSocketControl2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebSocketControl2> for IWebSocketControl {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebSocketControl2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl> for IWebSocketControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl> for &IWebSocketControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl> {
        ::core::convert::TryInto::<IWebSocketControl>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebSocketControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebSocketControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketControl2 {}
impl ::core::fmt::Debug for IWebSocketControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebSocketControl2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79c3be03-f2ca-461e-af4e-9665bc2d0620}");
}
unsafe impl ::windows::core::Interface for IWebSocketControl2 {
    type Vtable = IWebSocketControl2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c3be03_f2ca_461e_af4e_9665bc2d0620);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketControl2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebSocketErrorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebSocketErrorStatics {
    type Vtable = IWebSocketErrorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27cdf35b_1f61_4709_8e02_61283ada4e9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketErrorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web"))] usize,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IWebSocketInformation(::windows::core::IUnknown);
impl IWebSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: IWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &IWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: IWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &IWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketInformation {}
impl ::core::fmt::Debug for IWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5e01e316-c92a-47a5-b25f-07847639d181}");
}
unsafe impl ::windows::core::Interface for IWebSocketInformation {
    type Vtable = IWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e01e316_c92a_47a5_b25f_07847639d181);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct IWebSocketInformation2(::windows::core::IUnknown);
impl IWebSocketInformation2 {
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IWebSocketInformation2> for ::windows::core::IInspectable {
    fn from(value: IWebSocketInformation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketInformation2> for ::windows::core::IInspectable {
    fn from(value: &IWebSocketInformation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebSocketInformation2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebSocketInformation2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebSocketInformation2> for ::windows::core::IUnknown {
    fn from(value: IWebSocketInformation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebSocketInformation2> for ::windows::core::IUnknown {
    fn from(value: &IWebSocketInformation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebSocketInformation2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebSocketInformation2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebSocketInformation2> for IWebSocketInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebSocketInformation2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebSocketInformation2> for IWebSocketInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebSocketInformation2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation> for IWebSocketInformation2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation> for &IWebSocketInformation2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation> {
        ::core::convert::TryInto::<IWebSocketInformation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebSocketInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebSocketInformation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketInformation2 {}
impl ::core::fmt::Debug for IWebSocketInformation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketInformation2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebSocketInformation2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ce1d39ce-a1b7-4d43-8269-8d5b981bd47a}");
}
unsafe impl ::windows::core::Interface for IWebSocketInformation2 {
    type Vtable = IWebSocketInformation2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce1d39ce_a1b7_4d43_8269_8d5b981bd47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketInformation2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebSocketServerCustomValidationRequestedEventArgs {
    type Vtable = IWebSocketServerCustomValidationRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffeffe48_022a_4ab7_8b36_e10af4640e6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketServerCustomValidationRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct MessageWebSocket(::windows::core::IUnknown);
impl MessageWebSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MessageWebSocket, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Control(&self) -> ::windows::core::Result<MessageWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MessageWebSocketControl>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<MessageWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MessageWebSocketInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocket2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocket2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendNonfinalFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocket3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendFinalFrameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocket3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CloseWithStatus<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, code: u16, reason: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), code, reason.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MessageWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocket {}
impl ::core::fmt::Debug for MessageWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MessageWebSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocket;{33727d08-34d5-4746-ad7b-8dde5bc2ef88})");
}
unsafe impl ::windows::core::Interface for MessageWebSocket {
    type Vtable = IMessageWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33727d08_34d5_4746_ad7b_8dde5bc2ef88);
}
impl ::windows::core::RuntimeName for MessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocket";
}
impl ::core::convert::From<MessageWebSocket> for ::windows::core::IUnknown {
    fn from(value: MessageWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocket> for ::windows::core::IUnknown {
    fn from(value: &MessageWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MessageWebSocket> for ::windows::core::IInspectable {
    fn from(value: MessageWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocket> for ::windows::core::IInspectable {
    fn from(value: &MessageWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MessageWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MessageWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MessageWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MessageWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<MessageWebSocket> for IWebSocket {
    type Error = ::windows::core::Error;
    fn try_from(value: MessageWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MessageWebSocket> for IWebSocket {
    type Error = ::windows::core::Error;
    fn try_from(value: &MessageWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocket> for MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocket> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocket> for &MessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocket> {
        ::core::convert::TryInto::<IWebSocket>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MessageWebSocket {}
unsafe impl ::core::marker::Sync for MessageWebSocket {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct MessageWebSocketControl(::windows::core::IUnknown);
impl MessageWebSocketControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn MaxMessageSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetMaxMessageSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn MessageType(&self) -> ::windows::core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__: SocketMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketMessageType>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredUnsolicitedPongInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ReceiveMode(&self) -> ::windows::core::Result<MessageWebSocketReceiveMode> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__: MessageWebSocketReceiveMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MessageWebSocketReceiveMode>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetReceiveMode(&self, value: MessageWebSocketReceiveMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for MessageWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketControl {}
impl ::core::fmt::Debug for MessageWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MessageWebSocketControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocketControl;{8118388a-c629-4f0a-80fb-81fc05538862})");
}
unsafe impl ::windows::core::Interface for MessageWebSocketControl {
    type Vtable = IMessageWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8118388a_c629_4f0a_80fb_81fc05538862);
}
impl ::windows::core::RuntimeName for MessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketControl";
}
impl ::core::convert::From<MessageWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: MessageWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: &MessageWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MessageWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: MessageWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: &MessageWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MessageWebSocketControl> for IWebSocketControl {
    type Error = ::windows::core::Error;
    fn try_from(value: MessageWebSocketControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MessageWebSocketControl> for IWebSocketControl {
    type Error = ::windows::core::Error;
    fn try_from(value: &MessageWebSocketControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl> for MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl> for &MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl> {
        ::core::convert::TryInto::<IWebSocketControl>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<MessageWebSocketControl> for IWebSocketControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: MessageWebSocketControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MessageWebSocketControl> for IWebSocketControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &MessageWebSocketControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl2> for MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl2> for &MessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl2> {
        ::core::convert::TryInto::<IWebSocketControl2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MessageWebSocketControl {}
unsafe impl ::core::marker::Sync for MessageWebSocketControl {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct MessageWebSocketInformation(::windows::core::IUnknown);
impl MessageWebSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
}
impl ::core::clone::Clone for MessageWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketInformation {}
impl ::core::fmt::Debug for MessageWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MessageWebSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocketInformation;{5e01e316-c92a-47a5-b25f-07847639d181})");
}
unsafe impl ::windows::core::Interface for MessageWebSocketInformation {
    type Vtable = IWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e01e316_c92a_47a5_b25f_07847639d181);
}
impl ::windows::core::RuntimeName for MessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketInformation";
}
impl ::core::convert::From<MessageWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: MessageWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &MessageWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MessageWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: MessageWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &MessageWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MessageWebSocketInformation> for IWebSocketInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: MessageWebSocketInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MessageWebSocketInformation> for IWebSocketInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &MessageWebSocketInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation> for MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation> for &MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation> {
        ::core::convert::TryInto::<IWebSocketInformation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<MessageWebSocketInformation> for IWebSocketInformation2 {
    type Error = ::windows::core::Error;
    fn try_from(value: MessageWebSocketInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MessageWebSocketInformation> for IWebSocketInformation2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &MessageWebSocketInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation2> for MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation2> for &MessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation2> {
        ::core::convert::TryInto::<IWebSocketInformation2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MessageWebSocketInformation {}
unsafe impl ::core::marker::Sync for MessageWebSocketInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct MessageWebSocketMessageReceivedEventArgs(::windows::core::IUnknown);
impl MessageWebSocketMessageReceivedEventArgs {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn MessageType(&self) -> ::windows::core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__: SocketMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketMessageType>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::DataReader>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn IsMessageComplete(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMessageWebSocketMessageReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MessageWebSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageWebSocketMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketMessageReceivedEventArgs {}
impl ::core::fmt::Debug for MessageWebSocketMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketMessageReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MessageWebSocketMessageReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs;{478c22ac-4c4b-42ed-9ed7-1ef9f94fa3d5})");
}
unsafe impl ::windows::core::Interface for MessageWebSocketMessageReceivedEventArgs {
    type Vtable = IMessageWebSocketMessageReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x478c22ac_4c4b_42ed_9ed7_1ef9f94fa3d5);
}
impl ::windows::core::RuntimeName for MessageWebSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs";
}
impl ::core::convert::From<MessageWebSocketMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MessageWebSocketMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocketMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MessageWebSocketMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MessageWebSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MessageWebSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MessageWebSocketMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MessageWebSocketMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageWebSocketMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MessageWebSocketMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MessageWebSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MessageWebSocketMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MessageWebSocketMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MessageWebSocketMessageReceivedEventArgs {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct MessageWebSocketReceiveMode(pub i32);
impl MessageWebSocketReceiveMode {
    pub const FullMessage: Self = Self(0i32);
    pub const PartialMessage: Self = Self(1i32);
}
impl ::core::marker::Copy for MessageWebSocketReceiveMode {}
impl ::core::clone::Clone for MessageWebSocketReceiveMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MessageWebSocketReceiveMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MessageWebSocketReceiveMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketReceiveMode {}
impl ::core::fmt::Debug for MessageWebSocketReceiveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketReceiveMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MessageWebSocketReceiveMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.MessageWebSocketReceiveMode;i4)");
}
impl ::windows::core::DefaultType for MessageWebSocketReceiveMode {
    type DefaultType = Self;
}
#[repr(C)]
#[doc = "*Required features: 'Networking_Sockets'*"]
pub struct RoundTripTimeStatistics {
    pub Variance: u32,
    pub Max: u32,
    pub Min: u32,
    pub Sum: u32,
}
impl ::core::marker::Copy for RoundTripTimeStatistics {}
impl ::core::clone::Clone for RoundTripTimeStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RoundTripTimeStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RoundTripTimeStatistics").field("Variance", &self.Variance).field("Max", &self.Max).field("Min", &self.Min).field("Sum", &self.Sum).finish()
    }
}
unsafe impl ::windows::core::Abi for RoundTripTimeStatistics {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RoundTripTimeStatistics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.Sockets.RoundTripTimeStatistics;u4;u4;u4;u4)");
}
impl ::windows::core::DefaultType for RoundTripTimeStatistics {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for RoundTripTimeStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RoundTripTimeStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for RoundTripTimeStatistics {}
impl ::core::default::Default for RoundTripTimeStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ServerMessageWebSocket(::windows::core::IUnknown);
impl ServerMessageWebSocket {
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Control(&self) -> ::windows::core::Result<ServerMessageWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ServerMessageWebSocketControl>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<ServerMessageWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ServerMessageWebSocketInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CloseWithStatus<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, code: u16, reason: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), code, reason.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ServerMessageWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServerMessageWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerMessageWebSocket {}
impl ::core::fmt::Debug for ServerMessageWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerMessageWebSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServerMessageWebSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerMessageWebSocket;{e3ac9240-813b-5efd-7e11-ae2305fc77f1})");
}
unsafe impl ::windows::core::Interface for ServerMessageWebSocket {
    type Vtable = IServerMessageWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3ac9240_813b_5efd_7e11_ae2305fc77f1);
}
impl ::windows::core::RuntimeName for ServerMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocket";
}
impl ::core::convert::From<ServerMessageWebSocket> for ::windows::core::IUnknown {
    fn from(value: ServerMessageWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerMessageWebSocket> for ::windows::core::IUnknown {
    fn from(value: &ServerMessageWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ServerMessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ServerMessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServerMessageWebSocket> for ::windows::core::IInspectable {
    fn from(value: ServerMessageWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerMessageWebSocket> for ::windows::core::IInspectable {
    fn from(value: &ServerMessageWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ServerMessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ServerMessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ServerMessageWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ServerMessageWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ServerMessageWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ServerMessageWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ServerMessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ServerMessageWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ServerMessageWebSocket {}
unsafe impl ::core::marker::Sync for ServerMessageWebSocket {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ServerMessageWebSocketControl(::windows::core::IUnknown);
impl ServerMessageWebSocketControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn MessageType(&self) -> ::windows::core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__: SocketMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketMessageType>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for ServerMessageWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServerMessageWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerMessageWebSocketControl {}
impl ::core::fmt::Debug for ServerMessageWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerMessageWebSocketControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServerMessageWebSocketControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerMessageWebSocketControl;{69c2f051-1c1f-587a-4519-2181610192b7})");
}
unsafe impl ::windows::core::Interface for ServerMessageWebSocketControl {
    type Vtable = IServerMessageWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c2f051_1c1f_587a_4519_2181610192b7);
}
impl ::windows::core::RuntimeName for ServerMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocketControl";
}
impl ::core::convert::From<ServerMessageWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: ServerMessageWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerMessageWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: &ServerMessageWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ServerMessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ServerMessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServerMessageWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: ServerMessageWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerMessageWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: &ServerMessageWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ServerMessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ServerMessageWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ServerMessageWebSocketControl {}
unsafe impl ::core::marker::Sync for ServerMessageWebSocketControl {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ServerMessageWebSocketInformation(::windows::core::IUnknown);
impl ServerMessageWebSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
}
impl ::core::clone::Clone for ServerMessageWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServerMessageWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerMessageWebSocketInformation {}
impl ::core::fmt::Debug for ServerMessageWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerMessageWebSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServerMessageWebSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerMessageWebSocketInformation;{fc32b45f-4448-5505-6cc9-09afa8915f5d})");
}
unsafe impl ::windows::core::Interface for ServerMessageWebSocketInformation {
    type Vtable = IServerMessageWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc32b45f_4448_5505_6cc9_09afa8915f5d);
}
impl ::windows::core::RuntimeName for ServerMessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocketInformation";
}
impl ::core::convert::From<ServerMessageWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: ServerMessageWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerMessageWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &ServerMessageWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ServerMessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ServerMessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServerMessageWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: ServerMessageWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerMessageWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &ServerMessageWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ServerMessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ServerMessageWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ServerMessageWebSocketInformation {}
unsafe impl ::core::marker::Sync for ServerMessageWebSocketInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ServerStreamWebSocket(::windows::core::IUnknown);
impl ServerStreamWebSocket {
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<ServerStreamWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ServerStreamWebSocketInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CloseWithStatus<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, code: u16, reason: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), code, reason.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ServerStreamWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServerStreamWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerStreamWebSocket {}
impl ::core::fmt::Debug for ServerStreamWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerStreamWebSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServerStreamWebSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerStreamWebSocket;{2ced5bbf-74f6-55e4-79df-9132680dfee8})");
}
unsafe impl ::windows::core::Interface for ServerStreamWebSocket {
    type Vtable = IServerStreamWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ced5bbf_74f6_55e4_79df_9132680dfee8);
}
impl ::windows::core::RuntimeName for ServerStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerStreamWebSocket";
}
impl ::core::convert::From<ServerStreamWebSocket> for ::windows::core::IUnknown {
    fn from(value: ServerStreamWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerStreamWebSocket> for ::windows::core::IUnknown {
    fn from(value: &ServerStreamWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ServerStreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ServerStreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServerStreamWebSocket> for ::windows::core::IInspectable {
    fn from(value: ServerStreamWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerStreamWebSocket> for ::windows::core::IInspectable {
    fn from(value: &ServerStreamWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ServerStreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ServerStreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ServerStreamWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ServerStreamWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ServerStreamWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ServerStreamWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ServerStreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ServerStreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ServerStreamWebSocket {}
unsafe impl ::core::marker::Sync for ServerStreamWebSocket {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct ServerStreamWebSocketInformation(::windows::core::IUnknown);
impl ServerStreamWebSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
}
impl ::core::clone::Clone for ServerStreamWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServerStreamWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerStreamWebSocketInformation {}
impl ::core::fmt::Debug for ServerStreamWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerStreamWebSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServerStreamWebSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerStreamWebSocketInformation;{fc32b45f-4448-5505-6cc9-09aba8915f5d})");
}
unsafe impl ::windows::core::Interface for ServerStreamWebSocketInformation {
    type Vtable = IServerStreamWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc32b45f_4448_5505_6cc9_09aba8915f5d);
}
impl ::windows::core::RuntimeName for ServerStreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerStreamWebSocketInformation";
}
impl ::core::convert::From<ServerStreamWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: ServerStreamWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerStreamWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &ServerStreamWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ServerStreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ServerStreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServerStreamWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: ServerStreamWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServerStreamWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &ServerStreamWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ServerStreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ServerStreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ServerStreamWebSocketInformation {}
unsafe impl ::core::marker::Sync for ServerStreamWebSocketInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketActivityConnectedStandbyAction(pub i32);
impl SocketActivityConnectedStandbyAction {
    pub const DoNotWake: Self = Self(0i32);
    pub const Wake: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketActivityConnectedStandbyAction {}
impl ::core::clone::Clone for SocketActivityConnectedStandbyAction {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketActivityConnectedStandbyAction {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketActivityConnectedStandbyAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityConnectedStandbyAction {}
impl ::core::fmt::Debug for SocketActivityConnectedStandbyAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityConnectedStandbyAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketActivityConnectedStandbyAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityConnectedStandbyAction;i4)");
}
impl ::windows::core::DefaultType for SocketActivityConnectedStandbyAction {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketActivityContext(::windows::core::IUnknown);
impl SocketActivityContext {
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(data: Param0) -> ::windows::core::Result<SocketActivityContext> {
        Self::ISocketActivityContextFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<SocketActivityContext>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketActivityContextFactory<R, F: FnOnce(&ISocketActivityContextFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocketActivityContext, ISocketActivityContextFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SocketActivityContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SocketActivityContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityContext {}
impl ::core::fmt::Debug for SocketActivityContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketActivityContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.SocketActivityContext;{43b04d64-4c85-4396-a637-1d973f6ebd49})");
}
unsafe impl ::windows::core::Interface for SocketActivityContext {
    type Vtable = ISocketActivityContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43b04d64_4c85_4396_a637_1d973f6ebd49);
}
impl ::windows::core::RuntimeName for SocketActivityContext {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityContext";
}
impl ::core::convert::From<SocketActivityContext> for ::windows::core::IUnknown {
    fn from(value: SocketActivityContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityContext> for ::windows::core::IUnknown {
    fn from(value: &SocketActivityContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocketActivityContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocketActivityContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SocketActivityContext> for ::windows::core::IInspectable {
    fn from(value: SocketActivityContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityContext> for ::windows::core::IInspectable {
    fn from(value: &SocketActivityContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocketActivityContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocketActivityContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SocketActivityContext {}
unsafe impl ::core::marker::Sync for SocketActivityContext {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketActivityInformation(::windows::core::IUnknown);
impl SocketActivityInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SocketKind(&self) -> ::windows::core::Result<SocketActivityKind> {
        let this = self;
        unsafe {
            let mut result__: SocketActivityKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketActivityKind>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Context(&self) -> ::windows::core::Result<SocketActivityContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketActivityContext>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn DatagramSocket(&self) -> ::windows::core::Result<DatagramSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DatagramSocket>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn StreamSocket(&self) -> ::windows::core::Result<StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocket>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn StreamSocketListener(&self) -> ::windows::core::Result<StreamSocketListener> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocketListener>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllSockets() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>> {
        Self::ISocketActivityInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketActivityInformationStatics<R, F: FnOnce(&ISocketActivityInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocketActivityInformation, ISocketActivityInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SocketActivityInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SocketActivityInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityInformation {}
impl ::core::fmt::Debug for SocketActivityInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketActivityInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.SocketActivityInformation;{8d8a42e4-a87e-4b74-9968-185b2511defe})");
}
unsafe impl ::windows::core::Interface for SocketActivityInformation {
    type Vtable = ISocketActivityInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d8a42e4_a87e_4b74_9968_185b2511defe);
}
impl ::windows::core::RuntimeName for SocketActivityInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityInformation";
}
impl ::core::convert::From<SocketActivityInformation> for ::windows::core::IUnknown {
    fn from(value: SocketActivityInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityInformation> for ::windows::core::IUnknown {
    fn from(value: &SocketActivityInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocketActivityInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocketActivityInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SocketActivityInformation> for ::windows::core::IInspectable {
    fn from(value: SocketActivityInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityInformation> for ::windows::core::IInspectable {
    fn from(value: &SocketActivityInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocketActivityInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocketActivityInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SocketActivityInformation {}
unsafe impl ::core::marker::Sync for SocketActivityInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketActivityKind(pub i32);
impl SocketActivityKind {
    pub const None: Self = Self(0i32);
    pub const StreamSocketListener: Self = Self(1i32);
    pub const DatagramSocket: Self = Self(2i32);
    pub const StreamSocket: Self = Self(3i32);
}
impl ::core::marker::Copy for SocketActivityKind {}
impl ::core::clone::Clone for SocketActivityKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketActivityKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketActivityKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityKind {}
impl ::core::fmt::Debug for SocketActivityKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketActivityKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityKind;i4)");
}
impl ::windows::core::DefaultType for SocketActivityKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketActivityTriggerDetails(::windows::core::IUnknown);
impl SocketActivityTriggerDetails {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Reason(&self) -> ::windows::core::Result<SocketActivityTriggerReason> {
        let this = self;
        unsafe {
            let mut result__: SocketActivityTriggerReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketActivityTriggerReason>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SocketInformation(&self) -> ::windows::core::Result<SocketActivityInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketActivityInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for SocketActivityTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SocketActivityTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityTriggerDetails {}
impl ::core::fmt::Debug for SocketActivityTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketActivityTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.SocketActivityTriggerDetails;{45f406a7-fc9f-4f81-acad-355fef51e67b})");
}
unsafe impl ::windows::core::Interface for SocketActivityTriggerDetails {
    type Vtable = ISocketActivityTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45f406a7_fc9f_4f81_acad_355fef51e67b);
}
impl ::windows::core::RuntimeName for SocketActivityTriggerDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityTriggerDetails";
}
impl ::core::convert::From<SocketActivityTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: SocketActivityTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &SocketActivityTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocketActivityTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocketActivityTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SocketActivityTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: SocketActivityTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SocketActivityTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &SocketActivityTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocketActivityTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocketActivityTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SocketActivityTriggerDetails {}
unsafe impl ::core::marker::Sync for SocketActivityTriggerDetails {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketActivityTriggerReason(pub i32);
impl SocketActivityTriggerReason {
    pub const None: Self = Self(0i32);
    pub const SocketActivity: Self = Self(1i32);
    pub const ConnectionAccepted: Self = Self(2i32);
    pub const KeepAliveTimerExpired: Self = Self(3i32);
    pub const SocketClosed: Self = Self(4i32);
}
impl ::core::marker::Copy for SocketActivityTriggerReason {}
impl ::core::clone::Clone for SocketActivityTriggerReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketActivityTriggerReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketActivityTriggerReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityTriggerReason {}
impl ::core::fmt::Debug for SocketActivityTriggerReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityTriggerReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketActivityTriggerReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityTriggerReason;i4)");
}
impl ::windows::core::DefaultType for SocketActivityTriggerReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
pub struct SocketError {}
impl SocketError {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn GetStatus(hresult: i32) -> ::windows::core::Result<SocketErrorStatus> {
        Self::ISocketErrorStatics(|this| unsafe {
            let mut result__: SocketErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), hresult, &mut result__).from_abi::<SocketErrorStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketErrorStatics<R, F: FnOnce(&ISocketErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocketError, ISocketErrorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SocketError {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketError";
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketErrorStatus(pub i32);
impl SocketErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const OperationAborted: Self = Self(1i32);
    pub const HttpInvalidServerResponse: Self = Self(2i32);
    pub const ConnectionTimedOut: Self = Self(3i32);
    pub const AddressFamilyNotSupported: Self = Self(4i32);
    pub const SocketTypeNotSupported: Self = Self(5i32);
    pub const HostNotFound: Self = Self(6i32);
    pub const NoDataRecordOfRequestedType: Self = Self(7i32);
    pub const NonAuthoritativeHostNotFound: Self = Self(8i32);
    pub const ClassTypeNotFound: Self = Self(9i32);
    pub const AddressAlreadyInUse: Self = Self(10i32);
    pub const CannotAssignRequestedAddress: Self = Self(11i32);
    pub const ConnectionRefused: Self = Self(12i32);
    pub const NetworkIsUnreachable: Self = Self(13i32);
    pub const UnreachableHost: Self = Self(14i32);
    pub const NetworkIsDown: Self = Self(15i32);
    pub const NetworkDroppedConnectionOnReset: Self = Self(16i32);
    pub const SoftwareCausedConnectionAbort: Self = Self(17i32);
    pub const ConnectionResetByPeer: Self = Self(18i32);
    pub const HostIsDown: Self = Self(19i32);
    pub const NoAddressesFound: Self = Self(20i32);
    pub const TooManyOpenFiles: Self = Self(21i32);
    pub const MessageTooLong: Self = Self(22i32);
    pub const CertificateExpired: Self = Self(23i32);
    pub const CertificateUntrustedRoot: Self = Self(24i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(25i32);
    pub const CertificateWrongUsage: Self = Self(26i32);
    pub const CertificateRevoked: Self = Self(27i32);
    pub const CertificateNoRevocationCheck: Self = Self(28i32);
    pub const CertificateRevocationServerOffline: Self = Self(29i32);
    pub const CertificateIsInvalid: Self = Self(30i32);
}
impl ::core::marker::Copy for SocketErrorStatus {}
impl ::core::clone::Clone for SocketErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketErrorStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketErrorStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketErrorStatus {}
impl ::core::fmt::Debug for SocketErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketErrorStatus;i4)");
}
impl ::windows::core::DefaultType for SocketErrorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketMessageType(pub i32);
impl SocketMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Utf8: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketMessageType {}
impl ::core::clone::Clone for SocketMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketMessageType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketMessageType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketMessageType {}
impl ::core::fmt::Debug for SocketMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketMessageType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketMessageType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketMessageType;i4)");
}
impl ::windows::core::DefaultType for SocketMessageType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketProtectionLevel(pub i32);
impl SocketProtectionLevel {
    pub const PlainSocket: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const SslAllowNullEncryption: Self = Self(2i32);
    pub const BluetoothEncryptionAllowNullAuthentication: Self = Self(3i32);
    pub const BluetoothEncryptionWithAuthentication: Self = Self(4i32);
    pub const Ssl3AllowWeakEncryption: Self = Self(5i32);
    pub const Tls10: Self = Self(6i32);
    pub const Tls11: Self = Self(7i32);
    pub const Tls12: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for SocketProtectionLevel {}
impl ::core::clone::Clone for SocketProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketProtectionLevel {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketProtectionLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketProtectionLevel {}
impl ::core::fmt::Debug for SocketProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketProtectionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketProtectionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketProtectionLevel;i4)");
}
impl ::windows::core::DefaultType for SocketProtectionLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketQualityOfService(pub i32);
impl SocketQualityOfService {
    pub const Normal: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketQualityOfService {}
impl ::core::clone::Clone for SocketQualityOfService {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketQualityOfService {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketQualityOfService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketQualityOfService {}
impl ::core::fmt::Debug for SocketQualityOfService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketQualityOfService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketQualityOfService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketQualityOfService;i4)");
}
impl ::windows::core::DefaultType for SocketQualityOfService {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct SocketSslErrorSeverity(pub i32);
impl SocketSslErrorSeverity {
    pub const None: Self = Self(0i32);
    pub const Ignorable: Self = Self(1i32);
    pub const Fatal: Self = Self(2i32);
}
impl ::core::marker::Copy for SocketSslErrorSeverity {}
impl ::core::clone::Clone for SocketSslErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SocketSslErrorSeverity {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SocketSslErrorSeverity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketSslErrorSeverity {}
impl ::core::fmt::Debug for SocketSslErrorSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketSslErrorSeverity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SocketSslErrorSeverity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketSslErrorSeverity;i4)");
}
impl ::windows::core::DefaultType for SocketSslErrorSeverity {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocket(::windows::core::IUnknown);
impl StreamSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StreamSocket, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Control(&self) -> ::windows::core::Result<StreamSocketControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocketControl>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<StreamSocketInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocketInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithEndpointPairAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EndpointPair>>(&self, endpointpair: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), endpointpair.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, remotehostname: Param0, remoteservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithEndpointPairAndProtectionLevelAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EndpointPair>>(&self, endpointpair: Param0, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), endpointpair.into_param().abi(), protectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithProtectionLevelAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, remotehostname: Param0, remoteservicename: Param1, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), protectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpgradeToSslAsync<'a, Param1: ::windows::core::IntoParam<'a, super::HostName>>(&self, protectionlevel: SocketProtectionLevel, validationhostname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), protectionlevel, validationhostname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Networking_Connectivity'*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub fn ConnectWithProtectionLevelAndAdapterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::Connectivity::NetworkAdapter>>(&self, remotehostname: Param0, remoteservicename: Param1, protectionlevel: SocketProtectionLevel, adapter: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStreamSocket2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), protectionlevel, adapter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn EnableTransferOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskid: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), taskid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn EnableTransferOwnershipWithConnectedStandbyAction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskid: Param0, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), taskid.into_param().abi(), connectedstandbyaction).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransferOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, socketid: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), socketid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransferOwnershipWithContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SocketActivityContext>>(&self, socketid: Param0, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), socketid.into_param().abi(), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TransferOwnershipWithContextAndKeepAliveTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SocketActivityContext>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, socketid: Param0, data: Param1, keepalivetime: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), socketid.into_param().abi(), data.into_param().abi(), keepalivetime.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetEndpointPairsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(remotehostname: Param0, remoteservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IStreamSocketStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetEndpointPairsWithSortOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(remotehostname: Param0, remoteservicename: Param1, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IStreamSocketStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), sortoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStreamSocketStatics<R, F: FnOnce(&IStreamSocketStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StreamSocket, IStreamSocketStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StreamSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocket {}
impl ::core::fmt::Debug for StreamSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocket;{69a22cf3-fc7b-4857-af38-f6e7de6a5b49})");
}
unsafe impl ::windows::core::Interface for StreamSocket {
    type Vtable = IStreamSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a22cf3_fc7b_4857_af38_f6e7de6a5b49);
}
impl ::windows::core::RuntimeName for StreamSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocket";
}
impl ::core::convert::From<StreamSocket> for ::windows::core::IUnknown {
    fn from(value: StreamSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocket> for ::windows::core::IUnknown {
    fn from(value: &StreamSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocket> for ::windows::core::IInspectable {
    fn from(value: StreamSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocket> for ::windows::core::IInspectable {
    fn from(value: &StreamSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<StreamSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StreamSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for StreamSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &StreamSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StreamSocket {}
unsafe impl ::core::marker::Sync for StreamSocket {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocketControl(::windows::core::IUnknown);
impl StreamSocketControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn NoDelay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn KeepAlive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__: SocketQualityOfService = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketQualityOfService>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SerializeConnectionAttempts(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetSerializeConnectionAttempts(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn MinProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl4>(self)?;
        unsafe {
            let mut result__: SocketProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetMinProtectionLevel(&self, value: SocketProtectionLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketControl4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for StreamSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketControl {}
impl ::core::fmt::Debug for StreamSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocketControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketControl;{fe25adf1-92ab-4af3-9992-0f4c85e36cc4})");
}
unsafe impl ::windows::core::Interface for StreamSocketControl {
    type Vtable = IStreamSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe25adf1_92ab_4af3_9992_0f4c85e36cc4);
}
impl ::windows::core::RuntimeName for StreamSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketControl";
}
impl ::core::convert::From<StreamSocketControl> for ::windows::core::IUnknown {
    fn from(value: StreamSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketControl> for ::windows::core::IUnknown {
    fn from(value: &StreamSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocketControl> for ::windows::core::IInspectable {
    fn from(value: StreamSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketControl> for ::windows::core::IInspectable {
    fn from(value: &StreamSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StreamSocketControl {}
unsafe impl ::core::marker::Sync for StreamSocketControl {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocketInformation(::windows::core::IUnknown);
impl StreamSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn RoundTripTimeStatistics(&self) -> ::windows::core::Result<RoundTripTimeStatistics> {
        let this = self;
        unsafe {
            let mut result__: RoundTripTimeStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RoundTripTimeStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: SocketProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionKey(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = &::windows::core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__: SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &::windows::core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
}
impl ::core::clone::Clone for StreamSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketInformation {}
impl ::core::fmt::Debug for StreamSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketInformation;{3b80ae30-5e68-4205-88f0-dc85d2e25ded})");
}
unsafe impl ::windows::core::Interface for StreamSocketInformation {
    type Vtable = IStreamSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b80ae30_5e68_4205_88f0_dc85d2e25ded);
}
impl ::windows::core::RuntimeName for StreamSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketInformation";
}
impl ::core::convert::From<StreamSocketInformation> for ::windows::core::IUnknown {
    fn from(value: StreamSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &StreamSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocketInformation> for ::windows::core::IInspectable {
    fn from(value: StreamSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &StreamSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StreamSocketInformation {}
unsafe impl ::core::marker::Sync for StreamSocketInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocketListener(::windows::core::IUnknown);
impl StreamSocketListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StreamSocketListener, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Control(&self) -> ::windows::core::Result<StreamSocketListenerControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocketListenerControl>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<StreamSocketListenerInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocketListenerInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BindServiceNameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localservicename: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), localservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BindEndpointAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localhostname: Param0, localservicename: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), localhostname.into_param().abi(), localservicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BindServiceNameWithProtectionLevelAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localservicename: Param0, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), localservicename.into_param().abi(), protectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation', 'Networking_Connectivity'*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub fn BindServiceNameWithProtectionLevelAndAdapterAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::Connectivity::NetworkAdapter>>(&self, localservicename: Param0, protectionlevel: SocketProtectionLevel, adapter: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), localservicename.into_param().abi(), protectionlevel, adapter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn EnableTransferOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskid: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), taskid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn EnableTransferOwnershipWithConnectedStandbyAction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, taskid: Param0, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), taskid.into_param().abi(), connectedstandbyaction).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransferOwnership<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, socketid: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), socketid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn TransferOwnershipWithContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SocketActivityContext>>(&self, socketid: Param0, data: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), socketid.into_param().abi(), data.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StreamSocketListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocketListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListener {}
impl ::core::fmt::Debug for StreamSocketListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocketListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListener;{ff513437-df9f-4df0-bf82-0ec5d7b35aae})");
}
unsafe impl ::windows::core::Interface for StreamSocketListener {
    type Vtable = IStreamSocketListenerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff513437_df9f_4df0_bf82_0ec5d7b35aae);
}
impl ::windows::core::RuntimeName for StreamSocketListener {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListener";
}
impl ::core::convert::From<StreamSocketListener> for ::windows::core::IUnknown {
    fn from(value: StreamSocketListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListener> for ::windows::core::IUnknown {
    fn from(value: &StreamSocketListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocketListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocketListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocketListener> for ::windows::core::IInspectable {
    fn from(value: StreamSocketListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListener> for ::windows::core::IInspectable {
    fn from(value: &StreamSocketListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocketListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocketListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<StreamSocketListener> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamSocketListener) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StreamSocketListener> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamSocketListener) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for StreamSocketListener {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &StreamSocketListener {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StreamSocketListener {}
unsafe impl ::core::marker::Sync for StreamSocketListener {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocketListenerConnectionReceivedEventArgs(::windows::core::IUnknown);
impl StreamSocketListenerConnectionReceivedEventArgs {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Socket(&self) -> ::windows::core::Result<StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamSocket>(result__)
        }
    }
}
impl ::core::clone::Clone for StreamSocketListenerConnectionReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocketListenerConnectionReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListenerConnectionReceivedEventArgs {}
impl ::core::fmt::Debug for StreamSocketListenerConnectionReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListenerConnectionReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocketListenerConnectionReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs;{0c472ea9-373f-447b-85b1-ddd4548803ba})");
}
unsafe impl ::windows::core::Interface for StreamSocketListenerConnectionReceivedEventArgs {
    type Vtable = IStreamSocketListenerConnectionReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c472ea9_373f_447b_85b1_ddd4548803ba);
}
impl ::windows::core::RuntimeName for StreamSocketListenerConnectionReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs";
}
impl ::core::convert::From<StreamSocketListenerConnectionReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: StreamSocketListenerConnectionReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListenerConnectionReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &StreamSocketListenerConnectionReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocketListenerConnectionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocketListenerConnectionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocketListenerConnectionReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: StreamSocketListenerConnectionReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListenerConnectionReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &StreamSocketListenerConnectionReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocketListenerConnectionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocketListenerConnectionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StreamSocketListenerConnectionReceivedEventArgs {}
unsafe impl ::core::marker::Sync for StreamSocketListenerConnectionReceivedEventArgs {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocketListenerControl(::windows::core::IUnknown);
impl StreamSocketListenerControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__: SocketQualityOfService = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketQualityOfService>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn NoDelay(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn KeepAlive(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for StreamSocketListenerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocketListenerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListenerControl {}
impl ::core::fmt::Debug for StreamSocketListenerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListenerControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocketListenerControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListenerControl;{20d8c576-8d8a-4dba-9722-a16c4d984980})");
}
unsafe impl ::windows::core::Interface for StreamSocketListenerControl {
    type Vtable = IStreamSocketListenerControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20d8c576_8d8a_4dba_9722_a16c4d984980);
}
impl ::windows::core::RuntimeName for StreamSocketListenerControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerControl";
}
impl ::core::convert::From<StreamSocketListenerControl> for ::windows::core::IUnknown {
    fn from(value: StreamSocketListenerControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListenerControl> for ::windows::core::IUnknown {
    fn from(value: &StreamSocketListenerControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocketListenerControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocketListenerControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocketListenerControl> for ::windows::core::IInspectable {
    fn from(value: StreamSocketListenerControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListenerControl> for ::windows::core::IInspectable {
    fn from(value: &StreamSocketListenerControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocketListenerControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocketListenerControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StreamSocketListenerControl {}
unsafe impl ::core::marker::Sync for StreamSocketListenerControl {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamSocketListenerInformation(::windows::core::IUnknown);
impl StreamSocketListenerInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StreamSocketListenerInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamSocketListenerInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListenerInformation {}
impl ::core::fmt::Debug for StreamSocketListenerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListenerInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamSocketListenerInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListenerInformation;{e62ba82f-a63a-430b-bf62-29e93e5633b4})");
}
unsafe impl ::windows::core::Interface for StreamSocketListenerInformation {
    type Vtable = IStreamSocketListenerInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62ba82f_a63a_430b_bf62_29e93e5633b4);
}
impl ::windows::core::RuntimeName for StreamSocketListenerInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerInformation";
}
impl ::core::convert::From<StreamSocketListenerInformation> for ::windows::core::IUnknown {
    fn from(value: StreamSocketListenerInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListenerInformation> for ::windows::core::IUnknown {
    fn from(value: &StreamSocketListenerInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamSocketListenerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamSocketListenerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamSocketListenerInformation> for ::windows::core::IInspectable {
    fn from(value: StreamSocketListenerInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamSocketListenerInformation> for ::windows::core::IInspectable {
    fn from(value: &StreamSocketListenerInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamSocketListenerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamSocketListenerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StreamSocketListenerInformation {}
unsafe impl ::core::marker::Sync for StreamSocketListenerInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamWebSocket(::windows::core::IUnknown);
impl StreamWebSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StreamWebSocket, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Control(&self) -> ::windows::core::Result<StreamWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamWebSocketControl>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Information(&self) -> ::windows::core::Result<StreamWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamWebSocketInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocket2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocket2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn CloseWithStatus<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, code: u16, reason: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), code, reason.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StreamWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamWebSocket {}
impl ::core::fmt::Debug for StreamWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamWebSocket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamWebSocket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamWebSocket;{bd4a49d8-b289-45bb-97eb-c7525205a843})");
}
unsafe impl ::windows::core::Interface for StreamWebSocket {
    type Vtable = IStreamWebSocketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4a49d8_b289_45bb_97eb_c7525205a843);
}
impl ::windows::core::RuntimeName for StreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocket";
}
impl ::core::convert::From<StreamWebSocket> for ::windows::core::IUnknown {
    fn from(value: StreamWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamWebSocket> for ::windows::core::IUnknown {
    fn from(value: &StreamWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamWebSocket> for ::windows::core::IInspectable {
    fn from(value: StreamWebSocket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamWebSocket> for ::windows::core::IInspectable {
    fn from(value: &StreamWebSocket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<StreamWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StreamWebSocket> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StreamWebSocket> for IWebSocket {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamWebSocket) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StreamWebSocket> for IWebSocket {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamWebSocket) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocket> for StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocket> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocket> for &StreamWebSocket {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocket> {
        ::core::convert::TryInto::<IWebSocket>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StreamWebSocket {}
unsafe impl ::core::marker::Sync for StreamWebSocket {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamWebSocketControl(::windows::core::IUnknown);
impl StreamWebSocketControl {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn NoDelay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredUnsolicitedPongInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::Interface::cast::<IWebSocketControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for StreamWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamWebSocketControl {}
impl ::core::fmt::Debug for StreamWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamWebSocketControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamWebSocketControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamWebSocketControl;{b4f478b1-a45a-48db-953a-645b7d964c07})");
}
unsafe impl ::windows::core::Interface for StreamWebSocketControl {
    type Vtable = IStreamWebSocketControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4f478b1_a45a_48db_953a_645b7d964c07);
}
impl ::windows::core::RuntimeName for StreamWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocketControl";
}
impl ::core::convert::From<StreamWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: StreamWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamWebSocketControl> for ::windows::core::IUnknown {
    fn from(value: &StreamWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: StreamWebSocketControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamWebSocketControl> for ::windows::core::IInspectable {
    fn from(value: &StreamWebSocketControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StreamWebSocketControl> for IWebSocketControl {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamWebSocketControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StreamWebSocketControl> for IWebSocketControl {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamWebSocketControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl> for StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl> for &StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl> {
        ::core::convert::TryInto::<IWebSocketControl>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StreamWebSocketControl> for IWebSocketControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamWebSocketControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StreamWebSocketControl> for IWebSocketControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamWebSocketControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl2> for StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketControl2> for &StreamWebSocketControl {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketControl2> {
        ::core::convert::TryInto::<IWebSocketControl2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StreamWebSocketControl {}
unsafe impl ::core::marker::Sync for StreamWebSocketControl {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct StreamWebSocketInformation(::windows::core::IUnknown);
impl StreamWebSocketInformation {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__: BandwidthStatistics = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BandwidthStatistics>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &::windows::core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
}
impl ::core::clone::Clone for StreamWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreamWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamWebSocketInformation {}
impl ::core::fmt::Debug for StreamWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamWebSocketInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamWebSocketInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamWebSocketInformation;{5e01e316-c92a-47a5-b25f-07847639d181})");
}
unsafe impl ::windows::core::Interface for StreamWebSocketInformation {
    type Vtable = IWebSocketInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e01e316_c92a_47a5_b25f_07847639d181);
}
impl ::windows::core::RuntimeName for StreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocketInformation";
}
impl ::core::convert::From<StreamWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: StreamWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamWebSocketInformation> for ::windows::core::IUnknown {
    fn from(value: &StreamWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreamWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: StreamWebSocketInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreamWebSocketInformation> for ::windows::core::IInspectable {
    fn from(value: &StreamWebSocketInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StreamWebSocketInformation> for IWebSocketInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamWebSocketInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StreamWebSocketInformation> for IWebSocketInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamWebSocketInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation> for StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation> for &StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation> {
        ::core::convert::TryInto::<IWebSocketInformation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StreamWebSocketInformation> for IWebSocketInformation2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamWebSocketInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StreamWebSocketInformation> for IWebSocketInformation2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamWebSocketInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation2> for StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebSocketInformation2> for &StreamWebSocketInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebSocketInformation2> {
        ::core::convert::TryInto::<IWebSocketInformation2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StreamWebSocketInformation {}
unsafe impl ::core::marker::Sync for StreamWebSocketInformation {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct WebSocketClosedEventArgs(::windows::core::IUnknown);
impl WebSocketClosedEventArgs {
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Code(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Reason(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WebSocketClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebSocketClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebSocketClosedEventArgs {}
impl ::core::fmt::Debug for WebSocketClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSocketClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebSocketClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.WebSocketClosedEventArgs;{ceb78d07-d0a8-4703-a091-c8c2c0915bc3})");
}
unsafe impl ::windows::core::Interface for WebSocketClosedEventArgs {
    type Vtable = IWebSocketClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb78d07_d0a8_4703_a091_c8c2c0915bc3);
}
impl ::windows::core::RuntimeName for WebSocketClosedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketClosedEventArgs";
}
impl ::core::convert::From<WebSocketClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebSocketClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebSocketClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebSocketClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebSocketClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebSocketClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebSocketClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebSocketClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebSocketClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebSocketClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebSocketClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebSocketClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebSocketClosedEventArgs {}
unsafe impl ::core::marker::Sync for WebSocketClosedEventArgs {}
#[doc = "*Required features: 'Networking_Sockets'*"]
pub struct WebSocketError {}
impl WebSocketError {
    #[doc = "*Required features: 'Networking_Sockets', 'Web'*"]
    #[cfg(feature = "Web")]
    pub fn GetStatus(hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus> {
        Self::IWebSocketErrorStatics(|this| unsafe {
            let mut result__: super::super::Web::WebErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), hresult, &mut result__).from_abi::<super::super::Web::WebErrorStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebSocketErrorStatics<R, F: FnOnce(&IWebSocketErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebSocketError, IWebSocketErrorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WebSocketError {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketError";
}
#[doc = "*Required features: 'Networking_Sockets', 'ApplicationModel_Background'*"]
#[cfg(feature = "ApplicationModel_Background")]
#[repr(transparent)]
pub struct WebSocketKeepAlive(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Background")]
impl WebSocketKeepAlive {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebSocketKeepAlive, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'ApplicationModel_Background'*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Run<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Background::IBackgroundTaskInstance>>(&self, taskinstance: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), taskinstance.into_param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::clone::Clone for WebSocketKeepAlive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::cmp::PartialEq for WebSocketKeepAlive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::cmp::Eq for WebSocketKeepAlive {}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::fmt::Debug for WebSocketKeepAlive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSocketKeepAlive").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::windows::core::RuntimeType for WebSocketKeepAlive {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.WebSocketKeepAlive;{7d13d534-fd12-43ce-8c22-ea1ff13c06df})");
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::windows::core::Interface for WebSocketKeepAlive {
    type Vtable = super::super::ApplicationModel::Background::IBackgroundTaskVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d13d534_fd12_43ce_8c22_ea1ff13c06df);
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::RuntimeName for WebSocketKeepAlive {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketKeepAlive";
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::From<WebSocketKeepAlive> for ::windows::core::IUnknown {
    fn from(value: WebSocketKeepAlive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::From<&WebSocketKeepAlive> for ::windows::core::IUnknown {
    fn from(value: &WebSocketKeepAlive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebSocketKeepAlive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebSocketKeepAlive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::From<WebSocketKeepAlive> for ::windows::core::IInspectable {
    fn from(value: WebSocketKeepAlive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::From<&WebSocketKeepAlive> for ::windows::core::IInspectable {
    fn from(value: &WebSocketKeepAlive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebSocketKeepAlive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebSocketKeepAlive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::TryFrom<WebSocketKeepAlive> for super::super::ApplicationModel::Background::IBackgroundTask {
    type Error = ::windows::core::Error;
    fn try_from(value: WebSocketKeepAlive) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::TryFrom<&WebSocketKeepAlive> for super::super::ApplicationModel::Background::IBackgroundTask {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebSocketKeepAlive) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Background::IBackgroundTask> for WebSocketKeepAlive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Background::IBackgroundTask> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Background::IBackgroundTask> for &WebSocketKeepAlive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Background::IBackgroundTask> {
        ::core::convert::TryInto::<super::super::ApplicationModel::Background::IBackgroundTask>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::core::marker::Send for WebSocketKeepAlive {}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::core::marker::Sync for WebSocketKeepAlive {}
#[doc = "*Required features: 'Networking_Sockets'*"]
#[repr(transparent)]
pub struct WebSocketServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
impl WebSocketServerCustomValidationRequestedEventArgs {
    #[doc = "*Required features: 'Networking_Sockets', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_Sockets'*"]
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_Sockets', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for WebSocketServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebSocketServerCustomValidationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebSocketServerCustomValidationRequestedEventArgs {}
impl ::core::fmt::Debug for WebSocketServerCustomValidationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSocketServerCustomValidationRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebSocketServerCustomValidationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs;{ffeffe48-022a-4ab7-8b36-e10af4640e6b})");
}
unsafe impl ::windows::core::Interface for WebSocketServerCustomValidationRequestedEventArgs {
    type Vtable = IWebSocketServerCustomValidationRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffeffe48_022a_4ab7_8b36_e10af4640e6b);
}
impl ::windows::core::RuntimeName for WebSocketServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs";
}
impl ::core::convert::From<WebSocketServerCustomValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebSocketServerCustomValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebSocketServerCustomValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebSocketServerCustomValidationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebSocketServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebSocketServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebSocketServerCustomValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebSocketServerCustomValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebSocketServerCustomValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebSocketServerCustomValidationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebSocketServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebSocketServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebSocketServerCustomValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WebSocketServerCustomValidationRequestedEventArgs {}
