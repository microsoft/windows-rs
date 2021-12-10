#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAdapter {
    type Vtable = IWiFiAdapterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4e423_3d75_43a4_b9de_11e26b72d9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Foundation", feature = "Security_Credentials")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAdapter2 {
    type Vtable = IWiFiAdapter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bc4501d_81e4_453d_9430_1fcafbadd6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Foundation", feature = "Security_Credentials")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAdapterStatics {
    type Vtable = IWiFiAdapterStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda25fddd_d24c_43e3_aabd_c4659f730f99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapterStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAvailableNetwork(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetworkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26e96246_183e_4704_9826_71b4a2f0f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAvailableNetworkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiNetworkKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiPhyKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiConnectionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiConnectionResult {
    type Vtable = IWiFiConnectionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143bdfd9_c37d_40be_a5c8_857bce85a931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiConnectionResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiConnectionStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiNetworkReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiNetworkReport {
    type Vtable = IWiFiNetworkReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9524ded2_5911_445e_8194_be4f1a704895);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiNetworkReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiWpsConfigurationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b49871_17ee_42d1_b14f_5a11f1226fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiWpsConfigurationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiAccessStatus {}
impl ::core::clone::Clone for WiFiAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiAccessStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiAccessStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAccessStatus {}
unsafe impl ::windows::core::RuntimeType for WiFiAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiAccessStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiAccessStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiAdapter(::windows::core::IUnknown);
impl WiFiAdapter {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Connectivity::NetworkAdapter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ScanAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NetworkReport(&self) -> ::windows::core::Result<WiFiNetworkReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiNetworkReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AvailableNetworksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable>>>(&self, args: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), args.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailableNetworksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectWithPasswordCredentialAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectWithPasswordCredentialAndSsidAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2, ssid: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    pub fn Disconnect(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetWpsConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>> {
        let this = &::windows::core::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync<'a, Param0: ::windows::core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2, ssid: Param3, connectionmethod: WiFiConnectionMethod) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = &::windows::core::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), connectionmethod, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAdaptersAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiAdapter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>(result__)
        })
    }
    pub fn IWiFiAdapterStatics<R, F: FnOnce(&IWiFiAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiAdapter, IWiFiAdapterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAdapter {}
unsafe impl ::windows::core::RuntimeType for WiFiAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAdapter;{a6c4e423-3d75-43a4-b9de-11e26b72d9b0})");
}
unsafe impl ::windows::core::Interface for WiFiAdapter {
    type Vtable = IWiFiAdapterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4e423_3d75_43a4_b9de_11e26b72d9b0);
}
impl ::windows::core::RuntimeName for WiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAdapter";
}
impl ::core::convert::From<WiFiAdapter> for ::windows::core::IUnknown {
    fn from(value: WiFiAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAdapter> for ::windows::core::IUnknown {
    fn from(value: &WiFiAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiAdapter> for ::windows::core::IInspectable {
    fn from(value: WiFiAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAdapter> for ::windows::core::IInspectable {
    fn from(value: &WiFiAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiAdapter {}
unsafe impl ::core::marker::Sync for WiFiAdapter {}
#[repr(transparent)]
pub struct WiFiAvailableNetwork(::windows::core::IUnknown);
impl WiFiAvailableNetwork {
    #[cfg(feature = "Foundation")]
    pub fn Uptime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Bssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SignalBars(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn NetworkKind(&self) -> ::windows::core::Result<WiFiNetworkKind> {
        let this = self;
        unsafe {
            let mut result__: WiFiNetworkKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiNetworkKind>(result__)
        }
    }
    pub fn PhyKind(&self) -> ::windows::core::Result<WiFiPhyKind> {
        let this = self;
        unsafe {
            let mut result__: WiFiPhyKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiPhyKind>(result__)
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn SecuritySettings(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Connectivity::NetworkSecuritySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeaconInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn IsWiFiDirect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiAvailableNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiAvailableNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAvailableNetwork {}
unsafe impl ::windows::core::RuntimeType for WiFiAvailableNetwork {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAvailableNetwork;{26e96246-183e-4704-9826-71b4a2f0f668})");
}
unsafe impl ::windows::core::Interface for WiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetworkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26e96246_183e_4704_9826_71b4a2f0f668);
}
impl ::windows::core::RuntimeName for WiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAvailableNetwork";
}
impl ::core::convert::From<WiFiAvailableNetwork> for ::windows::core::IUnknown {
    fn from(value: WiFiAvailableNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAvailableNetwork> for ::windows::core::IUnknown {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiAvailableNetwork> for ::windows::core::IInspectable {
    fn from(value: WiFiAvailableNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAvailableNetwork> for ::windows::core::IInspectable {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiAvailableNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiAvailableNetwork {}
unsafe impl ::core::marker::Sync for WiFiAvailableNetwork {}
#[repr(transparent)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: Self = Self(0i32);
    pub const WpsPin: Self = Self(1i32);
    pub const WpsPushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiConnectionMethod {}
impl ::core::clone::Clone for WiFiConnectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiConnectionMethod {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiConnectionMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiConnectionMethod {}
unsafe impl ::windows::core::RuntimeType for WiFiConnectionMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionMethod;i4)");
}
impl ::windows::core::DefaultType for WiFiConnectionMethod {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiConnectionResult(::windows::core::IUnknown);
impl WiFiConnectionResult {
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiConnectionStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiConnectionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiConnectionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiConnectionResult {}
unsafe impl ::windows::core::RuntimeType for WiFiConnectionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiConnectionResult;{143bdfd9-c37d-40be-a5c8-857bce85a931})");
}
unsafe impl ::windows::core::Interface for WiFiConnectionResult {
    type Vtable = IWiFiConnectionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143bdfd9_c37d_40be_a5c8_857bce85a931);
}
impl ::windows::core::RuntimeName for WiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiConnectionResult";
}
impl ::core::convert::From<WiFiConnectionResult> for ::windows::core::IUnknown {
    fn from(value: WiFiConnectionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiConnectionResult> for ::windows::core::IUnknown {
    fn from(value: &WiFiConnectionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiConnectionResult> for ::windows::core::IInspectable {
    fn from(value: WiFiConnectionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiConnectionResult> for ::windows::core::IInspectable {
    fn from(value: &WiFiConnectionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiConnectionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiConnectionResult {}
unsafe impl ::core::marker::Sync for WiFiConnectionResult {}
#[repr(transparent)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AccessRevoked: Self = Self(2i32);
    pub const InvalidCredential: Self = Self(3i32);
    pub const NetworkNotAvailable: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
    pub const UnsupportedAuthenticationProtocol: Self = Self(6i32);
}
impl ::core::marker::Copy for WiFiConnectionStatus {}
impl ::core::clone::Clone for WiFiConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiConnectionStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiConnectionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiConnectionStatus {}
unsafe impl ::windows::core::RuntimeType for WiFiConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiConnectionStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: Self = Self(0i32);
    pub const Infrastructure: Self = Self(1i32);
    pub const Adhoc: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiNetworkKind {}
impl ::core::clone::Clone for WiFiNetworkKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiNetworkKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiNetworkKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiNetworkKind {}
unsafe impl ::windows::core::RuntimeType for WiFiNetworkKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiNetworkKind;i4)");
}
impl ::windows::core::DefaultType for WiFiNetworkKind {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiNetworkReport(::windows::core::IUnknown);
impl WiFiNetworkReport {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableNetworks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiNetworkReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiNetworkReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiNetworkReport {}
unsafe impl ::windows::core::RuntimeType for WiFiNetworkReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiNetworkReport;{9524ded2-5911-445e-8194-be4f1a704895})");
}
unsafe impl ::windows::core::Interface for WiFiNetworkReport {
    type Vtable = IWiFiNetworkReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9524ded2_5911_445e_8194_be4f1a704895);
}
impl ::windows::core::RuntimeName for WiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiNetworkReport";
}
impl ::core::convert::From<WiFiNetworkReport> for ::windows::core::IUnknown {
    fn from(value: WiFiNetworkReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiNetworkReport> for ::windows::core::IUnknown {
    fn from(value: &WiFiNetworkReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiNetworkReport> for ::windows::core::IInspectable {
    fn from(value: WiFiNetworkReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiNetworkReport> for ::windows::core::IInspectable {
    fn from(value: &WiFiNetworkReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiNetworkReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiNetworkReport {}
unsafe impl ::core::marker::Sync for WiFiNetworkReport {}
#[repr(transparent)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: Self = Self(0i32);
    pub const Fhss: Self = Self(1i32);
    pub const Dsss: Self = Self(2i32);
    pub const IRBaseband: Self = Self(3i32);
    pub const Ofdm: Self = Self(4i32);
    pub const Hrdsss: Self = Self(5i32);
    pub const Erp: Self = Self(6i32);
    pub const HT: Self = Self(7i32);
    pub const Vht: Self = Self(8i32);
    pub const Dmg: Self = Self(9i32);
    pub const HE: Self = Self(10i32);
}
impl ::core::marker::Copy for WiFiPhyKind {}
impl ::core::clone::Clone for WiFiPhyKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiPhyKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiPhyKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiPhyKind {}
unsafe impl ::windows::core::RuntimeType for WiFiPhyKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiPhyKind;i4)");
}
impl ::windows::core::DefaultType for WiFiPhyKind {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiReconnectionKind {}
impl ::core::clone::Clone for WiFiReconnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiReconnectionKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiReconnectionKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiReconnectionKind {}
unsafe impl ::windows::core::RuntimeType for WiFiReconnectionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiReconnectionKind;i4)");
}
impl ::windows::core::DefaultType for WiFiReconnectionKind {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiWpsConfigurationResult(::windows::core::IUnknown);
impl WiFiWpsConfigurationResult {
    pub fn Status(&self) -> ::windows::core::Result<WiFiWpsConfigurationStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiWpsConfigurationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiWpsConfigurationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedWpsKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiWpsConfigurationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiWpsConfigurationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiWpsConfigurationResult {}
unsafe impl ::windows::core::RuntimeType for WiFiWpsConfigurationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiWpsConfigurationResult;{67b49871-17ee-42d1-b14f-5a11f1226fb5})");
}
unsafe impl ::windows::core::Interface for WiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b49871_17ee_42d1_b14f_5a11f1226fb5);
}
impl ::windows::core::RuntimeName for WiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiWpsConfigurationResult";
}
impl ::core::convert::From<WiFiWpsConfigurationResult> for ::windows::core::IUnknown {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiWpsConfigurationResult> for ::windows::core::IUnknown {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiWpsConfigurationResult> for ::windows::core::IInspectable {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiWpsConfigurationResult> for ::windows::core::IInspectable {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiWpsConfigurationResult {}
unsafe impl ::core::marker::Sync for WiFiWpsConfigurationResult {}
#[repr(transparent)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Timeout: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiWpsConfigurationStatus {}
impl ::core::clone::Clone for WiFiWpsConfigurationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiWpsConfigurationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiWpsConfigurationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiWpsConfigurationStatus {}
unsafe impl ::windows::core::RuntimeType for WiFiWpsConfigurationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsConfigurationStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiWpsConfigurationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Ethernet: Self = Self(4i32);
    pub const Usb: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiWpsKind {}
impl ::core::clone::Clone for WiFiWpsKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiWpsKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiWpsKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiWpsKind {}
unsafe impl ::windows::core::RuntimeType for WiFiWpsKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsKind;i4)");
}
impl ::windows::core::DefaultType for WiFiWpsKind {
    type DefaultType = Self;
}
