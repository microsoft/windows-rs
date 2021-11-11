#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiAdapter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiAdapter {
    type Vtable = IWiFiAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4e423_3d75_43a4_b9de_11e26b72d9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, args: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiAdapter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiAdapter2 {
    type Vtable = IWiFiAdapter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bc4501d_81e4_453d_9430_1fcafbadd6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availablenetwork: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiAdapterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiAdapterStatics {
    type Vtable = IWiFiAdapterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda25fddd_d24c_43e3_aabd_c4659f730f99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiAvailableNetwork(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26e96246_183e_4704_9826_71b4a2f0f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAvailableNetwork_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiNetworkKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiPhyKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiConnectionResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143bdfd9_c37d_40be_a5c8_857bce85a931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiConnectionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiConnectionStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiNetworkReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9524ded2_5911_445e_8194_be4f1a704895);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiNetworkReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b49871_17ee_42d1_b14f_5a11f1226fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiWpsConfigurationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: WiFiAccessStatus = WiFiAccessStatus(0i32);
    pub const Allowed: WiFiAccessStatus = WiFiAccessStatus(1i32);
    pub const DeniedByUser: WiFiAccessStatus = WiFiAccessStatus(2i32);
    pub const DeniedBySystem: WiFiAccessStatus = WiFiAccessStatus(3i32);
}
impl ::core::convert::From<i32> for WiFiAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiAccessStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiAdapter(pub ::windows::core::IInspectable);
impl WiFiAdapter {
    #[cfg(feature = "Networking_Connectivity")]
    #[doc = "*Required features: `Devices_WiFi`, `Networking_Connectivity`*"]
    pub fn NetworkAdapter(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Connectivity::NetworkAdapter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn ScanAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn NetworkReport(&self) -> ::windows::core::Result<WiFiNetworkReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiNetworkReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn AvailableNetworksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable>>>(&self, args: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), args.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn RemoveAvailableNetworksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Security_Credentials`*"]
    pub fn ConnectWithPasswordCredentialAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Security_Credentials`*"]
    pub fn ConnectWithPasswordCredentialAndSsidAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2, ssid: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Disconnect(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAdaptersAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiAdapter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn GetWpsConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>> {
        let this = &::windows::core::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`, `Security_Credentials`*"]
    pub fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        availablenetwork: Param0,
        reconnectionkind: WiFiReconnectionKind,
        passwordcredential: Param2,
        ssid: Param3,
        connectionmethod: WiFiConnectionMethod,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = &::windows::core::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), connectionmethod, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    pub fn IWiFiAdapterStatics<R, F: FnOnce(&IWiFiAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiAdapter, IWiFiAdapterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAdapter;{a6c4e423-3d75-43a4-b9de-11e26b72d9b0})");
}
unsafe impl ::windows::core::Interface for WiFiAdapter {
    type Vtable = IWiFiAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4e423_3d75_43a4_b9de_11e26b72d9b0);
}
impl ::windows::core::RuntimeName for WiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAdapter";
}
impl ::core::convert::From<WiFiAdapter> for ::windows::core::IUnknown {
    fn from(value: WiFiAdapter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiAdapter> for ::windows::core::IUnknown {
    fn from(value: &WiFiAdapter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiAdapter> for ::windows::core::IInspectable {
    fn from(value: WiFiAdapter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiAdapter> for ::windows::core::IInspectable {
    fn from(value: &WiFiAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiAdapter {}
unsafe impl ::core::marker::Sync for WiFiAdapter {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiAvailableNetwork(pub ::windows::core::IInspectable);
impl WiFiAvailableNetwork {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn Uptime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Bssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn SignalBars(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn NetworkKind(&self) -> ::windows::core::Result<WiFiNetworkKind> {
        let this = self;
        unsafe {
            let mut result__: WiFiNetworkKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiNetworkKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn PhyKind(&self) -> ::windows::core::Result<WiFiPhyKind> {
        let this = self;
        unsafe {
            let mut result__: WiFiPhyKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiPhyKind>(result__)
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    #[doc = "*Required features: `Devices_WiFi`, `Networking_Connectivity`*"]
    pub fn SecuritySettings(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Connectivity::NetworkSecuritySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn BeaconInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn IsWiFiDirect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiAvailableNetwork {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAvailableNetwork;{26e96246-183e-4704-9826-71b4a2f0f668})");
}
unsafe impl ::windows::core::Interface for WiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26e96246_183e_4704_9826_71b4a2f0f668);
}
impl ::windows::core::RuntimeName for WiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAvailableNetwork";
}
impl ::core::convert::From<WiFiAvailableNetwork> for ::windows::core::IUnknown {
    fn from(value: WiFiAvailableNetwork) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiAvailableNetwork> for ::windows::core::IUnknown {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiAvailableNetwork> for ::windows::core::IInspectable {
    fn from(value: WiFiAvailableNetwork) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiAvailableNetwork> for ::windows::core::IInspectable {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiAvailableNetwork {}
unsafe impl ::core::marker::Sync for WiFiAvailableNetwork {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: WiFiConnectionMethod = WiFiConnectionMethod(0i32);
    pub const WpsPin: WiFiConnectionMethod = WiFiConnectionMethod(1i32);
    pub const WpsPushButton: WiFiConnectionMethod = WiFiConnectionMethod(2i32);
}
impl ::core::convert::From<i32> for WiFiConnectionMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiConnectionMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiConnectionMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionMethod;i4)");
}
impl ::windows::core::DefaultType for WiFiConnectionMethod {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiConnectionResult(pub ::windows::core::IInspectable);
impl WiFiConnectionResult {
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiConnectionStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiConnectionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiConnectionResult;{143bdfd9-c37d-40be-a5c8-857bce85a931})");
}
unsafe impl ::windows::core::Interface for WiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143bdfd9_c37d_40be_a5c8_857bce85a931);
}
impl ::windows::core::RuntimeName for WiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiConnectionResult";
}
impl ::core::convert::From<WiFiConnectionResult> for ::windows::core::IUnknown {
    fn from(value: WiFiConnectionResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiConnectionResult> for ::windows::core::IUnknown {
    fn from(value: &WiFiConnectionResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiConnectionResult> for ::windows::core::IInspectable {
    fn from(value: WiFiConnectionResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiConnectionResult> for ::windows::core::IInspectable {
    fn from(value: &WiFiConnectionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiConnectionResult {}
unsafe impl ::core::marker::Sync for WiFiConnectionResult {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for WiFiConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiConnectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiConnectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: WiFiNetworkKind = WiFiNetworkKind(0i32);
    pub const Infrastructure: WiFiNetworkKind = WiFiNetworkKind(1i32);
    pub const Adhoc: WiFiNetworkKind = WiFiNetworkKind(2i32);
}
impl ::core::convert::From<i32> for WiFiNetworkKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiNetworkKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiNetworkKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiNetworkKind;i4)");
}
impl ::windows::core::DefaultType for WiFiNetworkKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiNetworkReport(pub ::windows::core::IInspectable);
impl WiFiNetworkReport {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation_Collections`*"]
    pub fn AvailableNetworks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiNetworkReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiNetworkReport;{9524ded2-5911-445e-8194-be4f1a704895})");
}
unsafe impl ::windows::core::Interface for WiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9524ded2_5911_445e_8194_be4f1a704895);
}
impl ::windows::core::RuntimeName for WiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiNetworkReport";
}
impl ::core::convert::From<WiFiNetworkReport> for ::windows::core::IUnknown {
    fn from(value: WiFiNetworkReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiNetworkReport> for ::windows::core::IUnknown {
    fn from(value: &WiFiNetworkReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiNetworkReport> for ::windows::core::IInspectable {
    fn from(value: WiFiNetworkReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiNetworkReport> for ::windows::core::IInspectable {
    fn from(value: &WiFiNetworkReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiNetworkReport {}
unsafe impl ::core::marker::Sync for WiFiNetworkReport {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for WiFiPhyKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiPhyKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiPhyKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiPhyKind;i4)");
}
impl ::windows::core::DefaultType for WiFiPhyKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: WiFiReconnectionKind = WiFiReconnectionKind(0i32);
    pub const Manual: WiFiReconnectionKind = WiFiReconnectionKind(1i32);
}
impl ::core::convert::From<i32> for WiFiReconnectionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiReconnectionKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiReconnectionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiReconnectionKind;i4)");
}
impl ::windows::core::DefaultType for WiFiReconnectionKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiWpsConfigurationResult(pub ::windows::core::IInspectable);
impl WiFiWpsConfigurationResult {
    #[doc = "*Required features: `Devices_WiFi`*"]
    pub fn Status(&self) -> ::windows::core::Result<WiFiWpsConfigurationStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiWpsConfigurationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiWpsConfigurationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFi`, `Foundation_Collections`*"]
    pub fn SupportedWpsKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiWpsConfigurationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiWpsConfigurationResult;{67b49871-17ee-42d1-b14f-5a11f1226fb5})");
}
unsafe impl ::windows::core::Interface for WiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b49871_17ee_42d1_b14f_5a11f1226fb5);
}
impl ::windows::core::RuntimeName for WiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiWpsConfigurationResult";
}
impl ::core::convert::From<WiFiWpsConfigurationResult> for ::windows::core::IUnknown {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiWpsConfigurationResult> for ::windows::core::IUnknown {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiWpsConfigurationResult> for ::windows::core::IInspectable {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiWpsConfigurationResult> for ::windows::core::IInspectable {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiWpsConfigurationResult {}
unsafe impl ::core::marker::Sync for WiFiWpsConfigurationResult {}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(0i32);
    pub const Success: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(1i32);
    pub const Timeout: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(2i32);
}
impl ::core::convert::From<i32> for WiFiWpsConfigurationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiWpsConfigurationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiWpsConfigurationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsConfigurationStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiWpsConfigurationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for WiFiWpsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiWpsKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiWpsKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsKind;i4)");
}
impl ::windows::core::DefaultType for WiFiWpsKind {
    type DefaultType = Self;
}
