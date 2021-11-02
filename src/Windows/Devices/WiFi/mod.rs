#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiAdapter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiAdapter {
    type Vtable = IWiFiAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797921315, 15733, 17316, [185, 222, 17, 226, 107, 114, 217, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availablenetwork: ::windows::runtime::RawPtr, reconnectionkind: WiFiReconnectionKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availablenetwork: ::windows::runtime::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availablenetwork: ::windows::runtime::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::runtime::RawPtr, ssid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiAdapter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiAdapter2 {
    type Vtable = IWiFiAdapter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1539592221, 33252, 17725, [148, 48, 31, 202, 251, 173, 214, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availablenetwork: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availablenetwork: ::windows::runtime::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::runtime::RawPtr, ssid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiAdapterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiAdapterStatics {
    type Vtable = IWiFiAdapterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3659922909, 53836, 17379, [170, 189, 196, 101, 159, 115, 15, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiAvailableNetwork(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(652829254, 6206, 18180, [152, 38, 113, 180, 162, 240, 246, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAvailableNetwork_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiNetworkKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiPhyKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiConnectionResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(339468249, 50045, 16574, [165, 200, 133, 123, 206, 133, 169, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiConnectionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiConnectionStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiNetworkReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2502221522, 22801, 17502, [129, 148, 190, 79, 26, 112, 72, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiNetworkReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1739888753, 6126, 17105, [177, 79, 90, 17, 241, 34, 111, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiWpsConfigurationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: WiFiAccessStatus = WiFiAccessStatus(0i32);
    pub const Allowed: WiFiAccessStatus = WiFiAccessStatus(1i32);
    pub const DeniedByUser: WiFiAccessStatus = WiFiAccessStatus(2i32);
    pub const DeniedBySystem: WiFiAccessStatus = WiFiAccessStatus(3i32);
}
impl ::std::convert::From<i32> for WiFiAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiAccessStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiAccessStatus;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiAdapter(::windows::runtime::IInspectable);
impl WiFiAdapter {
    #[cfg(feature = "Networking_Connectivity")]
    #[doc = "*Required features: `Devices_WiFi`, `Networking_Connectivity`*"]
    pub fn NetworkAdapter(&self) -> ::windows::runtime::Result<super::super::Networking::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Connectivity::NetworkAdapter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn ScanAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn NetworkReport(&self) -> ::windows::runtime::Result<WiFiNetworkReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiNetworkReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn AvailableNetworksChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::runtime::IInspectable>>>(&self, args: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), args.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn RemoveAvailableNetworksChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn ConnectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Security_Credentials`*"]
    pub fn ConnectWithPasswordCredentialAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Security_Credentials`*"]
    pub fn ConnectWithPasswordCredentialAndSsidAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        availablenetwork: Param0,
        reconnectionkind: WiFiReconnectionKind,
        passwordcredential: Param2,
        ssid: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAdaptersAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiAdapter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn GetWpsConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>> {
        let this = &::windows::runtime::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), availablenetwork.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Security_Credentials`*"]
    pub fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        availablenetwork: Param0,
        reconnectionkind: WiFiReconnectionKind,
        passwordcredential: Param2,
        ssid: Param3,
        connectionmethod: WiFiConnectionMethod,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = &::windows::runtime::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), connectionmethod, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    pub fn IWiFiAdapterStatics<R, F: FnOnce(&IWiFiAdapterStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiAdapter, IWiFiAdapterStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiAdapter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAdapter;{a6c4e423-3d75-43a4-b9de-11e26b72d9b0})");
}
unsafe impl ::windows::runtime::Interface for WiFiAdapter {
    type Vtable = IWiFiAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797921315, 15733, 17316, [185, 222, 17, 226, 107, 114, 217, 176]);
}
impl ::windows::runtime::RuntimeName for WiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAdapter";
}
impl ::std::convert::From<WiFiAdapter> for ::windows::runtime::IUnknown {
    fn from(value: WiFiAdapter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiAdapter> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiAdapter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiAdapter> for ::windows::runtime::IInspectable {
    fn from(value: WiFiAdapter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiAdapter> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiAdapter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiAdapter {}
unsafe impl ::std::marker::Sync for WiFiAdapter {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiAvailableNetwork(::windows::runtime::IInspectable);
impl WiFiAvailableNetwork {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn Uptime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Ssid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Bssid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn SignalBars(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn NetworkKind(&self) -> ::windows::runtime::Result<WiFiNetworkKind> {
        let this = self;
        unsafe {
            let mut result__: WiFiNetworkKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiNetworkKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn PhyKind(&self) -> ::windows::runtime::Result<WiFiPhyKind> {
        let this = self;
        unsafe {
            let mut result__: WiFiPhyKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiPhyKind>(result__)
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    #[doc = "*Required features: `Devices_WiFi`, `Networking_Connectivity`*"]
    pub fn SecuritySettings(&self) -> ::windows::runtime::Result<super::super::Networking::Connectivity::NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Connectivity::NetworkSecuritySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn BeaconInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn IsWiFiDirect(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiAvailableNetwork {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAvailableNetwork;{26e96246-183e-4704-9826-71b4a2f0f668})");
}
unsafe impl ::windows::runtime::Interface for WiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(652829254, 6206, 18180, [152, 38, 113, 180, 162, 240, 246, 104]);
}
impl ::windows::runtime::RuntimeName for WiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAvailableNetwork";
}
impl ::std::convert::From<WiFiAvailableNetwork> for ::windows::runtime::IUnknown {
    fn from(value: WiFiAvailableNetwork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiAvailableNetwork> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiAvailableNetwork> for ::windows::runtime::IInspectable {
    fn from(value: WiFiAvailableNetwork) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiAvailableNetwork> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiAvailableNetwork {}
unsafe impl ::std::marker::Sync for WiFiAvailableNetwork {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: WiFiConnectionMethod = WiFiConnectionMethod(0i32);
    pub const WpsPin: WiFiConnectionMethod = WiFiConnectionMethod(1i32);
    pub const WpsPushButton: WiFiConnectionMethod = WiFiConnectionMethod(2i32);
}
impl ::std::convert::From<i32> for WiFiConnectionMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiConnectionMethod {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiConnectionMethod {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionMethod;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiConnectionResult(::windows::runtime::IInspectable);
impl WiFiConnectionResult {
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn ConnectionStatus(&self) -> ::windows::runtime::Result<WiFiConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiConnectionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiConnectionStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiConnectionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiConnectionResult;{143bdfd9-c37d-40be-a5c8-857bce85a931})");
}
unsafe impl ::windows::runtime::Interface for WiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(339468249, 50045, 16574, [165, 200, 133, 123, 206, 133, 169, 49]);
}
impl ::windows::runtime::RuntimeName for WiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiConnectionResult";
}
impl ::std::convert::From<WiFiConnectionResult> for ::windows::runtime::IUnknown {
    fn from(value: WiFiConnectionResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiConnectionResult> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiConnectionResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiConnectionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiConnectionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiConnectionResult> for ::windows::runtime::IInspectable {
    fn from(value: WiFiConnectionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiConnectionResult> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiConnectionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiConnectionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiConnectionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiConnectionResult {}
unsafe impl ::std::marker::Sync for WiFiConnectionResult {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: WiFiConnectionStatus = WiFiConnectionStatus(0i32);
    pub const Success: WiFiConnectionStatus = WiFiConnectionStatus(1i32);
    pub const AccessRevoked: WiFiConnectionStatus = WiFiConnectionStatus(2i32);
    pub const InvalidCredential: WiFiConnectionStatus = WiFiConnectionStatus(3i32);
    pub const NetworkNotAvailable: WiFiConnectionStatus = WiFiConnectionStatus(4i32);
    pub const Timeout: WiFiConnectionStatus = WiFiConnectionStatus(5i32);
    pub const UnsupportedAuthenticationProtocol: WiFiConnectionStatus = WiFiConnectionStatus(6i32);
}
impl ::std::convert::From<i32> for WiFiConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiConnectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiConnectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionStatus;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: WiFiNetworkKind = WiFiNetworkKind(0i32);
    pub const Infrastructure: WiFiNetworkKind = WiFiNetworkKind(1i32);
    pub const Adhoc: WiFiNetworkKind = WiFiNetworkKind(2i32);
}
impl ::std::convert::From<i32> for WiFiNetworkKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiNetworkKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiNetworkKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiNetworkKind;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiNetworkReport(::windows::runtime::IInspectable);
impl WiFiNetworkReport {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation_Collections`*"]
    pub fn AvailableNetworks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiNetworkReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiNetworkReport;{9524ded2-5911-445e-8194-be4f1a704895})");
}
unsafe impl ::windows::runtime::Interface for WiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2502221522, 22801, 17502, [129, 148, 190, 79, 26, 112, 72, 149]);
}
impl ::windows::runtime::RuntimeName for WiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiNetworkReport";
}
impl ::std::convert::From<WiFiNetworkReport> for ::windows::runtime::IUnknown {
    fn from(value: WiFiNetworkReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiNetworkReport> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiNetworkReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiNetworkReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiNetworkReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiNetworkReport> for ::windows::runtime::IInspectable {
    fn from(value: WiFiNetworkReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiNetworkReport> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiNetworkReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiNetworkReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiNetworkReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiNetworkReport {}
unsafe impl ::std::marker::Sync for WiFiNetworkReport {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: WiFiPhyKind = WiFiPhyKind(0i32);
    pub const Fhss: WiFiPhyKind = WiFiPhyKind(1i32);
    pub const Dsss: WiFiPhyKind = WiFiPhyKind(2i32);
    pub const IRBaseband: WiFiPhyKind = WiFiPhyKind(3i32);
    pub const Ofdm: WiFiPhyKind = WiFiPhyKind(4i32);
    pub const Hrdsss: WiFiPhyKind = WiFiPhyKind(5i32);
    pub const Erp: WiFiPhyKind = WiFiPhyKind(6i32);
    pub const HT: WiFiPhyKind = WiFiPhyKind(7i32);
    pub const Vht: WiFiPhyKind = WiFiPhyKind(8i32);
    pub const Dmg: WiFiPhyKind = WiFiPhyKind(9i32);
    pub const HE: WiFiPhyKind = WiFiPhyKind(10i32);
}
impl ::std::convert::From<i32> for WiFiPhyKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiPhyKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiPhyKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiPhyKind;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: WiFiReconnectionKind = WiFiReconnectionKind(0i32);
    pub const Manual: WiFiReconnectionKind = WiFiReconnectionKind(1i32);
}
impl ::std::convert::From<i32> for WiFiReconnectionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiReconnectionKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiReconnectionKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiReconnectionKind;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiWpsConfigurationResult(::windows::runtime::IInspectable);
impl WiFiWpsConfigurationResult {
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<WiFiWpsConfigurationStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiWpsConfigurationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiWpsConfigurationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation_Collections`*"]
    pub fn SupportedWpsKinds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiWpsConfigurationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiWpsConfigurationResult;{67b49871-17ee-42d1-b14f-5a11f1226fb5})");
}
unsafe impl ::windows::runtime::Interface for WiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1739888753, 6126, 17105, [177, 79, 90, 17, 241, 34, 111, 181]);
}
impl ::windows::runtime::RuntimeName for WiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiWpsConfigurationResult";
}
impl ::std::convert::From<WiFiWpsConfigurationResult> for ::windows::runtime::IUnknown {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiWpsConfigurationResult> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiWpsConfigurationResult> for ::windows::runtime::IInspectable {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiWpsConfigurationResult> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiWpsConfigurationResult {}
unsafe impl ::std::marker::Sync for WiFiWpsConfigurationResult {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(0i32);
    pub const Success: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(1i32);
    pub const Timeout: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(2i32);
}
impl ::std::convert::From<i32> for WiFiWpsConfigurationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiWpsConfigurationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiWpsConfigurationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsConfigurationStatus;i4)");
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: WiFiWpsKind = WiFiWpsKind(0i32);
    pub const Pin: WiFiWpsKind = WiFiWpsKind(1i32);
    pub const PushButton: WiFiWpsKind = WiFiWpsKind(2i32);
    pub const Nfc: WiFiWpsKind = WiFiWpsKind(3i32);
    pub const Ethernet: WiFiWpsKind = WiFiWpsKind(4i32);
    pub const Usb: WiFiWpsKind = WiFiWpsKind(5i32);
}
impl ::std::convert::From<i32> for WiFiWpsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiWpsKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiWpsKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsKind;i4)");
}
