#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AttributedNetworkUsage(::windows::runtime::IInspectable);
impl AttributedNetworkUsage {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesSent(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesReceived(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn AttributionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn AttributionName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Connectivity`, `Storage_Streams`*"]
    pub fn AttributionThumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AttributedNetworkUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.AttributedNetworkUsage;{f769b039-eca2-45eb-ade1-b0368b756c49})");
}
unsafe impl ::windows::runtime::Interface for AttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4150898745, 60578, 17899, [173, 225, 176, 54, 139, 117, 108, 73]);
}
impl ::windows::runtime::RuntimeName for AttributedNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.AttributedNetworkUsage";
}
unsafe impl ::std::marker::Send for AttributedNetworkUsage {}
unsafe impl ::std::marker::Sync for AttributedNetworkUsage {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: CellularApnAuthenticationType = CellularApnAuthenticationType(0i32);
    pub const Pap: CellularApnAuthenticationType = CellularApnAuthenticationType(1i32);
    pub const Chap: CellularApnAuthenticationType = CellularApnAuthenticationType(2i32);
    pub const Mschapv2: CellularApnAuthenticationType = CellularApnAuthenticationType(3i32);
}
impl ::std::convert::From<i32> for CellularApnAuthenticationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CellularApnAuthenticationType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CellularApnAuthenticationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.CellularApnAuthenticationType;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CellularApnContext(::windows::runtime::IInspectable);
impl CellularApnContext {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CellularApnContext, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetProviderId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn AccessPointName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetAccessPointName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetUserName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn Password(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetPassword<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IsCompressionEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetIsCompressionEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn AuthenticationType(&self) -> ::windows::runtime::Result<CellularApnAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__: CellularApnAuthenticationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularApnAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetAuthenticationType(&self, value: CellularApnAuthenticationType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ProfileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ICellularApnContext2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetProfileName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICellularApnContext2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CellularApnContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.CellularApnContext;{6fa529f4-effd-4542-9ab2-705bbf94943a})");
}
unsafe impl ::windows::runtime::Interface for CellularApnContext {
    type Vtable = ICellularApnContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1873095156, 61437, 17730, [154, 178, 112, 91, 191, 148, 148, 58]);
}
impl ::windows::runtime::RuntimeName for CellularApnContext {
    const NAME: &'static str = "Windows.Networking.Connectivity.CellularApnContext";
}
unsafe impl ::std::marker::Send for CellularApnContext {}
unsafe impl ::std::marker::Sync for CellularApnContext {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ConnectionCost(::windows::runtime::IInspectable);
impl ConnectionCost {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkCostType(&self) -> ::windows::runtime::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__: NetworkCostType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkCostType>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn Roaming(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn OverDataLimit(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ApproachingDataLimit(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BackgroundDataUsageRestricted(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IConnectionCost2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConnectionCost {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionCost;{bad7d829-3416-4b10-a202-bac0b075bdae})");
}
unsafe impl ::windows::runtime::Interface for ConnectionCost {
    type Vtable = IConnectionCost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134707753, 13334, 19216, [162, 2, 186, 192, 176, 117, 189, 174]);
}
impl ::windows::runtime::RuntimeName for ConnectionCost {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionCost";
}
unsafe impl ::std::marker::Send for ConnectionCost {}
unsafe impl ::std::marker::Sync for ConnectionCost {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ConnectionProfile(::windows::runtime::IInspectable);
impl ConnectionProfile {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ProfileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetNetworkConnectivityLevel(&self) -> ::windows::runtime::Result<NetworkConnectivityLevel> {
        let this = self;
        unsafe {
            let mut result__: NetworkConnectivityLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkConnectivityLevel>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn GetNetworkNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetConnectionCost(&self) -> ::windows::runtime::Result<ConnectionCost> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConnectionCost>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetDataPlanStatus(&self) -> ::windows::runtime::Result<DataPlanStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPlanStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkAdapter(&self) -> ::windows::runtime::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkAdapter>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn GetLocalUsage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, starttime: Param0, endtime: Param1) -> ::windows::runtime::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), starttime.into_param().abi(), endtime.into_param().abi(), &mut result__).from_abi::<DataUsage>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn GetLocalUsagePerRoamingStates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, starttime: Param0, endtime: Param1, states: RoamingStates) -> ::windows::runtime::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), starttime.into_param().abi(), endtime.into_param().abi(), states, &mut result__).from_abi::<DataUsage>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkSecuritySettings(&self) -> ::windows::runtime::Result<NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkSecuritySettings>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IsWwanConnectionProfile(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IsWlanConnectionProfile(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn WwanConnectionProfileDetails(&self) -> ::windows::runtime::Result<WwanConnectionProfileDetails> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WwanConnectionProfileDetails>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn WlanConnectionProfileDetails(&self) -> ::windows::runtime::Result<WlanConnectionProfileDetails> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WlanConnectionProfileDetails>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn ServiceProviderGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<::windows::runtime::GUID>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<::windows::runtime::GUID>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn GetSignalBars(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u8>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetDomainConnectivityLevel(&self) -> ::windows::runtime::Result<DomainConnectivityLevel> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: DomainConnectivityLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DomainConnectivityLevel>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetNetworkUsageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param3: ::windows::runtime::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, granularity: DataUsageGranularity, states: Param3) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), starttime.into_param().abi(), endtime.into_param().abi(), granularity, states.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetConnectivityIntervalsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::runtime::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, states: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), starttime.into_param().abi(), endtime.into_param().abi(), states.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetAttributedNetworkUsageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::runtime::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, states: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), starttime.into_param().abi(), endtime.into_param().abi(), states.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetProviderNetworkUsageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows::runtime::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, states: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), starttime.into_param().abi(), endtime.into_param().abi(), states.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn CanDelete(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn TryDeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConnectionProfile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionProfile;{71ba143c-598e-49d0-84eb-8febaedcc195})");
}
unsafe impl ::windows::runtime::Interface for ConnectionProfile {
    type Vtable = IConnectionProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908020284, 22926, 18896, [132, 235, 143, 235, 174, 220, 193, 149]);
}
impl ::windows::runtime::RuntimeName for ConnectionProfile {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfile";
}
unsafe impl ::std::marker::Send for ConnectionProfile {}
unsafe impl ::std::marker::Sync for ConnectionProfile {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConnectionProfileDeleteStatus(pub i32);
impl ConnectionProfileDeleteStatus {
    pub const Success: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(0i32);
    pub const DeniedByUser: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(1i32);
    pub const DeniedBySystem: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(2i32);
    pub const UnknownError: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(3i32);
}
impl ::std::convert::From<i32> for ConnectionProfileDeleteStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConnectionProfileDeleteStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConnectionProfileDeleteStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.ConnectionProfileDeleteStatus;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ConnectionProfileFilter(::windows::runtime::IInspectable);
impl ConnectionProfileFilter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ConnectionProfileFilter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetIsConnected(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IsConnected(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetIsWwanConnectionProfile(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IsWwanConnectionProfile(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetIsWlanConnectionProfile(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IsWlanConnectionProfile(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn SetNetworkCostType(&self, value: NetworkCostType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkCostType(&self) -> ::windows::runtime::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__: NetworkCostType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkCostType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn SetServiceProviderGuid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<::windows::runtime::GUID>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn ServiceProviderGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<::windows::runtime::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<::windows::runtime::GUID>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn SetIsRoaming<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn IsRoaming(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn SetIsOverDataLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn IsOverDataLimit(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn SetIsBackgroundDataUsageRestricted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn IsBackgroundDataUsageRestricted(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Connectivity`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn SetPurposeGuid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<::windows::runtime::GUID>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn PurposeGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<::windows::runtime::GUID>> {
        let this = &::windows::runtime::Interface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<::windows::runtime::GUID>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConnectionProfileFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionProfileFilter;{204c7cc8-bd2d-4e8d-a4b3-455ec337388a})");
}
unsafe impl ::windows::runtime::Interface for ConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541883592, 48429, 20109, [164, 179, 69, 94, 195, 55, 56, 138]);
}
impl ::windows::runtime::RuntimeName for ConnectionProfileFilter {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfileFilter";
}
unsafe impl ::std::marker::Send for ConnectionProfileFilter {}
unsafe impl ::std::marker::Sync for ConnectionProfileFilter {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ConnectionSession(::windows::runtime::IInspectable);
impl ConnectionSession {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ConnectionProfile(&self) -> ::windows::runtime::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConnectionProfile>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConnectionSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionSession;{ff905d4c-f83b-41b0-8a0c-1462d9c56b73})");
}
unsafe impl ::windows::runtime::Interface for ConnectionSession {
    type Vtable = IConnectionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4287651148, 63547, 16816, [138, 12, 20, 98, 217, 197, 107, 115]);
}
impl ::windows::runtime::RuntimeName for ConnectionSession {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionSession";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ConnectionSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ConnectionSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ConnectionSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ConnectionSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ConnectionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ConnectionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ConnectionSession {}
unsafe impl ::std::marker::Sync for ConnectionSession {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ConnectivityInterval(::windows::runtime::IInspectable);
impl ConnectivityInterval {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn ConnectionDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConnectivityInterval {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectivityInterval;{4faa3fff-6746-4824-a964-eed8e87f8709})");
}
unsafe impl ::windows::runtime::Interface for ConnectivityInterval {
    type Vtable = IConnectivityInterval_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1336557567, 26438, 18468, [169, 100, 238, 216, 232, 127, 135, 9]);
}
impl ::windows::runtime::RuntimeName for ConnectivityInterval {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityInterval";
}
unsafe impl ::std::marker::Send for ConnectivityInterval {}
unsafe impl ::std::marker::Sync for ConnectivityInterval {}
#[doc = "*Required features: `Networking_Connectivity`*"]
pub struct ConnectivityManager {}
impl ConnectivityManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn AcquireConnectionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CellularApnContext>>(cellularapncontext: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>> {
        Self::IConnectivityManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), cellularapncontext.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConnectionSession>>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn AddHttpRoutePolicy<'a, Param0: ::windows::runtime::IntoParam<'a, RoutePolicy>>(routepolicy: Param0) -> ::windows::runtime::Result<()> {
        Self::IConnectivityManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), routepolicy.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn RemoveHttpRoutePolicy<'a, Param0: ::windows::runtime::IntoParam<'a, RoutePolicy>>(routepolicy: Param0) -> ::windows::runtime::Result<()> {
        Self::IConnectivityManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), routepolicy.into_param().abi()).ok() })
    }
    pub fn IConnectivityManagerStatics<R, F: FnOnce(&IConnectivityManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ConnectivityManager, IConnectivityManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ConnectivityManager {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityManager";
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DataPlanStatus(::windows::runtime::IInspectable);
impl DataPlanStatus {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn DataPlanUsage(&self) -> ::windows::runtime::Result<DataPlanUsage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataPlanUsage>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn DataLimitInMegabytes(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn InboundBitsPerSecond(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn OutboundBitsPerSecond(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn NextBillingCycle(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn MaxTransferSizeInMegabytes(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataPlanStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataPlanStatus;{977a8b8c-3885-40f3-8851-42cd2bd568bb})");
}
unsafe impl ::windows::runtime::Interface for DataPlanStatus {
    type Vtable = IDataPlanStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541390732, 14469, 16627, [136, 81, 66, 205, 43, 213, 104, 187]);
}
impl ::windows::runtime::RuntimeName for DataPlanStatus {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanStatus";
}
unsafe impl ::std::marker::Send for DataPlanStatus {}
unsafe impl ::std::marker::Sync for DataPlanStatus {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DataPlanUsage(::windows::runtime::IInspectable);
impl DataPlanUsage {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn MegabytesUsed(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn LastSyncTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataPlanUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataPlanUsage;{b921492d-3b44-47ff-b361-be59e69ed1b0})");
}
unsafe impl ::windows::runtime::Interface for DataPlanUsage {
    type Vtable = IDataPlanUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3105966381, 15172, 18431, [179, 97, 190, 89, 230, 158, 209, 176]);
}
impl ::windows::runtime::RuntimeName for DataPlanUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanUsage";
}
unsafe impl ::std::marker::Send for DataPlanUsage {}
unsafe impl ::std::marker::Sync for DataPlanUsage {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DataUsage(::windows::runtime::IInspectable);
impl DataUsage {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesSent(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesReceived(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataUsage;{c1431dd3-b146-4d39-b959-0c69b096c512})");
}
unsafe impl ::windows::runtime::Interface for DataUsage {
    type Vtable = IDataUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3242401235, 45382, 19769, [185, 89, 12, 105, 176, 150, 197, 18]);
}
impl ::windows::runtime::RuntimeName for DataUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataUsage";
}
unsafe impl ::std::marker::Send for DataUsage {}
unsafe impl ::std::marker::Sync for DataUsage {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataUsageGranularity(pub i32);
impl DataUsageGranularity {
    pub const PerMinute: DataUsageGranularity = DataUsageGranularity(0i32);
    pub const PerHour: DataUsageGranularity = DataUsageGranularity(1i32);
    pub const PerDay: DataUsageGranularity = DataUsageGranularity(2i32);
    pub const Total: DataUsageGranularity = DataUsageGranularity(3i32);
}
impl ::std::convert::From<i32> for DataUsageGranularity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DataUsageGranularity {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DataUsageGranularity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DataUsageGranularity;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: DomainConnectivityLevel = DomainConnectivityLevel(0i32);
    pub const Unauthenticated: DomainConnectivityLevel = DomainConnectivityLevel(1i32);
    pub const Authenticated: DomainConnectivityLevel = DomainConnectivityLevel(2i32);
}
impl ::std::convert::From<i32> for DomainConnectivityLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DomainConnectivityLevel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DomainConnectivityLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DomainConnectivityLevel;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAttributedNetworkUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4150898745, 60578, 17899, [173, 225, 176, 54, 139, 117, 108, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttributedNetworkUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICellularApnContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICellularApnContext {
    type Vtable = ICellularApnContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1873095156, 61437, 17730, [154, 178, 112, 91, 191, 148, 148, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICellularApnContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CellularApnAuthenticationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CellularApnAuthenticationType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICellularApnContext2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICellularApnContext2 {
    type Vtable = ICellularApnContext2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1991306010, 44105, 17232, [177, 229, 220, 71, 99, 188, 105, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICellularApnContext2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionCost(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionCost {
    type Vtable = IConnectionCost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134707753, 13334, 19216, [162, 2, 186, 192, 176, 117, 189, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionCost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkCostType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionCost2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionCost2 {
    type Vtable = IConnectionCost2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2383493637, 57865, 17737, [187, 37, 94, 13, 182, 145, 203, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionCost2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfile {
    type Vtable = IConnectionProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908020284, 22926, 18896, [132, 235, 143, 235, 174, 220, 193, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkConnectivityLevel) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfile2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfile2 {
    type Vtable = IConnectionProfile2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3791933765, 19615, 16396, [145, 80, 126, 199, 214, 226, 136, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DomainConnectivityLevel) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfile3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfile3 {
    type Vtable = IConnectionProfile3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1468802344, 19673, 16737, [128, 69, 32, 28, 253, 91, 17, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfile4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfile4 {
    type Vtable = IConnectionProfile4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2049786573, 33248, 19174, [171, 237, 171, 156, 161, 62, 183, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfile5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfile5 {
    type Vtable = IConnectionProfile5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2234916551, 40051, 19424, [143, 20, 87, 142, 236, 113, 238, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfileFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541883592, 48429, 20109, [164, 179, 69, 94, 195, 55, 56, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: NetworkCostType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkCostType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfileFilter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfileFilter2 {
    type Vtable = IConnectionProfileFilter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3439759073, 50172, 20397, [157, 220, 89, 63, 170, 75, 120, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionProfileFilter3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionProfileFilter3 {
    type Vtable = IConnectionProfileFilter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(178915776, 20500, 17532, [136, 9, 174, 228, 203, 10, 249, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectionSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionSession {
    type Vtable = IConnectionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4287651148, 63547, 16816, [138, 12, 20, 98, 217, 197, 107, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionSession_abi(
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
#[doc(hidden)]
pub struct IConnectivityInterval(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectivityInterval {
    type Vtable = IConnectivityInterval_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1336557567, 26438, 18468, [169, 100, 238, 216, 232, 127, 135, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectivityInterval_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IConnectivityManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectivityManagerStatics {
    type Vtable = IConnectivityManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1361106097, 20401, 18608, [175, 201, 66, 224, 9, 42, 129, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectivityManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cellularapncontext: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, routepolicy: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, routepolicy: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPlanStatus(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPlanStatus {
    type Vtable = IDataPlanStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541390732, 14469, 16627, [136, 81, 66, 205, 43, 213, 104, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPlanStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataPlanUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataPlanUsage {
    type Vtable = IDataPlanUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3105966381, 15172, 18431, [179, 97, 190, 89, 230, 158, 209, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPlanUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDataUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataUsage {
    type Vtable = IDataUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3242401235, 45382, 19769, [185, 89, 12, 105, 176, 150, 197, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IIPInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIPInformation {
    type Vtable = IIPInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3629204960, 5007, 18391, [155, 58, 54, 187, 72, 140, 239, 51]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIPInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ILanIdentifier(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanIdentifier {
    type Vtable = ILanIdentifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1219122090, 4360, 17734, [166, 203, 154, 116, 218, 75, 123, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanIdentifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ILanIdentifierData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanIdentifierData {
    type Vtable = ILanIdentifierData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2806940611, 54841, 17854, [163, 106, 196, 228, 174, 175, 109, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanIdentifierData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkAdapter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkAdapter {
    type Vtable = INetworkAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(995372547, 21384, 18796, [168, 163, 175, 253, 57, 174, 194, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkInformationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkInformationStatics {
    type Vtable = INetworkInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1349843025, 38157, 16741, [156, 21, 54, 86, 25, 72, 30, 234]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destinationlist: ::windows::runtime::RawPtr, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkstatushandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkInformationStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkInformationStatics2 {
    type Vtable = INetworkInformationStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1167912212, 10290, 18870, [186, 110, 226, 101, 240, 71, 134, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkInformationStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprofilefilter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkItem {
    type Vtable = INetworkItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29117753, 62944, 17767, [162, 140, 66, 8, 12, 131, 27, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkTypes) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkSecuritySettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2090892941, 37243, 19295, [184, 77, 40, 247, 165, 172, 84, 2]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSecuritySettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkAuthenticationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkEncryptionType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(520942387, 55206, 17629, [164, 233, 104, 124, 71, 107, 144, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkStateChangeEventDetails2 {
    type Vtable = INetworkStateChangeEventDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3594764520, 12499, 20330, [173, 71, 106, 24, 115, 206, 179, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkUsage {
    type Vtable = INetworkUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1239060430, 39301, 18727, [191, 91, 7, 43, 92, 101, 248, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPInformation(::windows::runtime::IInspectable);
impl IPInformation {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkAdapter(&self) -> ::windows::runtime::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkAdapter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn PrefixLength(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u8>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.IPInformation;{d85145e0-138f-47d7-9b3a-36bb488cef33})");
}
unsafe impl ::windows::runtime::Interface for IPInformation {
    type Vtable = IIPInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3629204960, 5007, 18391, [155, 58, 54, 187, 72, 140, 239, 51]);
}
impl ::windows::runtime::RuntimeName for IPInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.IPInformation";
}
unsafe impl ::std::marker::Send for IPInformation {}
unsafe impl ::std::marker::Sync for IPInformation {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProviderNetworkUsage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1590074884, 31025, 18632, [184, 243, 70, 48, 15, 164, 39, 40]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderNetworkUsage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProxyConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProxyConfiguration {
    type Vtable = IProxyConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4013580468, 36868, 19926, [183, 216, 179, 229, 2, 244, 170, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRoutePolicy(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRoutePolicy {
    type Vtable = IRoutePolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296469676, 4039, 17124, [135, 66, 86, 153, 35, 177, 202, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutePolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::DomainNameType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRoutePolicyFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRoutePolicyFactory {
    type Vtable = IRoutePolicyFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(906131763, 41358, 19893, [166, 151, 245, 143, 167, 54, 78, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutePolicyFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionprofile: ::windows::runtime::RawPtr, hostname: ::windows::runtime::RawPtr, r#type: super::DomainNameType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWlanConnectionProfileDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1444976843, 45914, 19441, [168, 132, 183, 85, 126, 136, 255, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWlanConnectionProfileDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(239970558, 33631, 19955, [130, 253, 223, 85, 110, 188, 9, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WwanNetworkRegistrationState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WwanDataClass) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWwanConnectionProfileDetails2 {
    type Vtable = IWwanConnectionProfileDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054508254, 41453, 18610, [142, 146, 180, 96, 3, 61, 82, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WwanNetworkIPKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LanIdentifier(::windows::runtime::IInspectable);
impl LanIdentifier {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn InfrastructureId(&self) -> ::windows::runtime::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanIdentifierData>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn PortId(&self) -> ::windows::runtime::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanIdentifierData>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkAdapterId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LanIdentifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.LanIdentifier;{48aa53aa-1108-4546-a6cb-9a74da4b7ba0})");
}
unsafe impl ::windows::runtime::Interface for LanIdentifier {
    type Vtable = ILanIdentifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1219122090, 4360, 17734, [166, 203, 154, 116, 218, 75, 123, 160]);
}
impl ::windows::runtime::RuntimeName for LanIdentifier {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifier";
}
unsafe impl ::std::marker::Send for LanIdentifier {}
unsafe impl ::std::marker::Sync for LanIdentifier {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LanIdentifierData(::windows::runtime::IInspectable);
impl LanIdentifierData {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u8>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LanIdentifierData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.LanIdentifierData;{a74e83c3-d639-45be-a36a-c4e4aeaf6d9b})");
}
unsafe impl ::windows::runtime::Interface for LanIdentifierData {
    type Vtable = ILanIdentifierData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2806940611, 54841, 17854, [163, 106, 196, 228, 174, 175, 109, 155]);
}
impl ::windows::runtime::RuntimeName for LanIdentifierData {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifierData";
}
unsafe impl ::std::marker::Send for LanIdentifierData {}
unsafe impl ::std::marker::Sync for LanIdentifierData {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct NetworkAdapter(::windows::runtime::IInspectable);
impl NetworkAdapter {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn OutboundMaxBitsPerSecond(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn InboundMaxBitsPerSecond(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IanaInterfaceType(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkItem(&self) -> ::windows::runtime::Result<NetworkItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkItem>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkAdapterId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn GetConnectedProfileAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConnectionProfile>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkAdapter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkAdapter;{3b542e03-5388-496c-a8a3-affd39aec2e6})");
}
unsafe impl ::windows::runtime::Interface for NetworkAdapter {
    type Vtable = INetworkAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(995372547, 21384, 18796, [168, 163, 175, 253, 57, 174, 194, 230]);
}
impl ::windows::runtime::RuntimeName for NetworkAdapter {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkAdapter";
}
unsafe impl ::std::marker::Send for NetworkAdapter {}
unsafe impl ::std::marker::Sync for NetworkAdapter {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkAuthenticationType(pub i32);
impl NetworkAuthenticationType {
    pub const None: NetworkAuthenticationType = NetworkAuthenticationType(0i32);
    pub const Unknown: NetworkAuthenticationType = NetworkAuthenticationType(1i32);
    pub const Open80211: NetworkAuthenticationType = NetworkAuthenticationType(2i32);
    pub const SharedKey80211: NetworkAuthenticationType = NetworkAuthenticationType(3i32);
    pub const Wpa: NetworkAuthenticationType = NetworkAuthenticationType(4i32);
    pub const WpaPsk: NetworkAuthenticationType = NetworkAuthenticationType(5i32);
    pub const WpaNone: NetworkAuthenticationType = NetworkAuthenticationType(6i32);
    pub const Rsna: NetworkAuthenticationType = NetworkAuthenticationType(7i32);
    pub const RsnaPsk: NetworkAuthenticationType = NetworkAuthenticationType(8i32);
    pub const Ihv: NetworkAuthenticationType = NetworkAuthenticationType(9i32);
    pub const Wpa3: NetworkAuthenticationType = NetworkAuthenticationType(10i32);
    pub const Wpa3Enterprise192Bits: NetworkAuthenticationType = NetworkAuthenticationType(10i32);
    pub const Wpa3Sae: NetworkAuthenticationType = NetworkAuthenticationType(11i32);
    pub const Owe: NetworkAuthenticationType = NetworkAuthenticationType(12i32);
    pub const Wpa3Enterprise: NetworkAuthenticationType = NetworkAuthenticationType(13i32);
}
impl ::std::convert::From<i32> for NetworkAuthenticationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkAuthenticationType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkAuthenticationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkAuthenticationType;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: NetworkConnectivityLevel = NetworkConnectivityLevel(0i32);
    pub const LocalAccess: NetworkConnectivityLevel = NetworkConnectivityLevel(1i32);
    pub const ConstrainedInternetAccess: NetworkConnectivityLevel = NetworkConnectivityLevel(2i32);
    pub const InternetAccess: NetworkConnectivityLevel = NetworkConnectivityLevel(3i32);
}
impl ::std::convert::From<i32> for NetworkConnectivityLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkConnectivityLevel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkConnectivityLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkConnectivityLevel;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: NetworkCostType = NetworkCostType(0i32);
    pub const Unrestricted: NetworkCostType = NetworkCostType(1i32);
    pub const Fixed: NetworkCostType = NetworkCostType(2i32);
    pub const Variable: NetworkCostType = NetworkCostType(3i32);
}
impl ::std::convert::From<i32> for NetworkCostType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkCostType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkCostType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkCostType;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: NetworkEncryptionType = NetworkEncryptionType(0i32);
    pub const Unknown: NetworkEncryptionType = NetworkEncryptionType(1i32);
    pub const Wep: NetworkEncryptionType = NetworkEncryptionType(2i32);
    pub const Wep40: NetworkEncryptionType = NetworkEncryptionType(3i32);
    pub const Wep104: NetworkEncryptionType = NetworkEncryptionType(4i32);
    pub const Tkip: NetworkEncryptionType = NetworkEncryptionType(5i32);
    pub const Ccmp: NetworkEncryptionType = NetworkEncryptionType(6i32);
    pub const WpaUseGroup: NetworkEncryptionType = NetworkEncryptionType(7i32);
    pub const RsnUseGroup: NetworkEncryptionType = NetworkEncryptionType(8i32);
    pub const Ihv: NetworkEncryptionType = NetworkEncryptionType(9i32);
    pub const Gcmp: NetworkEncryptionType = NetworkEncryptionType(10i32);
    pub const Gcmp256: NetworkEncryptionType = NetworkEncryptionType(11i32);
}
impl ::std::convert::From<i32> for NetworkEncryptionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkEncryptionType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkEncryptionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkEncryptionType;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
pub struct NetworkInformation {}
impl NetworkInformation {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn GetConnectionProfiles() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetInternetConnectionProfile() -> ::windows::runtime::Result<ConnectionProfile> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConnectionProfile>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn GetLanIdentifiers() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<LanIdentifier>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn GetHostNames() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::HostName>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn GetProxyConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn GetSortedEndpointPairs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::EndpointPair>>>(destinationlist: Param0, sortoptions: super::HostNameSortOptions) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), destinationlist.into_param().abi(), sortoptions, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn NetworkStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, NetworkStatusChangedEventHandler>>(networkstatushandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), networkstatushandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn RemoveNetworkStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(eventcookie: Param0) -> ::windows::runtime::Result<()> {
        Self::INetworkInformationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindConnectionProfilesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ConnectionProfileFilter>>(pprofilefilter: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>> {
        Self::INetworkInformationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pprofilefilter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>(result__)
        })
    }
    pub fn INetworkInformationStatics<R, F: FnOnce(&INetworkInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkInformation, INetworkInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn INetworkInformationStatics2<R, F: FnOnce(&INetworkInformationStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkInformation, INetworkInformationStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for NetworkInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkInformation";
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct NetworkItem(::windows::runtime::IInspectable);
impl NetworkItem {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetNetworkTypes(&self) -> ::windows::runtime::Result<NetworkTypes> {
        let this = self;
        unsafe {
            let mut result__: NetworkTypes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkTypes>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkItem;{01bc4d39-f5e0-4567-a28c-42080c831b2b})");
}
unsafe impl ::windows::runtime::Interface for NetworkItem {
    type Vtable = INetworkItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29117753, 62944, 17767, [162, 140, 66, 8, 12, 131, 27, 43]);
}
impl ::windows::runtime::RuntimeName for NetworkItem {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkItem";
}
unsafe impl ::std::marker::Send for NetworkItem {}
unsafe impl ::std::marker::Sync for NetworkItem {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct NetworkSecuritySettings(::windows::runtime::IInspectable);
impl NetworkSecuritySettings {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkAuthenticationType(&self) -> ::windows::runtime::Result<NetworkAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__: NetworkAuthenticationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn NetworkEncryptionType(&self) -> ::windows::runtime::Result<NetworkEncryptionType> {
        let this = self;
        unsafe {
            let mut result__: NetworkEncryptionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkEncryptionType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkSecuritySettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkSecuritySettings;{7ca07e8d-917b-4b5f-b84d-28f7a5ac5402})");
}
unsafe impl ::windows::runtime::Interface for NetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2090892941, 37243, 19295, [184, 77, 40, 247, 165, 172, 84, 2]);
}
impl ::windows::runtime::RuntimeName for NetworkSecuritySettings {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkSecuritySettings";
}
unsafe impl ::std::marker::Send for NetworkSecuritySettings {}
unsafe impl ::std::marker::Sync for NetworkSecuritySettings {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct NetworkStateChangeEventDetails(::windows::runtime::IInspectable);
impl NetworkStateChangeEventDetails {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewInternetConnectionProfile(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewConnectionCost(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewNetworkConnectivityLevel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewDomainConnectivityLevel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewHostNameList(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewWwanRegistrationState(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewTetheringOperationalState(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HasNewTetheringClientCount(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkStateChangeEventDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkStateChangeEventDetails;{1f0cf333-d7a6-44dd-a4e9-687c476b903d})");
}
unsafe impl ::windows::runtime::Interface for NetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(520942387, 55206, 17629, [164, 233, 104, 124, 71, 107, 144, 61]);
}
impl ::windows::runtime::RuntimeName for NetworkStateChangeEventDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkStateChangeEventDetails";
}
unsafe impl ::std::marker::Send for NetworkStateChangeEventDetails {}
unsafe impl ::std::marker::Sync for NetworkStateChangeEventDetails {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkStatusChangedEventHandler(::windows::runtime::IUnknown);
impl NetworkStatusChangedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = NetworkStatusChangedEventHandler_box::<F> {
            vtable: &NetworkStatusChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkStatusChangedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({71ba143f-598e-49d0-84eb-8febaedcc195})");
}
unsafe impl ::windows::runtime::Interface for NetworkStatusChangedEventHandler {
    type Vtable = NetworkStatusChangedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908020287, 22926, 18896, [132, 235, 143, 235, 174, 220, 193, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct NetworkStatusChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NetworkStatusChangedEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const NetworkStatusChangedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static> NetworkStatusChangedEventHandler_box<F> {
    const VTABLE: NetworkStatusChangedEventHandler_abi = NetworkStatusChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NetworkStatusChangedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkTypes(pub u32);
impl NetworkTypes {
    pub const None: NetworkTypes = NetworkTypes(0u32);
    pub const Internet: NetworkTypes = NetworkTypes(1u32);
    pub const PrivateNetwork: NetworkTypes = NetworkTypes(2u32);
}
impl ::std::convert::From<u32> for NetworkTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkTypes {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkTypes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkTypes;u4)");
}
impl ::std::ops::BitOr for NetworkTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NetworkTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NetworkTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NetworkTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NetworkTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct NetworkUsage(::windows::runtime::IInspectable);
impl NetworkUsage {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesSent(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesReceived(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`*"]
    pub fn ConnectionDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkUsage;{49da8fce-9985-4927-bf5b-072b5c65f8d9})");
}
unsafe impl ::windows::runtime::Interface for NetworkUsage {
    type Vtable = INetworkUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1239060430, 39301, 18727, [191, 91, 7, 43, 92, 101, 248, 217]);
}
impl ::windows::runtime::RuntimeName for NetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkUsage";
}
unsafe impl ::std::marker::Send for NetworkUsage {}
unsafe impl ::std::marker::Sync for NetworkUsage {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Networking_Connectivity`*"]
pub struct NetworkUsageStates {
    pub Roaming: TriStates,
    pub Shared: TriStates,
}
impl NetworkUsageStates {}
impl ::std::default::Default for NetworkUsageStates {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NetworkUsageStates {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NetworkUsageStates").field("Roaming", &self.Roaming).field("Shared", &self.Shared).finish()
    }
}
impl ::std::cmp::PartialEq for NetworkUsageStates {
    fn eq(&self, other: &Self) -> bool {
        self.Roaming == other.Roaming && self.Shared == other.Shared
    }
}
impl ::std::cmp::Eq for NetworkUsageStates {}
unsafe impl ::windows::runtime::Abi for NetworkUsageStates {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkUsageStates {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Networking.Connectivity.NetworkUsageStates;enum(Windows.Networking.Connectivity.TriStates;i4);enum(Windows.Networking.Connectivity.TriStates;i4))");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ProviderNetworkUsage(::windows::runtime::IInspectable);
impl ProviderNetworkUsage {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesSent(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn BytesReceived(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProviderNetworkUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ProviderNetworkUsage;{5ec69e04-7931-48c8-b8f3-46300fa42728})");
}
unsafe impl ::windows::runtime::Interface for ProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1590074884, 31025, 18632, [184, 243, 70, 48, 15, 164, 39, 40]);
}
impl ::windows::runtime::RuntimeName for ProviderNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProviderNetworkUsage";
}
unsafe impl ::std::marker::Send for ProviderNetworkUsage {}
unsafe impl ::std::marker::Sync for ProviderNetworkUsage {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ProxyConfiguration(::windows::runtime::IInspectable);
impl ProxyConfiguration {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation`, `Foundation_Collections`*"]
    pub fn ProxyUris(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn CanConnectDirectly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProxyConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ProxyConfiguration;{ef3a60b4-9004-4dd6-b7d8-b3e502f4aad0})");
}
unsafe impl ::windows::runtime::Interface for ProxyConfiguration {
    type Vtable = IProxyConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4013580468, 36868, 19926, [183, 216, 179, 229, 2, 244, 170, 208]);
}
impl ::windows::runtime::RuntimeName for ProxyConfiguration {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProxyConfiguration";
}
unsafe impl ::std::marker::Send for ProxyConfiguration {}
unsafe impl ::std::marker::Sync for ProxyConfiguration {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RoamingStates(pub u32);
impl RoamingStates {
    pub const None: RoamingStates = RoamingStates(0u32);
    pub const NotRoaming: RoamingStates = RoamingStates(1u32);
    pub const Roaming: RoamingStates = RoamingStates(2u32);
}
impl ::std::convert::From<u32> for RoamingStates {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RoamingStates {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RoamingStates {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.RoamingStates;u4)");
}
impl ::std::ops::BitOr for RoamingStates {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RoamingStates {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RoamingStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RoamingStates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RoamingStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RoutePolicy(::windows::runtime::IInspectable);
impl RoutePolicy {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn ConnectionProfile(&self) -> ::windows::runtime::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConnectionProfile>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HostName(&self) -> ::windows::runtime::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HostNameType(&self) -> ::windows::runtime::Result<super::DomainNameType> {
        let this = self;
        unsafe {
            let mut result__: super::DomainNameType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::DomainNameType>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn CreateRoutePolicy<'a, Param0: ::windows::runtime::IntoParam<'a, ConnectionProfile>, Param1: ::windows::runtime::IntoParam<'a, super::HostName>>(connectionprofile: Param0, hostname: Param1, r#type: super::DomainNameType) -> ::windows::runtime::Result<RoutePolicy> {
        Self::IRoutePolicyFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectionprofile.into_param().abi(), hostname.into_param().abi(), r#type, &mut result__).from_abi::<RoutePolicy>(result__)
        })
    }
    pub fn IRoutePolicyFactory<R, F: FnOnce(&IRoutePolicyFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RoutePolicy, IRoutePolicyFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RoutePolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.RoutePolicy;{11abc4ac-0fc7-42e4-8742-569923b1ca11})");
}
unsafe impl ::windows::runtime::Interface for RoutePolicy {
    type Vtable = IRoutePolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296469676, 4039, 17124, [135, 66, 86, 153, 35, 177, 202, 17]);
}
impl ::windows::runtime::RuntimeName for RoutePolicy {
    const NAME: &'static str = "Windows.Networking.Connectivity.RoutePolicy";
}
unsafe impl ::std::marker::Send for RoutePolicy {}
unsafe impl ::std::marker::Sync for RoutePolicy {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: TriStates = TriStates(0i32);
    pub const No: TriStates = TriStates(1i32);
    pub const Yes: TriStates = TriStates(2i32);
}
impl ::std::convert::From<i32> for TriStates {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TriStates {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TriStates {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.TriStates;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WlanConnectionProfileDetails(::windows::runtime::IInspectable);
impl WlanConnectionProfileDetails {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetConnectedSsid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WlanConnectionProfileDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.WlanConnectionProfileDetails;{562098cb-b35a-4bf1-a884-b7557e88ff86})");
}
unsafe impl ::windows::runtime::Interface for WlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1444976843, 45914, 19441, [168, 132, 183, 85, 126, 136, 255, 134]);
}
impl ::windows::runtime::RuntimeName for WlanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WlanConnectionProfileDetails";
}
unsafe impl ::std::marker::Send for WlanConnectionProfileDetails {}
unsafe impl ::std::marker::Sync for WlanConnectionProfileDetails {}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WwanConnectionProfileDetails(::windows::runtime::IInspectable);
impl WwanConnectionProfileDetails {
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn HomeProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn AccessPointName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetNetworkRegistrationState(&self) -> ::windows::runtime::Result<WwanNetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__: WwanNetworkRegistrationState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WwanNetworkRegistrationState>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn GetCurrentDataClass(&self) -> ::windows::runtime::Result<WwanDataClass> {
        let this = self;
        unsafe {
            let mut result__: WwanDataClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WwanDataClass>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Connectivity`*"]
    pub fn IPKind(&self) -> ::windows::runtime::Result<WwanNetworkIPKind> {
        let this = &::windows::runtime::Interface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__: WwanNetworkIPKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WwanNetworkIPKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Connectivity`, `Foundation_Collections`*"]
    pub fn PurposeGuids(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::GUID>> {
        let this = &::windows::runtime::Interface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::GUID>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WwanConnectionProfileDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.WwanConnectionProfileDetails;{0e4da8fe-835f-4df3-82fd-df556ebc09ef})");
}
unsafe impl ::windows::runtime::Interface for WwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(239970558, 33631, 19955, [130, 253, 223, 85, 110, 188, 9, 239]);
}
impl ::windows::runtime::RuntimeName for WwanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WwanConnectionProfileDetails";
}
unsafe impl ::std::marker::Send for WwanConnectionProfileDetails {}
unsafe impl ::std::marker::Sync for WwanConnectionProfileDetails {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct WwanContract(pub u8);
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WwanDataClass(pub u32);
impl WwanDataClass {
    pub const None: WwanDataClass = WwanDataClass(0u32);
    pub const Gprs: WwanDataClass = WwanDataClass(1u32);
    pub const Edge: WwanDataClass = WwanDataClass(2u32);
    pub const Umts: WwanDataClass = WwanDataClass(4u32);
    pub const Hsdpa: WwanDataClass = WwanDataClass(8u32);
    pub const Hsupa: WwanDataClass = WwanDataClass(16u32);
    pub const LteAdvanced: WwanDataClass = WwanDataClass(32u32);
    pub const Cdma1xRtt: WwanDataClass = WwanDataClass(65536u32);
    pub const Cdma1xEvdo: WwanDataClass = WwanDataClass(131072u32);
    pub const Cdma1xEvdoRevA: WwanDataClass = WwanDataClass(262144u32);
    pub const Cdma1xEvdv: WwanDataClass = WwanDataClass(524288u32);
    pub const Cdma3xRtt: WwanDataClass = WwanDataClass(1048576u32);
    pub const Cdma1xEvdoRevB: WwanDataClass = WwanDataClass(2097152u32);
    pub const CdmaUmb: WwanDataClass = WwanDataClass(4194304u32);
    pub const Custom: WwanDataClass = WwanDataClass(2147483648u32);
}
impl ::std::convert::From<u32> for WwanDataClass {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WwanDataClass {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WwanDataClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanDataClass;u4)");
}
impl ::std::ops::BitOr for WwanDataClass {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WwanDataClass {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WwanDataClass {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WwanDataClass {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WwanDataClass {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: WwanNetworkIPKind = WwanNetworkIPKind(0i32);
    pub const Ipv4: WwanNetworkIPKind = WwanNetworkIPKind(1i32);
    pub const Ipv6: WwanNetworkIPKind = WwanNetworkIPKind(2i32);
    pub const Ipv4v6: WwanNetworkIPKind = WwanNetworkIPKind(3i32);
    pub const Ipv4v6v4Xlat: WwanNetworkIPKind = WwanNetworkIPKind(4i32);
}
impl ::std::convert::From<i32> for WwanNetworkIPKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WwanNetworkIPKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WwanNetworkIPKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkIPKind;i4)");
}
#[doc = "*Required features: `Networking_Connectivity`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: WwanNetworkRegistrationState = WwanNetworkRegistrationState(0i32);
    pub const Deregistered: WwanNetworkRegistrationState = WwanNetworkRegistrationState(1i32);
    pub const Searching: WwanNetworkRegistrationState = WwanNetworkRegistrationState(2i32);
    pub const Home: WwanNetworkRegistrationState = WwanNetworkRegistrationState(3i32);
    pub const Roaming: WwanNetworkRegistrationState = WwanNetworkRegistrationState(4i32);
    pub const Partner: WwanNetworkRegistrationState = WwanNetworkRegistrationState(5i32);
    pub const Denied: WwanNetworkRegistrationState = WwanNetworkRegistrationState(6i32);
}
impl ::std::convert::From<i32> for WwanNetworkRegistrationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WwanNetworkRegistrationState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WwanNetworkRegistrationState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkRegistrationState;i4)");
}
