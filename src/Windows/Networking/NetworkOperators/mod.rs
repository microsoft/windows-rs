#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: DataClasses = DataClasses(0u32);
    pub const Gprs: DataClasses = DataClasses(1u32);
    pub const Edge: DataClasses = DataClasses(2u32);
    pub const Umts: DataClasses = DataClasses(4u32);
    pub const Hsdpa: DataClasses = DataClasses(8u32);
    pub const Hsupa: DataClasses = DataClasses(16u32);
    pub const LteAdvanced: DataClasses = DataClasses(32u32);
    pub const NewRadioNonStandalone: DataClasses = DataClasses(64u32);
    pub const NewRadioStandalone: DataClasses = DataClasses(128u32);
    pub const Cdma1xRtt: DataClasses = DataClasses(65536u32);
    pub const Cdma1xEvdo: DataClasses = DataClasses(131072u32);
    pub const Cdma1xEvdoRevA: DataClasses = DataClasses(262144u32);
    pub const Cdma1xEvdv: DataClasses = DataClasses(524288u32);
    pub const Cdma3xRtt: DataClasses = DataClasses(1048576u32);
    pub const Cdma1xEvdoRevB: DataClasses = DataClasses(2097152u32);
    pub const CdmaUmb: DataClasses = DataClasses(4194304u32);
    pub const Custom: DataClasses = DataClasses(2147483648u32);
}
impl ::std::convert::From<u32> for DataClasses {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DataClasses {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DataClasses {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.DataClasses;u4)");
}
impl ::std::ops::BitOr for DataClasses {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DataClasses {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DataClasses {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DataClasses {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DataClasses {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESim(::windows::runtime::IInspectable);
impl ESim {
    #[cfg(feature = "Foundation")]
    pub fn AvailableMemoryInBytes(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn Eid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn FirmwareVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn MobileBroadbandModemDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Policy(&self) -> ::windows::runtime::Result<ESimPolicy> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimPolicy>(result__)
        }
    }
    pub fn State(&self) -> ::windows::runtime::Result<ESimState> {
        let this = self;
        unsafe {
            let mut result__: ESimState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimState>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProfiles(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ESimProfile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteProfileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, profileid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), profileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DownloadProfileMetadataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activationcode: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), activationcode.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResetAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProfileChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESim, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProfileChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Discover(&self) -> ::windows::runtime::Result<ESimDiscoverResult> {
        let this = &::windows::runtime::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimDiscoverResult>(result__)
        }
    }
    pub fn DiscoverWithServerAddressAndMatchingId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, serveraddress: Param0, matchingid: Param1) -> ::windows::runtime::Result<ESimDiscoverResult> {
        let this = &::windows::runtime::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), serveraddress.into_param().abi(), matchingid.into_param().abi(), &mut result__).from_abi::<ESimDiscoverResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DiscoverAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &::windows::runtime::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DiscoverWithServerAddressAndMatchingIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, serveraddress: Param0, matchingid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &::windows::runtime::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), serveraddress.into_param().abi(), matchingid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESim {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESim;{6f6e6e26-f123-437d-8ced-dc1d2bc0c3a9})");
}
unsafe impl ::windows::runtime::Interface for ESim {
    type Vtable = IESim_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1869508134, 61731, 17277, [140, 237, 220, 29, 43, 192, 195, 169]);
}
impl ::windows::runtime::RuntimeName for ESim {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESim";
}
impl ::std::convert::From<ESim> for ::windows::runtime::IUnknown {
    fn from(value: ESim) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESim> for ::windows::runtime::IUnknown {
    fn from(value: &ESim) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESim {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESim {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESim> for ::windows::runtime::IInspectable {
    fn from(value: ESim) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESim> for ::windows::runtime::IInspectable {
    fn from(value: &ESim) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESim {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESim {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESim {}
unsafe impl ::std::marker::Sync for ESim {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimAddedEventArgs(::windows::runtime::IInspectable);
impl ESimAddedEventArgs {
    pub fn ESim(&self) -> ::windows::runtime::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESim>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimAddedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimAddedEventArgs;{38bd0a58-4d5a-4d08-8da7-e73eff369ddd})");
}
unsafe impl ::windows::runtime::Interface for ESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(951913048, 19802, 19720, [141, 167, 231, 62, 255, 54, 157, 221]);
}
impl ::windows::runtime::RuntimeName for ESimAddedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimAddedEventArgs";
}
impl ::std::convert::From<ESimAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ESimAddedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ESimAddedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ESimAddedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ESimAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimAddedEventArgs {}
unsafe impl ::std::marker::Sync for ESimAddedEventArgs {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: ESimAuthenticationPreference = ESimAuthenticationPreference(0i32);
    pub const OnAction: ESimAuthenticationPreference = ESimAuthenticationPreference(1i32);
    pub const Never: ESimAuthenticationPreference = ESimAuthenticationPreference(2i32);
}
impl ::std::convert::From<i32> for ESimAuthenticationPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimAuthenticationPreference {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimAuthenticationPreference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimAuthenticationPreference;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimDiscoverEvent(::windows::runtime::IInspectable);
impl ESimDiscoverEvent {
    pub fn MatchingId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RspServerAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimDiscoverEvent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimDiscoverEvent;{e59ac3e3-39bc-5f6f-9321-0d4a182d261b})");
}
unsafe impl ::windows::runtime::Interface for ESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3852125155, 14780, 24431, [147, 33, 13, 74, 24, 45, 38, 27]);
}
impl ::windows::runtime::RuntimeName for ESimDiscoverEvent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverEvent";
}
impl ::std::convert::From<ESimDiscoverEvent> for ::windows::runtime::IUnknown {
    fn from(value: ESimDiscoverEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimDiscoverEvent> for ::windows::runtime::IUnknown {
    fn from(value: &ESimDiscoverEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimDiscoverEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimDiscoverEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimDiscoverEvent> for ::windows::runtime::IInspectable {
    fn from(value: ESimDiscoverEvent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimDiscoverEvent> for ::windows::runtime::IInspectable {
    fn from(value: &ESimDiscoverEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimDiscoverEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimDiscoverEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimDiscoverEvent {}
unsafe impl ::std::marker::Sync for ESimDiscoverEvent {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimDiscoverResult(::windows::runtime::IInspectable);
impl ESimDiscoverResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Events(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<ESimDiscoverResultKind> {
        let this = self;
        unsafe {
            let mut result__: ESimDiscoverResultKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimDiscoverResultKind>(result__)
        }
    }
    pub fn ProfileMetadata(&self) -> ::windows::runtime::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileMetadata>(result__)
        }
    }
    pub fn Result(&self) -> ::windows::runtime::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimOperationResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimDiscoverResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimDiscoverResult;{56b4bb5e-ab2f-5ac6-b359-dd5a8e237926})");
}
unsafe impl ::windows::runtime::Interface for ESimDiscoverResult {
    type Vtable = IESimDiscoverResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1454685022, 43823, 23238, [179, 89, 221, 90, 142, 35, 121, 38]);
}
impl ::windows::runtime::RuntimeName for ESimDiscoverResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverResult";
}
impl ::std::convert::From<ESimDiscoverResult> for ::windows::runtime::IUnknown {
    fn from(value: ESimDiscoverResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimDiscoverResult> for ::windows::runtime::IUnknown {
    fn from(value: &ESimDiscoverResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimDiscoverResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimDiscoverResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimDiscoverResult> for ::windows::runtime::IInspectable {
    fn from(value: ESimDiscoverResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimDiscoverResult> for ::windows::runtime::IInspectable {
    fn from(value: &ESimDiscoverResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimDiscoverResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimDiscoverResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimDiscoverResult {}
unsafe impl ::std::marker::Sync for ESimDiscoverResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: ESimDiscoverResultKind = ESimDiscoverResultKind(0i32);
    pub const Events: ESimDiscoverResultKind = ESimDiscoverResultKind(1i32);
    pub const ProfileMetadata: ESimDiscoverResultKind = ESimDiscoverResultKind(2i32);
}
impl ::std::convert::From<i32> for ESimDiscoverResultKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimDiscoverResultKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimDiscoverResultKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimDiscoverResultKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimDownloadProfileMetadataResult(::windows::runtime::IInspectable);
impl ESimDownloadProfileMetadataResult {
    pub fn Result(&self) -> ::windows::runtime::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimOperationResult>(result__)
        }
    }
    pub fn ProfileMetadata(&self) -> ::windows::runtime::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileMetadata>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimDownloadProfileMetadataResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimDownloadProfileMetadataResult;{c4234d9e-5ad6-426d-8d00-4434f449afec})");
}
unsafe impl ::windows::runtime::Interface for ESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3290647966, 23254, 17005, [141, 0, 68, 52, 244, 73, 175, 236]);
}
impl ::windows::runtime::RuntimeName for ESimDownloadProfileMetadataResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDownloadProfileMetadataResult";
}
impl ::std::convert::From<ESimDownloadProfileMetadataResult> for ::windows::runtime::IUnknown {
    fn from(value: ESimDownloadProfileMetadataResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimDownloadProfileMetadataResult> for ::windows::runtime::IUnknown {
    fn from(value: &ESimDownloadProfileMetadataResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimDownloadProfileMetadataResult> for ::windows::runtime::IInspectable {
    fn from(value: ESimDownloadProfileMetadataResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimDownloadProfileMetadataResult> for ::windows::runtime::IInspectable {
    fn from(value: &ESimDownloadProfileMetadataResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimDownloadProfileMetadataResult {}
unsafe impl ::std::marker::Sync for ESimDownloadProfileMetadataResult {}
pub struct ESimManager {}
impl ESimManager {
    pub fn ServiceInfo() -> ::windows::runtime::Result<ESimServiceInfo> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimServiceInfo>(result__)
        })
    }
    pub fn TryCreateESimWatcher() -> ::windows::runtime::Result<ESimWatcher> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ServiceInfoChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceInfoChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IESimManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IESimManagerStatics<R, F: FnOnce(&IESimManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ESimManager, IESimManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ESimManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimManager";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimOperationResult(::windows::runtime::IInspectable);
impl ESimOperationResult {
    pub fn Status(&self) -> ::windows::runtime::Result<ESimOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: ESimOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimOperationStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimOperationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimOperationResult;{a67b63b1-309b-4e77-9e7e-cd93f1ddc7b9})");
}
unsafe impl ::windows::runtime::Interface for ESimOperationResult {
    type Vtable = IESimOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2793104305, 12443, 20087, [158, 126, 205, 147, 241, 221, 199, 185]);
}
impl ::windows::runtime::RuntimeName for ESimOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimOperationResult";
}
impl ::std::convert::From<ESimOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: ESimOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &ESimOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: ESimOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: &ESimOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimOperationResult {}
unsafe impl ::std::marker::Sync for ESimOperationResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: ESimOperationStatus = ESimOperationStatus(0i32);
    pub const NotAuthorized: ESimOperationStatus = ESimOperationStatus(1i32);
    pub const NotFound: ESimOperationStatus = ESimOperationStatus(2i32);
    pub const PolicyViolation: ESimOperationStatus = ESimOperationStatus(3i32);
    pub const InsufficientSpaceOnCard: ESimOperationStatus = ESimOperationStatus(4i32);
    pub const ServerFailure: ESimOperationStatus = ESimOperationStatus(5i32);
    pub const ServerNotReachable: ESimOperationStatus = ESimOperationStatus(6i32);
    pub const TimeoutWaitingForUserConsent: ESimOperationStatus = ESimOperationStatus(7i32);
    pub const IncorrectConfirmationCode: ESimOperationStatus = ESimOperationStatus(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: ESimOperationStatus = ESimOperationStatus(9i32);
    pub const CardRemoved: ESimOperationStatus = ESimOperationStatus(10i32);
    pub const CardBusy: ESimOperationStatus = ESimOperationStatus(11i32);
    pub const Other: ESimOperationStatus = ESimOperationStatus(12i32);
    pub const CardGeneralFailure: ESimOperationStatus = ESimOperationStatus(13i32);
    pub const ConfirmationCodeMissing: ESimOperationStatus = ESimOperationStatus(14i32);
    pub const InvalidMatchingId: ESimOperationStatus = ESimOperationStatus(15i32);
    pub const NoEligibleProfileForThisDevice: ESimOperationStatus = ESimOperationStatus(16i32);
    pub const OperationAborted: ESimOperationStatus = ESimOperationStatus(17i32);
    pub const EidMismatch: ESimOperationStatus = ESimOperationStatus(18i32);
    pub const ProfileNotAvailableForNewBinding: ESimOperationStatus = ESimOperationStatus(19i32);
    pub const ProfileNotReleasedByOperator: ESimOperationStatus = ESimOperationStatus(20i32);
    pub const OperationProhibitedByProfileClass: ESimOperationStatus = ESimOperationStatus(21i32);
    pub const ProfileNotPresent: ESimOperationStatus = ESimOperationStatus(22i32);
    pub const NoCorrespondingRequest: ESimOperationStatus = ESimOperationStatus(23i32);
    pub const TimeoutWaitingForResponse: ESimOperationStatus = ESimOperationStatus(24i32);
    pub const IccidAlreadyExists: ESimOperationStatus = ESimOperationStatus(25i32);
    pub const ProfileProcessingError: ESimOperationStatus = ESimOperationStatus(26i32);
    pub const ServerNotTrusted: ESimOperationStatus = ESimOperationStatus(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: ESimOperationStatus = ESimOperationStatus(28i32);
}
impl ::std::convert::From<i32> for ESimOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimOperationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimPolicy(::windows::runtime::IInspectable);
impl ESimPolicy {
    pub fn ShouldEnableManagingUi(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimPolicy;{41e1b99d-cf7e-4315-882b-6f1e74b0d38f})");
}
unsafe impl ::windows::runtime::Interface for ESimPolicy {
    type Vtable = IESimPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1105312157, 53118, 17173, [136, 43, 111, 30, 116, 176, 211, 143]);
}
impl ::windows::runtime::RuntimeName for ESimPolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimPolicy";
}
impl ::std::convert::From<ESimPolicy> for ::windows::runtime::IUnknown {
    fn from(value: ESimPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimPolicy> for ::windows::runtime::IUnknown {
    fn from(value: &ESimPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimPolicy> for ::windows::runtime::IInspectable {
    fn from(value: ESimPolicy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimPolicy> for ::windows::runtime::IInspectable {
    fn from(value: &ESimPolicy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimPolicy {}
unsafe impl ::std::marker::Sync for ESimPolicy {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimProfile(::windows::runtime::IInspectable);
impl ESimProfile {
    pub fn Class(&self) -> ::windows::runtime::Result<ESimProfileClass> {
        let this = self;
        unsafe {
            let mut result__: ESimProfileClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileClass>(result__)
        }
    }
    pub fn Nickname(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Policy(&self) -> ::windows::runtime::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfilePolicy>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ProviderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::runtime::Result<ESimProfileState> {
        let this = self;
        unsafe {
            let mut result__: ESimProfileState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetNicknameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, newnickname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), newnickname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimProfile;{ee1e7880-06a9-4027-b4f8-ddb23d7810e0})");
}
unsafe impl ::windows::runtime::Interface for ESimProfile {
    type Vtable = IESimProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3994974336, 1705, 16423, [180, 248, 221, 178, 61, 120, 16, 224]);
}
impl ::windows::runtime::RuntimeName for ESimProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfile";
}
impl ::std::convert::From<ESimProfile> for ::windows::runtime::IUnknown {
    fn from(value: ESimProfile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimProfile> for ::windows::runtime::IUnknown {
    fn from(value: &ESimProfile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimProfile> for ::windows::runtime::IInspectable {
    fn from(value: ESimProfile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimProfile> for ::windows::runtime::IInspectable {
    fn from(value: &ESimProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimProfile {}
unsafe impl ::std::marker::Sync for ESimProfile {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: ESimProfileClass = ESimProfileClass(0i32);
    pub const Test: ESimProfileClass = ESimProfileClass(1i32);
    pub const Provisioning: ESimProfileClass = ESimProfileClass(2i32);
}
impl ::std::convert::From<i32> for ESimProfileClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimProfileClass {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfileClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileClass;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ESimProfileInstallProgress {
    pub TotalSizeInBytes: i32,
    pub InstalledSizeInBytes: i32,
}
impl ESimProfileInstallProgress {}
impl ::std::default::Default for ESimProfileInstallProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ESimProfileInstallProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ESimProfileInstallProgress").field("TotalSizeInBytes", &self.TotalSizeInBytes).field("InstalledSizeInBytes", &self.InstalledSizeInBytes).finish()
    }
}
impl ::std::cmp::PartialEq for ESimProfileInstallProgress {
    fn eq(&self, other: &Self) -> bool {
        self.TotalSizeInBytes == other.TotalSizeInBytes && self.InstalledSizeInBytes == other.InstalledSizeInBytes
    }
}
impl ::std::cmp::Eq for ESimProfileInstallProgress {}
unsafe impl ::windows::runtime::Abi for ESimProfileInstallProgress {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfileInstallProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ESimProfileInstallProgress;i4;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimProfileMetadata(::windows::runtime::IInspectable);
impl ESimProfileMetadata {
    pub fn IsConfirmationCodeRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Policy(&self) -> ::windows::runtime::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfilePolicy>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ProviderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::runtime::Result<ESimProfileMetadataState> {
        let this = self;
        unsafe {
            let mut result__: ESimProfileMetadataState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileMetadataState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DenyInstallAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConfirmInstallAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConfirmInstallWithConfirmationCodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, confirmationcode: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), confirmationcode.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PostponeInstallAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfileMetadata {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimProfileMetadata;{ed25831f-90db-498d-a7b4-ebce807d3c23})");
}
unsafe impl ::windows::runtime::Interface for ESimProfileMetadata {
    type Vtable = IESimProfileMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3978658591, 37083, 18829, [167, 180, 235, 206, 128, 125, 60, 35]);
}
impl ::windows::runtime::RuntimeName for ESimProfileMetadata {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfileMetadata";
}
impl ::std::convert::From<ESimProfileMetadata> for ::windows::runtime::IUnknown {
    fn from(value: ESimProfileMetadata) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimProfileMetadata> for ::windows::runtime::IUnknown {
    fn from(value: &ESimProfileMetadata) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimProfileMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimProfileMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimProfileMetadata> for ::windows::runtime::IInspectable {
    fn from(value: ESimProfileMetadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimProfileMetadata> for ::windows::runtime::IInspectable {
    fn from(value: &ESimProfileMetadata) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimProfileMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimProfileMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimProfileMetadata {}
unsafe impl ::std::marker::Sync for ESimProfileMetadata {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: ESimProfileMetadataState = ESimProfileMetadataState(0i32);
    pub const WaitingForInstall: ESimProfileMetadataState = ESimProfileMetadataState(1i32);
    pub const Downloading: ESimProfileMetadataState = ESimProfileMetadataState(2i32);
    pub const Installing: ESimProfileMetadataState = ESimProfileMetadataState(3i32);
    pub const Expired: ESimProfileMetadataState = ESimProfileMetadataState(4i32);
    pub const RejectingDownload: ESimProfileMetadataState = ESimProfileMetadataState(5i32);
    pub const NoLongerAvailable: ESimProfileMetadataState = ESimProfileMetadataState(6i32);
    pub const DeniedByPolicy: ESimProfileMetadataState = ESimProfileMetadataState(7i32);
}
impl ::std::convert::From<i32> for ESimProfileMetadataState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimProfileMetadataState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfileMetadataState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileMetadataState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimProfilePolicy(::windows::runtime::IInspectable);
impl ESimProfilePolicy {
    pub fn CanDelete(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanDisable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsManagedByEnterprise(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfilePolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimProfilePolicy;{e6dd0f1d-9c5c-46c5-a289-a948999bf062})");
}
unsafe impl ::windows::runtime::Interface for ESimProfilePolicy {
    type Vtable = IESimProfilePolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3873247005, 40028, 18117, [162, 137, 169, 72, 153, 155, 240, 98]);
}
impl ::windows::runtime::RuntimeName for ESimProfilePolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfilePolicy";
}
impl ::std::convert::From<ESimProfilePolicy> for ::windows::runtime::IUnknown {
    fn from(value: ESimProfilePolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimProfilePolicy> for ::windows::runtime::IUnknown {
    fn from(value: &ESimProfilePolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimProfilePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimProfilePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimProfilePolicy> for ::windows::runtime::IInspectable {
    fn from(value: ESimProfilePolicy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimProfilePolicy> for ::windows::runtime::IInspectable {
    fn from(value: &ESimProfilePolicy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimProfilePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimProfilePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimProfilePolicy {}
unsafe impl ::std::marker::Sync for ESimProfilePolicy {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: ESimProfileState = ESimProfileState(0i32);
    pub const Disabled: ESimProfileState = ESimProfileState(1i32);
    pub const Enabled: ESimProfileState = ESimProfileState(2i32);
    pub const Deleted: ESimProfileState = ESimProfileState(3i32);
}
impl ::std::convert::From<i32> for ESimProfileState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimProfileState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimProfileState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimRemovedEventArgs(::windows::runtime::IInspectable);
impl ESimRemovedEventArgs {
    pub fn ESim(&self) -> ::windows::runtime::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESim>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimRemovedEventArgs;{dec5277b-2fd9-4ed9-8376-d9b5e41278a3})");
}
unsafe impl ::windows::runtime::Interface for ESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3737462651, 12249, 20185, [131, 118, 217, 181, 228, 18, 120, 163]);
}
impl ::windows::runtime::RuntimeName for ESimRemovedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimRemovedEventArgs";
}
impl ::std::convert::From<ESimRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ESimRemovedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ESimRemovedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ESimRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ESimRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimRemovedEventArgs {}
unsafe impl ::std::marker::Sync for ESimRemovedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimServiceInfo(::windows::runtime::IInspectable);
impl ESimServiceInfo {
    pub fn AuthenticationPreference(&self) -> ::windows::runtime::Result<ESimAuthenticationPreference> {
        let this = self;
        unsafe {
            let mut result__: ESimAuthenticationPreference = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimAuthenticationPreference>(result__)
        }
    }
    pub fn IsESimUiEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimServiceInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimServiceInfo;{f16aabcf-7f59-4a51-8494-bd89d5ff50ee})");
}
unsafe impl ::windows::runtime::Interface for ESimServiceInfo {
    type Vtable = IESimServiceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4050299855, 32601, 19025, [132, 148, 189, 137, 213, 255, 80, 238]);
}
impl ::windows::runtime::RuntimeName for ESimServiceInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimServiceInfo";
}
impl ::std::convert::From<ESimServiceInfo> for ::windows::runtime::IUnknown {
    fn from(value: ESimServiceInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimServiceInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ESimServiceInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimServiceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimServiceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimServiceInfo> for ::windows::runtime::IInspectable {
    fn from(value: ESimServiceInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimServiceInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ESimServiceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimServiceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimServiceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimServiceInfo {}
unsafe impl ::std::marker::Sync for ESimServiceInfo {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: ESimState = ESimState(0i32);
    pub const Idle: ESimState = ESimState(1i32);
    pub const Removed: ESimState = ESimState(2i32);
    pub const Busy: ESimState = ESimState(3i32);
}
impl ::std::convert::From<i32> for ESimState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimUpdatedEventArgs(::windows::runtime::IInspectable);
impl ESimUpdatedEventArgs {
    pub fn ESim(&self) -> ::windows::runtime::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESim>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimUpdatedEventArgs;{4c125cec-508d-4b88-83cb-68bef8168d12})");
}
unsafe impl ::windows::runtime::Interface for ESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1276271852, 20621, 19336, [131, 203, 104, 190, 248, 22, 141, 18]);
}
impl ::windows::runtime::RuntimeName for ESimUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimUpdatedEventArgs";
}
impl ::std::convert::From<ESimUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ESimUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ESimUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ESimUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ESimUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for ESimUpdatedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ESimWatcher(::windows::runtime::IInspectable);
impl ESimWatcher {
    pub fn Status(&self) -> ::windows::runtime::Result<ESimWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: ESimWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ESimWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ESimWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimWatcher;{c1f84ceb-a28d-4fbf-9771-6e31b81ccf22})");
}
unsafe impl ::windows::runtime::Interface for ESimWatcher {
    type Vtable = IESimWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3254275307, 41613, 20415, [151, 113, 110, 49, 184, 28, 207, 34]);
}
impl ::windows::runtime::RuntimeName for ESimWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimWatcher";
}
impl ::std::convert::From<ESimWatcher> for ::windows::runtime::IUnknown {
    fn from(value: ESimWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ESimWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &ESimWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ESimWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ESimWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ESimWatcher> for ::windows::runtime::IInspectable {
    fn from(value: ESimWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ESimWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &ESimWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ESimWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ESimWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ESimWatcher {}
unsafe impl ::std::marker::Sync for ESimWatcher {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: ESimWatcherStatus = ESimWatcherStatus(0i32);
    pub const Started: ESimWatcherStatus = ESimWatcherStatus(1i32);
    pub const EnumerationCompleted: ESimWatcherStatus = ESimWatcherStatus(2i32);
    pub const Stopping: ESimWatcherStatus = ESimWatcherStatus(3i32);
    pub const Stopped: ESimWatcherStatus = ESimWatcherStatus(4i32);
}
impl ::std::convert::From<i32> for ESimWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESimWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ESimWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimWatcherStatus;i4)");
}
pub struct FdnAccessManager {}
impl FdnAccessManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestUnlockAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contactlistid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFdnAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contactlistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IFdnAccessManagerStatics<R, F: FnOnce(&IFdnAccessManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FdnAccessManager, IFdnAccessManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for FdnAccessManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.FdnAccessManager";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HotspotAuthenticationContext(::windows::runtime::IInspectable);
impl HotspotAuthenticationContext {
    pub fn WirelessNetworkId(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows::runtime::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Connectivity::NetworkAdapter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RedirectMessageUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn RedirectMessageXml(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn IssueCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0, password: Param1, extraparameters: Param2, markasmanualconnectonfailure: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), username.into_param().abi(), password.into_param().abi(), extraparameters.into_param().abi(), markasmanualconnectonfailure).ok() }
    }
    pub fn AbortAuthentication(&self, markasmanual: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), markasmanual).ok() }
    }
    pub fn SkipAuthentication(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn TriggerAttentionRequired<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, packagerelativeapplicationid: Param0, applicationparameters: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), packagerelativeapplicationid.into_param().abi(), applicationparameters.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn IssueCredentialsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0, password: Param1, extraparameters: Param2, markasmanualconnectonfailure: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>> {
        let this = &::windows::runtime::Interface::cast::<IHotspotAuthenticationContext2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), username.into_param().abi(), password.into_param().abi(), extraparameters.into_param().abi(), markasmanualconnectonfailure, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>>(result__)
        }
    }
    pub fn TryGetAuthenticationContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(eventoken: Param0, context: &mut ::std::option::Option<HotspotAuthenticationContext>) -> ::windows::runtime::Result<bool> {
        Self::IHotspotAuthenticationContextStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), eventoken.into_param().abi(), context as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHotspotAuthenticationContextStatics<R, F: FnOnce(&IHotspotAuthenticationContextStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HotspotAuthenticationContext, IHotspotAuthenticationContextStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HotspotAuthenticationContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.HotspotAuthenticationContext;{e756c791-1003-4de5-83c7-de61d88831d0})");
}
unsafe impl ::windows::runtime::Interface for HotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4099, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
impl ::windows::runtime::RuntimeName for HotspotAuthenticationContext {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationContext";
}
impl ::std::convert::From<HotspotAuthenticationContext> for ::windows::runtime::IUnknown {
    fn from(value: HotspotAuthenticationContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HotspotAuthenticationContext> for ::windows::runtime::IUnknown {
    fn from(value: &HotspotAuthenticationContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HotspotAuthenticationContext> for ::windows::runtime::IInspectable {
    fn from(value: HotspotAuthenticationContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HotspotAuthenticationContext> for ::windows::runtime::IInspectable {
    fn from(value: &HotspotAuthenticationContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HotspotAuthenticationEventDetails(::windows::runtime::IInspectable);
impl HotspotAuthenticationEventDetails {
    pub fn EventToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HotspotAuthenticationEventDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.HotspotAuthenticationEventDetails;{e756c791-1001-4de5-83c7-de61d88831d0})");
}
unsafe impl ::windows::runtime::Interface for HotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4097, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
impl ::windows::runtime::RuntimeName for HotspotAuthenticationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationEventDetails";
}
impl ::std::convert::From<HotspotAuthenticationEventDetails> for ::windows::runtime::IUnknown {
    fn from(value: HotspotAuthenticationEventDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HotspotAuthenticationEventDetails> for ::windows::runtime::IUnknown {
    fn from(value: &HotspotAuthenticationEventDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HotspotAuthenticationEventDetails> for ::windows::runtime::IInspectable {
    fn from(value: HotspotAuthenticationEventDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HotspotAuthenticationEventDetails> for ::windows::runtime::IInspectable {
    fn from(value: &HotspotAuthenticationEventDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(0i32);
    pub const LoginSucceeded: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(50i32);
    pub const LoginFailed: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(100i32);
    pub const RadiusServerError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(102i32);
    pub const NetworkAdministratorError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(105i32);
    pub const LoginAborted: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(151i32);
    pub const AccessGatewayInternalError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(255i32);
}
impl ::std::convert::From<i32> for HotspotAuthenticationResponseCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HotspotAuthenticationResponseCode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HotspotAuthenticationResponseCode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.HotspotAuthenticationResponseCode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HotspotCredentialsAuthenticationResult(::windows::runtime::IInspectable);
impl HotspotCredentialsAuthenticationResult {
    pub fn HasNetworkErrorOccurred(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ResponseCode(&self) -> ::windows::runtime::Result<HotspotAuthenticationResponseCode> {
        let this = self;
        unsafe {
            let mut result__: HotspotAuthenticationResponseCode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HotspotAuthenticationResponseCode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LogoffUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn AuthenticationReplyXml(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HotspotCredentialsAuthenticationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.HotspotCredentialsAuthenticationResult;{e756c791-1005-4de5-83c7-de61d88831d0})");
}
unsafe impl ::windows::runtime::Interface for HotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4101, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
impl ::windows::runtime::RuntimeName for HotspotCredentialsAuthenticationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotCredentialsAuthenticationResult";
}
impl ::std::convert::From<HotspotCredentialsAuthenticationResult> for ::windows::runtime::IUnknown {
    fn from(value: HotspotCredentialsAuthenticationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HotspotCredentialsAuthenticationResult> for ::windows::runtime::IUnknown {
    fn from(value: &HotspotCredentialsAuthenticationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HotspotCredentialsAuthenticationResult> for ::windows::runtime::IInspectable {
    fn from(value: HotspotCredentialsAuthenticationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HotspotCredentialsAuthenticationResult> for ::windows::runtime::IInspectable {
    fn from(value: &HotspotCredentialsAuthenticationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESim(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESim {
    type Vtable = IESim_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1869508134, 61731, 17277, [140, 237, 220, 29, 43, 192, 195, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activationcode: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESim2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESim2 {
    type Vtable = IESim2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3176124576, 50831, 22251, [185, 155, 143, 52, 184, 16, 2, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serveraddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, matchingid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serveraddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, matchingid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimAddedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(951913048, 19802, 19720, [141, 167, 231, 62, 255, 54, 157, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimAddedEventArgs_abi(
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
pub struct IESimDiscoverEvent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3852125155, 14780, 24431, [147, 33, 13, 74, 24, 45, 38, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDiscoverEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimDiscoverResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimDiscoverResult {
    type Vtable = IESimDiscoverResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1454685022, 43823, 23238, [179, 89, 221, 90, 142, 35, 121, 38]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDiscoverResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimDiscoverResultKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimDownloadProfileMetadataResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3290647966, 23254, 17005, [141, 0, 68, 52, 244, 73, 175, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDownloadProfileMetadataResult_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimManagerStatics {
    type Vtable = IESimManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(200944652, 57224, 17969, [191, 4, 193, 46, 40, 27, 57, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimOperationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimOperationResult {
    type Vtable = IESimOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2793104305, 12443, 20087, [158, 126, 205, 147, 241, 221, 199, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimOperationStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimPolicy(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimPolicy {
    type Vtable = IESimPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1105312157, 53118, 17173, [136, 43, 111, 30, 116, 176, 211, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimPolicy_abi(
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
pub struct IESimProfile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimProfile {
    type Vtable = IESimProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3994974336, 1705, 16423, [180, 248, 221, 178, 61, 120, 16, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimProfileClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimProfileState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newnickname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimProfileMetadata(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimProfileMetadata {
    type Vtable = IESimProfileMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3978658591, 37083, 18829, [167, 180, 235, 206, 128, 125, 60, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfileMetadata_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimProfileMetadataState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confirmationcode: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimProfilePolicy(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimProfilePolicy {
    type Vtable = IESimProfilePolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3873247005, 40028, 18117, [162, 137, 169, 72, 153, 155, 240, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfilePolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimRemovedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3737462651, 12249, 20185, [131, 118, 217, 181, 228, 18, 120, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimRemovedEventArgs_abi(
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
pub struct IESimServiceInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimServiceInfo {
    type Vtable = IESimServiceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4050299855, 32601, 19025, [132, 148, 189, 137, 213, 255, 80, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimServiceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimAuthenticationPreference) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IESimUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1276271852, 20621, 19336, [131, 203, 104, 190, 248, 22, 141, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimUpdatedEventArgs_abi(
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
pub struct IESimWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IESimWatcher {
    type Vtable = IESimWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3254275307, 41613, 20415, [151, 113, 110, 49, 184, 28, 207, 34]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ESimWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IFdnAccessManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFdnAccessManagerStatics {
    type Vtable = IFdnAccessManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4071244693, 61926, 17177, [170, 62, 71, 124, 166, 75, 43, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFdnAccessManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactlistid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4099, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, password: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, extraparameters: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, markasmanualconnectonfailure: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, markasmanual: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagerelativeapplicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, applicationparameters: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHotspotAuthenticationContext2 {
    type Vtable = IHotspotAuthenticationContext2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4100, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, password: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, extraparameters: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, markasmanualconnectonfailure: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContextStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHotspotAuthenticationContextStatics {
    type Vtable = IHotspotAuthenticationContextStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4098, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContextStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventoken: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, context: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHotspotAuthenticationEventDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4097, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationEventDetails_abi(
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
pub struct IHotspotCredentialsAuthenticationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3881224081, 4101, 19941, [131, 199, 222, 97, 216, 136, 49, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotCredentialsAuthenticationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HotspotAuthenticationResponseCode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IKnownCSimFilePathsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownCSimFilePathsStatics {
    type Vtable = IKnownCSimFilePathsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3025710829, 18929, 19490, [176, 115, 150, 213, 17, 191, 156, 53]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownCSimFilePathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IKnownRuimFilePathsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownRuimFilePathsStatics {
    type Vtable = IKnownRuimFilePathsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(948160697, 65316, 17777, [168, 103, 9, 249, 96, 66, 110, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRuimFilePathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IKnownSimFilePathsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownSimFilePathsStatics {
    type Vtable = IKnownSimFilePathsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2160925283, 14245, 17363, [128, 163, 204, 210, 62, 143, 236, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimFilePathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IKnownUSimFilePathsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownUSimFilePathsStatics {
    type Vtable = IKnownUSimFilePathsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2083841409, 7963, 17396, [149, 48, 139, 9, 45, 50, 215, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUSimFilePathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(918703309, 52962, 17376, [166, 3, 238, 134, 163, 109, 101, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccount2 {
    type Vtable = IMobileBroadbandAccount2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(955592476, 4406, 16983, [149, 159, 182, 88, 163, 82, 182, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking_Connectivity")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccount3 {
    type Vtable = IMobileBroadbandAccount3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(153755169, 37753, 19355, [173, 49, 213, 254, 226, 247, 72, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount3_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945014912, 30686, 19460, [190, 173, 161, 35, 176, 140, 159, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountEventArgs_abi(
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
pub struct IMobileBroadbandAccountStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccountStatics {
    type Vtable = IMobileBroadbandAccountStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860469540, 44993, 20424, [174, 154, 169, 23, 83, 16, 250, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2076384648, 42685, 18913, [128, 171, 107, 145, 53, 74, 87, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1811100510, 9141, 17567, [146, 141, 94, 13, 62, 4, 71, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandAccountWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSar(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3115273086, 52217, 16649, [144, 190, 92, 6, 191, 213, 19, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSar_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSarFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandAntennaSarFactory {
    type Vtable = IMobileBroadbandAntennaSarFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2837321494, 49229, 18977, [134, 152, 20, 89, 220, 103, 44, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSarFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, antennaindex: i32, sarbackoffindex: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandCellCdma(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(100774836, 16666, 20270, [130, 135, 118, 245, 101, 12, 96, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellCdma_abi(
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
pub struct IMobileBroadbandCellGsm(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3432087302, 32480, 18360, [158, 31, 195, 180, 141, 249, 223, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellGsm_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandCellLte(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2442643579, 11128, 17773, [139, 83, 170, 162, 93, 10, 247, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellLte_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
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
pub struct IMobileBroadbandCellNR(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2705264107, 26364, 19275, [131, 169, 164, 135, 163, 165, 160, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellNR_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
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
pub struct IMobileBroadbandCellTdscdma(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(249173589, 56078, 16770, [140, 218, 204, 65, 154, 123, 222, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellTdscdma_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandCellUmts(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2008331694, 18888, 20245, [178, 133, 76, 38, 167, 246, 114, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellUmts_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2309576234, 58482, 19877, [146, 156, 222, 97, 113, 29, 210, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCellsInfo2 {
    type Vtable = IMobileBroadbandCellsInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1713395986, 47263, 19986, [187, 182, 213, 207, 9, 168, 32, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4145590660, 50032, 24532, [166, 112, 24, 70, 203, 155, 206, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872424296, 58241, 19566, [155, 232, 254, 21, 105, 105, 164, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkDeviceStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Sms")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Sms::CellularClass) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DataClasses) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandDeviceType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandRadioState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceInformation2 {
    type Vtable = IMobileBroadbandDeviceInformation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(776370929, 63794, 18231, [167, 34, 3, 186, 114, 55, 12, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceInformation3 {
    type Vtable = IMobileBroadbandDeviceInformation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3767252157, 23856, 19290, [146, 204, 213, 77, 248, 129, 212, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceInformation4 {
    type Vtable = IMobileBroadbandDeviceInformation4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(641675602, 31645, 22572, [177, 124, 248, 10, 96, 181, 0, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation4_abi(
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
pub struct IMobileBroadbandDeviceService(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582883922, 48512, 16556, [142, 31, 46, 7, 131, 106, 61, 189]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2968808123, 38102, 17593, [165, 56, 240, 129, 11, 100, 83, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4228483653, 37179, 18708, [182, 195, 174, 99, 4, 89, 62, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u32, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3064599518, 4992, 16611, [134, 24, 115, 203, 202, 72, 19, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3671466803, 35791, 17033, [138, 55, 4, 92, 33, 105, 72, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1406573403, 50413, 17904, [128, 58, 217, 65, 122, 109, 152, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1241865072, 47534, 17496, [146, 65, 166, 165, 251, 241, 138, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3493161234, 59897, 20327, [160, 61, 67, 24, 154, 49, 107, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceserviceid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModem2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModem2 {
    type Vtable = IMobileBroadbandModem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(310782760, 47595, 20194, [187, 227, 113, 31, 83, 238, 163, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModem3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModem3 {
    type Vtable = IMobileBroadbandModem3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3925788394, 12084, 17794, [145, 2, 195, 20, 210, 168, 126, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4242552227, 54989, 17184, [185, 130, 190, 157, 62, 199, 137, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModemConfiguration2 {
    type Vtable = IMobileBroadbandModemConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(839906757, 58464, 17070, [170, 81, 105, 98, 30, 122, 68, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration2_abi(
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
pub struct IMobileBroadbandModemIsolation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3043069932, 58977, 17200, [155, 180, 52, 128, 33, 46, 195, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, host: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, first: ::windows::runtime::RawPtr, last: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModemIsolationFactory {
    type Vtable = IMobileBroadbandModemIsolationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(567798872, 49841, 19503, [160, 48, 114, 130, 10, 36, 236, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modemdeviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, rulegroupid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandModemStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandModemStatics {
    type Vtable = IMobileBroadbandModemStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4187936311, 55025, 19064, [140, 188, 100, 33, 166, 80, 99, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3412300428, 777, 19638, [168, 193, 106, 90, 60, 142, 31, 246]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkRegistrationState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DataClasses) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandNetwork2 {
    type Vtable = IMobileBroadbandNetwork2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1515576098, 25335, 19421, [186, 29, 71, 116, 65, 150, 11, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandNetwork3 {
    type Vtable = IMobileBroadbandNetwork3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(862390922, 51183, 17484, [171, 108, 223, 126, 247, 163, 144, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork3_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3199177953, 38415, 18868, [160, 141, 125, 133, 233, 104, 199, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2299747583, 10424, 18090, [177, 55, 28, 75, 15, 33, 237, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandPco(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3571776702, 58275, 17349, [168, 123, 108, 134, 210, 41, 215, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPco_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(641683732, 25824, 17555, [144, 155, 45, 20, 160, 25, 98, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails_abi(
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
pub struct IMobileBroadbandPin(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3865171721, 59257, 17855, [130, 129, 117, 50, 61, 249, 227, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandPinType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandPinLockState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandPinFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newpin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinunblockkey: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newpin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189139262, 7940, 20373, [139, 144, 231, 245, 89, 221, 231, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandPinType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandPinLockState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3543711889, 16017, 19768, [144, 54, 174, 232, 58, 110, 121, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandPinManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2203483869, 28191, 19355, [164, 19, 43, 31, 80, 204, 54, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pintype: MobileBroadbandPinType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandPinOperationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(299752498, 12775, 18933, [182, 99, 18, 61, 59, 239, 3, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2958337377, 38963, 19181, [151, 23, 67, 72, 178, 26, 36, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandRadioState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1898977998, 2364, 17094, [176, 219, 173, 31, 117, 166, 84, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandSarManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3853674547, 38526, 16585, [164, 133, 25, 192, 221, 32, 158, 34]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSarManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, antennas: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timerperiod: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3174370098, 34862, 21546, [177, 125, 11, 177, 180, 155, 174, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandSlotState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(827884447, 38156, 21710, [164, 141, 186, 69, 41, 180, 143, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs_abi(
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
pub struct IMobileBroadbandSlotManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3953163478, 8217, 24449, [162, 148, 204, 54, 74, 17, 208, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, slotindex: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1630419061, 1034, 20377, [164, 249, 97, 215, 195, 45, 161, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs_abi(
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
pub struct IMobileBroadbandUicc(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3862230673, 21082, 19682, [143, 206, 170, 65, 98, 87, 145, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUicc_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccApp(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1293354326, 39073, 17373, [178, 236, 80, 201, 12, 242, 72, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccApp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UiccAppKind) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiccfilepath: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiccfilepath: ::windows::runtime::RawPtr, recordindex: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppReadRecordResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1690915461, 13710, 18373, [130, 73, 105, 95, 56, 59, 43, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppReadRecordResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3642320943, 48660, 18740, [152, 29, 47, 87, 185, 237, 131, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UiccAppRecordKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UiccAccessCondition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UiccAccessCondition) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppsResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950953707, 33111, 19009, [132, 148, 107, 245, 76, 155, 29, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorDataUsageTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1357058669, 42085, 20203, [147, 23, 40, 161, 103, 99, 12, 234]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorDataUsageTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkOperatorDataUsageNotificationKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationEventDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3160975825, 33505, 17544, [159, 44, 18, 118, 194, 70, 143, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationEventDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NetworkOperatorEventMessageType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Sms")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(197919364, 16686, 16445, [172, 198, 183, 87, 227, 71, 116, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration_abi(
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
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringAccessPointConfiguration2 {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2977993026, 29240, 22944, [146, 139, 116, 171, 70, 253, 100, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, band: TetheringWiFiBand, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, band: TetheringWiFiBand, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TetheringWiFiBand) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: TetheringWiFiBand) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClient(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1889346892, 22879, 18503, [187, 48, 100, 105, 53, 84, 41, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClientManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringClientManager {
    type Vtable = INetworkOperatorTetheringClientManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2444312598, 36298, 16933, [187, 237, 238, 248, 184, 215, 24, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClientManager_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringEntitlementCheck(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringEntitlementCheck {
    type Vtable = INetworkOperatorTetheringEntitlementCheck_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(17338733, 40602, 19190, [141, 163, 96, 73, 59, 25, 194, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringEntitlementCheck_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: bool, entitlementfailurereason: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3562704288, 3718, 19864, [139, 164, 221, 112, 212, 183, 100, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TetheringOperationalState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringManagerStatics {
    type Vtable = INetworkOperatorTetheringManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1052555980, 63683, 16476, [153, 100, 112, 161, 238, 171, 225, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut TetheringCapability) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringManagerStatics2 {
    type Vtable = INetworkOperatorTetheringManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1529041938, 13808, 18919, [155, 8, 22, 210, 120, 251, 170, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut TetheringCapability) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringManagerStatics3 {
    type Vtable = INetworkOperatorTetheringManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2413473206, 19193, 20257, [155, 88, 213, 62, 159, 36, 35, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, adapter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringManagerStatics4 {
    type Vtable = INetworkOperatorTetheringManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3015309776, 60415, 18084, [168, 71, 214, 99, 216, 176, 151, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringOperationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3956409249, 442, 18285, [180, 179, 191, 61, 18, 200, 248, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TetheringOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProvisionFromXmlDocumentResults(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33283, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisionFromXmlDocumentResults_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProvisionedProfile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProvisionedProfile {
    type Vtable = IProvisionedProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33282, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisionedProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Connectivity::NetworkCostType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ProfileUsage) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProvisioningAgent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProvisioningAgent {
    type Vtable = IProvisioningAgent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33281, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningAgent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provisioningxmldocument: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediatype: ProfileMediaType, profilename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProvisioningAgentStaticMethods(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProvisioningAgentStaticMethods {
    type Vtable = IProvisioningAgentStaticMethods_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33025, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningAgentStaticMethods_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITetheringEntitlementCheckTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(63331997, 22822, 16883, [169, 78, 181, 9, 38, 252, 66, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITetheringEntitlementCheckTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, entitlementfailurereason: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUssdMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUssdMessage {
    type Vtable = IUssdMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 8196, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUssdMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUssdMessageFactory {
    type Vtable = IUssdMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 4099, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUssdReply(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUssdReply {
    type Vtable = IUssdReply_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 8197, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdReply_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UssdResultCode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUssdSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUssdSession {
    type Vtable = IUssdSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 8194, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUssdSessionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUssdSessionStatics {
    type Vtable = IUssdSessionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 4097, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkinterfaceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
pub struct KnownCSimFilePaths {}
impl KnownCSimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    pub fn IKnownCSimFilePathsStatics<R, F: FnOnce(&IKnownCSimFilePathsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownCSimFilePaths, IKnownCSimFilePathsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownCSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownCSimFilePaths";
}
pub struct KnownRuimFilePaths {}
impl KnownRuimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    pub fn IKnownRuimFilePathsStatics<R, F: FnOnce(&IKnownRuimFilePathsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownRuimFilePaths, IKnownRuimFilePathsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownRuimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownRuimFilePaths";
}
pub struct KnownSimFilePaths {}
impl KnownSimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOns() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    pub fn IKnownSimFilePathsStatics<R, F: FnOnce(&IKnownSimFilePathsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownSimFilePaths, IKnownSimFilePathsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownSimFilePaths";
}
pub struct KnownUSimFilePaths {}
impl KnownUSimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOpl() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFPnn() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    pub fn IKnownUSimFilePathsStatics<R, F: FnOnce(&IKnownUSimFilePathsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownUSimFilePaths, IKnownUSimFilePathsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownUSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownUSimFilePaths";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct LegacyNetworkOperatorsContract(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandAccount(::windows::runtime::IInspectable);
impl MobileBroadbandAccount {
    pub fn NetworkAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ServiceProviderGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn ServiceProviderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CurrentNetwork(&self) -> ::windows::runtime::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandNetwork>(result__)
        }
    }
    pub fn CurrentDeviceInformation(&self) -> ::windows::runtime::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceInformation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub fn GetConnectionProfiles(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableNetworkAccountIds() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkaccountid: Param0) -> ::windows::runtime::Result<MobileBroadbandAccount> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandAccount>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AccountExperienceUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandAccount3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn IMobileBroadbandAccountStatics<R, F: FnOnce(&IMobileBroadbandAccountStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MobileBroadbandAccount, IMobileBroadbandAccountStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandAccount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccount;{36c24ccd-cee2-43e0-a603-ee86a36d6570})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(918703309, 52962, 17376, [166, 3, 238, 134, 163, 109, 101, 112]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandAccount {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccount";
}
impl ::std::convert::From<MobileBroadbandAccount> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandAccount> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandAccount> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandAccount) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandAccount> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandAccount) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandAccountEventArgs(::windows::runtime::IInspectable);
impl MobileBroadbandAccountEventArgs {
    pub fn NetworkAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandAccountEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccountEventArgs;{3853c880-77de-4c04-bead-a123b08c9f59})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945014912, 30686, 19460, [190, 173, 161, 35, 176, 140, 159, 89]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandAccountEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountEventArgs";
}
impl ::std::convert::From<MobileBroadbandAccountEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandAccountEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandAccountEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandAccountEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandAccountEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandAccountEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandAccountEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandAccountEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandAccountUpdatedEventArgs(::windows::runtime::IInspectable);
impl MobileBroadbandAccountUpdatedEventArgs {
    pub fn NetworkAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn HasDeviceInformationChanged(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasNetworkChanged(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandAccountUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccountUpdatedEventArgs;{7bc31d88-a6bd-49e1-80ab-6b91354a57d4})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2076384648, 42685, 18913, [128, 171, 107, 145, 53, 74, 87, 212]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandAccountUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountUpdatedEventArgs";
}
impl ::std::convert::From<MobileBroadbandAccountUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandAccountUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandAccountUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandAccountUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandAccountUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandAccountUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandAccountUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandAccountUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandAccountWatcher(::windows::runtime::IInspectable);
impl MobileBroadbandAccountWatcher {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MobileBroadbandAccountWatcher, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn AccountAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AccountUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AccountRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows::runtime::Result<MobileBroadbandAccountWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandAccountWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandAccountWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandAccountWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcher;{6bf3335e-23b5-449f-928d-5e0d3e04471d})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1811100510, 9141, 17567, [146, 141, 94, 13, 62, 4, 71, 29]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandAccountWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcher";
}
impl ::std::convert::From<MobileBroadbandAccountWatcher> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandAccountWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandAccountWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandAccountWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandAccountWatcher> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandAccountWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandAccountWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandAccountWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(0i32);
    pub const Started: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(1i32);
    pub const EnumerationCompleted: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(2i32);
    pub const Stopped: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(3i32);
    pub const Aborted: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(4i32);
}
impl ::std::convert::From<i32> for MobileBroadbandAccountWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandAccountWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandAccountWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandAntennaSar(::windows::runtime::IInspectable);
impl MobileBroadbandAntennaSar {
    pub fn AntennaIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SarBackoffIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn CreateWithIndex(antennaindex: i32, sarbackoffindex: i32) -> ::windows::runtime::Result<MobileBroadbandAntennaSar> {
        Self::IMobileBroadbandAntennaSarFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), antennaindex, sarbackoffindex, &mut result__).from_abi::<MobileBroadbandAntennaSar>(result__)
        })
    }
    pub fn IMobileBroadbandAntennaSarFactory<R, F: FnOnce(&IMobileBroadbandAntennaSarFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MobileBroadbandAntennaSar, IMobileBroadbandAntennaSarFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandAntennaSar {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAntennaSar;{b9af4b7e-cbf9-4109-90be-5c06bfd513b6})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3115273086, 52217, 16649, [144, 190, 92, 6, 191, 213, 19, 182]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandAntennaSar {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAntennaSar";
}
impl ::std::convert::From<MobileBroadbandAntennaSar> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandAntennaSar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandAntennaSar> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandAntennaSar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandAntennaSar> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandAntennaSar) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandAntennaSar> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandAntennaSar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandAntennaSar {}
unsafe impl ::std::marker::Sync for MobileBroadbandAntennaSar {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellCdma(::windows::runtime::IInspectable);
impl MobileBroadbandCellCdma {
    #[cfg(feature = "Foundation")]
    pub fn BaseStationId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BaseStationPNCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLatitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLongitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLastBroadcastGpsTime(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NetworkId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PilotSignalStrengthInDB(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellCdma {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellCdma;{0601b3b4-411a-4f2e-8287-76f5650c60cd})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(100774836, 16666, 20270, [130, 135, 118, 245, 101, 12, 96, 205]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellCdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellCdma";
}
impl ::std::convert::From<MobileBroadbandCellCdma> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellCdma) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellCdma> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellCdma) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellCdma> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellCdma) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellCdma> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellCdma) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellCdma {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellCdma {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellGsm(::windows::runtime::IInspectable);
impl MobileBroadbandCellGsm {
    #[cfg(feature = "Foundation")]
    pub fn BaseStationId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalStrengthInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellGsm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellGsm;{cc917f06-7ee0-47b8-9e1f-c3b48df9df5b})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3432087302, 32480, 18360, [158, 31, 195, 180, 141, 249, 223, 91]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellGsm {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellGsm";
}
impl ::std::convert::From<MobileBroadbandCellGsm> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellGsm) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellGsm> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellGsm) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellGsm> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellGsm) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellGsm> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellGsm) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellGsm {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellGsm {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellLte(::windows::runtime::IInspectable);
impl MobileBroadbandCellLte {
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PhysicalCellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrackingAreaCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellLte {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellLte;{9197c87b-2b78-456d-8b53-aaa25d0af741})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2442643579, 11128, 17773, [139, 83, 170, 162, 93, 10, 247, 65]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellLte {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellLte";
}
impl ::std::convert::From<MobileBroadbandCellLte> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellLte) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellLte> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellLte) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellLte> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellLte) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellLte> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellLte) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellLte {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellLte {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellNR(::windows::runtime::IInspectable);
impl MobileBroadbandCellNR {
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PhysicalCellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInNanoseconds(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrackingAreaCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignalToNoiseRatioInDB(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellNR {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellNR;{a13f0deb-66fc-4b4b-83a9-a487a3a5a0a6})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2705264107, 26364, 19275, [131, 169, 164, 135, 163, 165, 160, 166]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellNR {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellNR";
}
impl ::std::convert::From<MobileBroadbandCellNR> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellNR) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellNR> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellNR) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellNR> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellNR) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellNR> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellNR) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellNR {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellNR {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellTdscdma(::windows::runtime::IInspectable);
impl MobileBroadbandCellTdscdma {
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CellParameterId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PathLossInDB(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellTdscdma {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellTdscdma;{0eda1655-db0e-4182-8cda-cc419a7bde08})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(249173589, 56078, 16770, [140, 218, 204, 65, 154, 123, 222, 8]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellTdscdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellTdscdma";
}
impl ::std::convert::From<MobileBroadbandCellTdscdma> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellTdscdma) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellTdscdma> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellTdscdma) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellTdscdma> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellTdscdma) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellTdscdma> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellTdscdma) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellTdscdma {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellTdscdma {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellUmts(::windows::runtime::IInspectable);
impl MobileBroadbandCellUmts {
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PathLossInDB(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrimaryScramblingCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignalToNoiseRatioInDB(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellUmts {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellUmts;{77b4b5ae-49c8-4f15-b285-4c26a7f67215})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2008331694, 18888, 20245, [178, 133, 76, 38, 167, 246, 114, 21]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellUmts {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellUmts";
}
impl ::std::convert::From<MobileBroadbandCellUmts> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellUmts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellUmts> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellUmts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellUmts> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellUmts) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellUmts> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellUmts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellUmts {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellUmts {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCellsInfo(::windows::runtime::IInspectable);
impl MobileBroadbandCellsInfo {
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsCdma(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsGsm(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsLte(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsTdscdma(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsUmts(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsCdma(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsGsm(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsLte(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsTdscdma(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsUmts(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsNR(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsNR(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCellsInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellsInfo;{89a9562a-e472-4da5-929c-de61711dd261})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2309576234, 58482, 19877, [146, 156, 222, 97, 113, 29, 210, 97]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCellsInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellsInfo";
}
impl ::std::convert::From<MobileBroadbandCellsInfo> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCellsInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCellsInfo> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCellsInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCellsInfo> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCellsInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCellsInfo> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCellsInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCellsInfo {}
unsafe impl ::std::marker::Sync for MobileBroadbandCellsInfo {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandCurrentSlotIndexChangedEventArgs(::windows::runtime::IInspectable);
impl MobileBroadbandCurrentSlotIndexChangedEventArgs {
    pub fn CurrentSlotIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCurrentSlotIndexChangedEventArgs;{f718b184-c370-5fd4-a670-1846cb9bce47})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4145590660, 50032, 24532, [166, 112, 24, 70, 203, 155, 206, 71]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCurrentSlotIndexChangedEventArgs";
}
impl ::std::convert::From<MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
unsafe impl ::std::marker::Sync for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceInformation(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceInformation {
    pub fn NetworkDeviceStatus(&self) -> ::windows::runtime::Result<NetworkDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__: NetworkDeviceStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkDeviceStatus>(result__)
        }
    }
    pub fn Manufacturer(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Model(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn FirmwareInformation(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Sms")]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<super::super::Devices::Sms::CellularClass> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sms::CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sms::CellularClass>(result__)
        }
    }
    pub fn DataClasses(&self) -> ::windows::runtime::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__: DataClasses = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataClasses>(result__)
        }
    }
    pub fn CustomDataClass(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn MobileEquipmentId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TelephoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    pub fn SubscriberId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::runtime::Result<MobileBroadbandDeviceType> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceType>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CurrentRadioState(&self) -> ::windows::runtime::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandRadioState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandRadioState>(result__)
        }
    }
    pub fn PinManager(&self) -> ::windows::runtime::Result<MobileBroadbandPinManager> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinManager>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SimSpn(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SimPnn(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SimGid1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SlotManager(&self) -> ::windows::runtime::Result<MobileBroadbandSlotManager> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandDeviceInformation4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSlotManager>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceInformation;{e6d08168-e381-4c6e-9be8-fe156969a446})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872424296, 58241, 19566, [155, 232, 254, 21, 105, 105, 164, 70]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceInformation";
}
impl ::std::convert::From<MobileBroadbandDeviceInformation> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceInformation> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceInformation> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceInformation> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceService(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceService {
    pub fn DeviceServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCommands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn OpenDataSession(&self) -> ::windows::runtime::Result<MobileBroadbandDeviceServiceDataSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceServiceDataSession>(result__)
        }
    }
    pub fn OpenCommandSession(&self) -> ::windows::runtime::Result<MobileBroadbandDeviceServiceCommandSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceServiceCommandSession>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceService {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceService;{22be1a52-bd80-40ac-8e1f-2e07836a3dbd})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582883922, 48512, 16556, [142, 31, 46, 7, 131, 106, 61, 189]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceService {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceService";
}
impl ::std::convert::From<MobileBroadbandDeviceService> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceService) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceService> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceService) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceService> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceService) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceService> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceService {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceService {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceServiceCommandResult(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceCommandResult {
    pub fn StatusCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceCommandResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandResult;{b0f46abb-94d6-44b9-a538-f0810b645389})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2968808123, 38102, 17593, [165, 56, 240, 129, 11, 100, 83, 137]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceCommandResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandResult";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceCommandResult> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceCommandResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceCommandResult> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceCommandResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceCommandResult> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceCommandResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceCommandResult> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceCommandResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceCommandResult {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceCommandResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceServiceCommandSession(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceCommandSession {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendQueryCommandAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, commandid: u32, data: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), commandid, data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendSetCommandAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, commandid: u32, data: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), commandid, data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>(result__)
        }
    }
    pub fn CloseSession(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceCommandSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandSession;{fc098a45-913b-4914-b6c3-ae6304593e75})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4228483653, 37179, 18708, [182, 195, 174, 99, 4, 89, 62, 117]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceCommandSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandSession";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceCommandSession> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceCommandSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceCommandSession> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceCommandSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceCommandSession> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceCommandSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceCommandSession> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceCommandSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceCommandSession {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceCommandSession {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceServiceDataReceivedEventArgs(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceDataReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataReceivedEventArgs;{b6aa13de-1380-40e3-8618-73cbca48138c})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3064599518, 4992, 16611, [134, 24, 115, 203, 202, 72, 19, 140]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataReceivedEventArgs";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceServiceDataSession(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceDataSession {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CloseSession(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceDataSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataSession;{dad62333-8bcf-4289-8a37-045c2169486a})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3671466803, 35791, 17033, [138, 55, 4, 92, 33, 105, 72, 106]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceDataSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataSession";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceDataSession> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceDataSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceDataSession> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceDataSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceDataSession> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceDataSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceDataSession> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceDataSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceDataSession {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceDataSession {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceServiceInformation(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceInformation {
    pub fn DeviceServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn IsDataReadSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDataWriteSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceInformation;{53d69b5b-c4ed-45f0-803a-d9417a6d9846})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1406573403, 50413, 17904, [128, 58, 217, 65, 122, 109, 152, 70]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceInformation";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceInformation> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceInformation> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceInformation> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceInformation> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceInformation {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceInformation {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandDeviceServiceTriggerDetails(::windows::runtime::IInspectable);
impl MobileBroadbandDeviceServiceTriggerDetails {
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DeviceServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceServiceTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceTriggerDetails;{4a055b70-b9ae-4458-9241-a6a5fbf18a0c})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1241865072, 47534, 17496, [146, 65, 166, 165, 251, 241, 138, 12]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandDeviceServiceTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceTriggerDetails";
}
impl ::std::convert::From<MobileBroadbandDeviceServiceTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandDeviceServiceTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandDeviceServiceTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandDeviceServiceTriggerDetails {}
unsafe impl ::std::marker::Sync for MobileBroadbandDeviceServiceTriggerDetails {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: MobileBroadbandDeviceType = MobileBroadbandDeviceType(0i32);
    pub const Embedded: MobileBroadbandDeviceType = MobileBroadbandDeviceType(1i32);
    pub const Removable: MobileBroadbandDeviceType = MobileBroadbandDeviceType(2i32);
    pub const Remote: MobileBroadbandDeviceType = MobileBroadbandDeviceType(3i32);
}
impl ::std::convert::From<i32> for MobileBroadbandDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandDeviceType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandDeviceType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandDeviceType;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandModem(::windows::runtime::IInspectable);
impl MobileBroadbandModem {
    pub fn CurrentAccount(&self) -> ::windows::runtime::Result<MobileBroadbandAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandAccount>(result__)
        }
    }
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceInformation>(result__)
        }
    }
    pub fn MaxDeviceServiceCommandSizeInBytes(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxDeviceServiceDataSizeInBytes(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceServices(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>>(result__)
        }
    }
    pub fn GetDeviceService<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, deviceserviceid: Param0) -> ::windows::runtime::Result<MobileBroadbandDeviceService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), deviceserviceid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandDeviceService>(result__)
        }
    }
    pub fn IsResetSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResetAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentConfigurationAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>>(result__)
        }
    }
    pub fn CurrentNetwork(&self) -> ::windows::runtime::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandNetwork>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn FromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandModem>(result__)
        })
    }
    pub fn GetDefault() -> ::windows::runtime::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandModem>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsPassthroughEnabledAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetIsPassthroughEnabledAsync(&self, value: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryGetPcoAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>>(result__)
        }
    }
    pub fn IsInEmergencyCallMode(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsInEmergencyCallModeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsInEmergencyCallModeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IMobileBroadbandModemStatics<R, F: FnOnce(&IMobileBroadbandModemStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MobileBroadbandModem, IMobileBroadbandModemStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandModem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandModem;{d0356912-e9f9-4f67-a03d-43189a316bf1})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3493161234, 59897, 20327, [160, 61, 67, 24, 154, 49, 107, 241]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandModem {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModem";
}
impl ::std::convert::From<MobileBroadbandModem> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandModem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandModem> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandModem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandModem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandModem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandModem> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandModem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandModem> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandModem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandModem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandModem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandModem {}
unsafe impl ::std::marker::Sync for MobileBroadbandModem {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandModemConfiguration(::windows::runtime::IInspectable);
impl MobileBroadbandModemConfiguration {
    pub fn Uicc(&self) -> ::windows::runtime::Result<MobileBroadbandUicc> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUicc>(result__)
        }
    }
    pub fn HomeProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn HomeProviderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SarManager(&self) -> ::windows::runtime::Result<MobileBroadbandSarManager> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandModemConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSarManager>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandModemConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandModemConfiguration;{fce035a3-d6cd-4320-b982-be9d3ec7890f})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4242552227, 54989, 17184, [185, 130, 190, 157, 62, 199, 137, 15]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandModemConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemConfiguration";
}
impl ::std::convert::From<MobileBroadbandModemConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandModemConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandModemConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandModemConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandModemConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandModemConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandModemConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandModemConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandModemIsolation(::windows::runtime::IInspectable);
impl MobileBroadbandModemIsolation {
    pub fn AddAllowedHost<'a, Param0: ::windows::runtime::IntoParam<'a, super::HostName>>(&self, host: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), host.into_param().abi()).ok() }
    }
    pub fn AddAllowedHostRange<'a, Param0: ::windows::runtime::IntoParam<'a, super::HostName>, Param1: ::windows::runtime::IntoParam<'a, super::HostName>>(&self, first: Param0, last: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), first.into_param().abi(), last.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ApplyConfigurationAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClearConfigurationAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(modemdeviceid: Param0, rulegroupid: Param1) -> ::windows::runtime::Result<MobileBroadbandModemIsolation> {
        Self::IMobileBroadbandModemIsolationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), modemdeviceid.into_param().abi(), rulegroupid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandModemIsolation>(result__)
        })
    }
    pub fn IMobileBroadbandModemIsolationFactory<R, F: FnOnce(&IMobileBroadbandModemIsolationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MobileBroadbandModemIsolation, IMobileBroadbandModemIsolationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandModemIsolation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandModemIsolation;{b5618fec-e661-4330-9bb4-3480212ec354})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3043069932, 58977, 17200, [155, 180, 52, 128, 33, 46, 195, 84]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandModemIsolation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemIsolation";
}
impl ::std::convert::From<MobileBroadbandModemIsolation> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandModemIsolation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandModemIsolation> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandModemIsolation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandModemIsolation> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandModemIsolation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandModemIsolation> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandModemIsolation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandModemIsolation {}
unsafe impl ::std::marker::Sync for MobileBroadbandModemIsolation {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: MobileBroadbandModemStatus = MobileBroadbandModemStatus(0i32);
    pub const OtherFailure: MobileBroadbandModemStatus = MobileBroadbandModemStatus(1i32);
    pub const Busy: MobileBroadbandModemStatus = MobileBroadbandModemStatus(2i32);
    pub const NoDeviceSupport: MobileBroadbandModemStatus = MobileBroadbandModemStatus(3i32);
}
impl ::std::convert::From<i32> for MobileBroadbandModemStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandModemStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandModemStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandModemStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandNetwork(::windows::runtime::IInspectable);
impl MobileBroadbandNetwork {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows::runtime::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Connectivity::NetworkAdapter>(result__)
        }
    }
    pub fn NetworkRegistrationState(&self) -> ::windows::runtime::Result<NetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__: NetworkRegistrationState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkRegistrationState>(result__)
        }
    }
    pub fn RegistrationNetworkError(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PacketAttachNetworkError(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ActivationNetworkError(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AccessPointName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RegisteredDataClass(&self) -> ::windows::runtime::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__: DataClasses = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataClasses>(result__)
        }
    }
    pub fn RegisteredProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RegisteredProviderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ShowConnectionUI(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetVoiceCallSupportAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegistrationUiccApps(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetCellsInfoAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>> {
        let this = &::windows::runtime::Interface::cast::<IMobileBroadbandNetwork3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandNetwork {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandNetwork;{cb63928c-0309-4cb6-a8c1-6a5a3c8e1ff6})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3412300428, 777, 19638, [168, 193, 106, 90, 60, 142, 31, 246]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandNetwork {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetwork";
}
impl ::std::convert::From<MobileBroadbandNetwork> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandNetwork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandNetwork> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandNetwork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandNetwork> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandNetwork) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandNetwork> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandNetwork) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandNetworkRegistrationStateChange(::windows::runtime::IInspectable);
impl MobileBroadbandNetworkRegistrationStateChange {
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Network(&self) -> ::windows::runtime::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandNetwork>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandNetworkRegistrationStateChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChange;{beaf94e1-960f-49b4-a08d-7d85e968c7ec})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3199177953, 38415, 18868, [160, 141, 125, 133, 233, 104, 199, 236]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandNetworkRegistrationStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChange";
}
impl ::std::convert::From<MobileBroadbandNetworkRegistrationStateChange> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandNetworkRegistrationStateChange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandNetworkRegistrationStateChange> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandNetworkRegistrationStateChange> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandNetworkRegistrationStateChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandNetworkRegistrationStateChange> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandNetworkRegistrationStateChange {}
unsafe impl ::std::marker::Sync for MobileBroadbandNetworkRegistrationStateChange {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandNetworkRegistrationStateChangeTriggerDetails(::windows::runtime::IInspectable);
impl MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn NetworkRegistrationStateChanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChangeTriggerDetails;{89135cff-28b8-46aa-b137-1c4b0f21edfe})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2299747583, 10424, 18090, [177, 55, 28, 75, 15, 33, 237, 254]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChangeTriggerDetails";
}
impl ::std::convert::From<MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
unsafe impl ::std::marker::Sync for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPco(::windows::runtime::IInspectable);
impl MobileBroadbandPco {
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPco {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPco;{d4e4fcbe-e3a3-43c5-a87b-6c86d229d7fa})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3571776702, 58275, 17349, [168, 123, 108, 134, 210, 41, 215, 250]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPco {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPco";
}
impl ::std::convert::From<MobileBroadbandPco> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPco) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPco> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPco) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPco {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPco {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPco> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPco) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPco> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPco) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPco {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPco {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPco {}
unsafe impl ::std::marker::Sync for MobileBroadbandPco {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPcoDataChangeTriggerDetails(::windows::runtime::IInspectable);
impl MobileBroadbandPcoDataChangeTriggerDetails {
    pub fn UpdatedData(&self) -> ::windows::runtime::Result<MobileBroadbandPco> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPco>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPcoDataChangeTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPcoDataChangeTriggerDetails;{263f5114-64e0-4493-909b-2d14a01962b1})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(641683732, 25824, 17555, [144, 155, 45, 20, 160, 25, 98, 177]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPcoDataChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPcoDataChangeTriggerDetails";
}
impl ::std::convert::From<MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPcoDataChangeTriggerDetails {}
unsafe impl ::std::marker::Sync for MobileBroadbandPcoDataChangeTriggerDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPin(::windows::runtime::IInspectable);
impl MobileBroadbandPin {
    pub fn Type(&self) -> ::windows::runtime::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinType>(result__)
        }
    }
    pub fn LockState(&self) -> ::windows::runtime::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinLockState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinLockState>(result__)
        }
    }
    pub fn Format(&self) -> ::windows::runtime::Result<MobileBroadbandPinFormat> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinFormat>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MaxLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MinLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AttemptsRemaining(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, currentpin: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), currentpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, currentpin: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), currentpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnterAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, currentpin: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), currentpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChangeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, currentpin: Param0, newpin: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), currentpin.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnblockAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, pinunblockkey: Param0, newpin: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), pinunblockkey.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPin {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPin;{e661d709-e779-45bf-8281-75323df9e321})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3865171721, 59257, 17855, [130, 129, 117, 50, 61, 249, 227, 33]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPin {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPin";
}
impl ::std::convert::From<MobileBroadbandPin> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPin) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPin> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPin) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPin> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPin) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPin> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPin {}
unsafe impl ::std::marker::Sync for MobileBroadbandPin {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: MobileBroadbandPinFormat = MobileBroadbandPinFormat(0i32);
    pub const Numeric: MobileBroadbandPinFormat = MobileBroadbandPinFormat(1i32);
    pub const Alphanumeric: MobileBroadbandPinFormat = MobileBroadbandPinFormat(2i32);
}
impl ::std::convert::From<i32> for MobileBroadbandPinFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandPinFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinFormat;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: MobileBroadbandPinLockState = MobileBroadbandPinLockState(0i32);
    pub const Unlocked: MobileBroadbandPinLockState = MobileBroadbandPinLockState(1i32);
    pub const PinRequired: MobileBroadbandPinLockState = MobileBroadbandPinLockState(2i32);
    pub const PinUnblockKeyRequired: MobileBroadbandPinLockState = MobileBroadbandPinLockState(3i32);
}
impl ::std::convert::From<i32> for MobileBroadbandPinLockState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandPinLockState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinLockState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinLockState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPinLockStateChange(::windows::runtime::IInspectable);
impl MobileBroadbandPinLockStateChange {
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn PinType(&self) -> ::windows::runtime::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinType>(result__)
        }
    }
    pub fn PinLockState(&self) -> ::windows::runtime::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinLockState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinLockState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinLockStateChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChange;{be16673e-1f04-4f95-8b90-e7f559dde7e5})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189139262, 7940, 20373, [139, 144, 231, 245, 89, 221, 231, 229]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPinLockStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChange";
}
impl ::std::convert::From<MobileBroadbandPinLockStateChange> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPinLockStateChange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChange> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPinLockStateChange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPinLockStateChange> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPinLockStateChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChange> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPinLockStateChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPinLockStateChange {}
unsafe impl ::std::marker::Sync for MobileBroadbandPinLockStateChange {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(::windows::runtime::IInspectable);
impl MobileBroadbandPinLockStateChangeTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PinLockStateChanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinLockStateChangeTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChangeTriggerDetails;{d338c091-3e91-4d38-9036-aee83a6e79ad})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3543711889, 16017, 19768, [144, 54, 174, 232, 58, 110, 121, 173]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPinLockStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChangeTriggerDetails";
}
impl ::std::convert::From<MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPinLockStateChangeTriggerDetails {}
unsafe impl ::std::marker::Sync for MobileBroadbandPinLockStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPinManager(::windows::runtime::IInspectable);
impl MobileBroadbandPinManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPins(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>>(result__)
        }
    }
    pub fn GetPin(&self, pintype: MobileBroadbandPinType) -> ::windows::runtime::Result<MobileBroadbandPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), pintype, &mut result__).from_abi::<MobileBroadbandPin>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinManager;{83567edd-6e1f-4b9b-a413-2b1f50cc36df})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2203483869, 28191, 19355, [164, 19, 43, 31, 80, 204, 54, 223]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPinManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinManager";
}
impl ::std::convert::From<MobileBroadbandPinManager> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPinManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPinManager> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPinManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPinManager> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPinManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPinManager> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPinManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPinManager {}
unsafe impl ::std::marker::Sync for MobileBroadbandPinManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandPinOperationResult(::windows::runtime::IInspectable);
impl MobileBroadbandPinOperationResult {
    pub fn IsSuccessful(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AttemptsRemaining(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinOperationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinOperationResult;{11dddc32-31e7-49f5-b663-123d3bef0362})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(299752498, 12775, 18933, [182, 99, 18, 61, 59, 239, 3, 98]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandPinOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinOperationResult";
}
impl ::std::convert::From<MobileBroadbandPinOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandPinOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandPinOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandPinOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandPinOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandPinOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandPinOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandPinOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandPinOperationResult {}
unsafe impl ::std::marker::Sync for MobileBroadbandPinOperationResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandPinType(pub i32);
impl MobileBroadbandPinType {
    pub const None: MobileBroadbandPinType = MobileBroadbandPinType(0i32);
    pub const Custom: MobileBroadbandPinType = MobileBroadbandPinType(1i32);
    pub const Pin1: MobileBroadbandPinType = MobileBroadbandPinType(2i32);
    pub const Pin2: MobileBroadbandPinType = MobileBroadbandPinType(3i32);
    pub const SimPin: MobileBroadbandPinType = MobileBroadbandPinType(4i32);
    pub const FirstSimPin: MobileBroadbandPinType = MobileBroadbandPinType(5i32);
    pub const NetworkPin: MobileBroadbandPinType = MobileBroadbandPinType(6i32);
    pub const NetworkSubsetPin: MobileBroadbandPinType = MobileBroadbandPinType(7i32);
    pub const ServiceProviderPin: MobileBroadbandPinType = MobileBroadbandPinType(8i32);
    pub const CorporatePin: MobileBroadbandPinType = MobileBroadbandPinType(9i32);
    pub const SubsidyLock: MobileBroadbandPinType = MobileBroadbandPinType(10i32);
}
impl ::std::convert::From<i32> for MobileBroadbandPinType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandPinType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandPinType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinType;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: MobileBroadbandRadioState = MobileBroadbandRadioState(0i32);
    pub const On: MobileBroadbandRadioState = MobileBroadbandRadioState(1i32);
}
impl ::std::convert::From<i32> for MobileBroadbandRadioState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandRadioState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandRadioState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandRadioState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandRadioStateChange(::windows::runtime::IInspectable);
impl MobileBroadbandRadioStateChange {
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RadioState(&self) -> ::windows::runtime::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandRadioState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandRadioState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandRadioStateChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChange;{b054a561-9833-4aed-9717-4348b21a24b3})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2958337377, 38963, 19181, [151, 23, 67, 72, 178, 26, 36, 179]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandRadioStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChange";
}
impl ::std::convert::From<MobileBroadbandRadioStateChange> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandRadioStateChange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChange> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandRadioStateChange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandRadioStateChange> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandRadioStateChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChange> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandRadioStateChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandRadioStateChange {}
unsafe impl ::std::marker::Sync for MobileBroadbandRadioStateChange {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandRadioStateChangeTriggerDetails(::windows::runtime::IInspectable);
impl MobileBroadbandRadioStateChangeTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn RadioStateChanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandRadioStateChangeTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChangeTriggerDetails;{71301ace-093c-42c6-b0db-ad1f75a65445})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1898977998, 2364, 17094, [176, 219, 173, 31, 117, 166, 84, 69]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandRadioStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChangeTriggerDetails";
}
impl ::std::convert::From<MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandRadioStateChangeTriggerDetails {}
unsafe impl ::std::marker::Sync for MobileBroadbandRadioStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandSarManager(::windows::runtime::IInspectable);
impl MobileBroadbandSarManager {
    pub fn IsBackoffEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsWiFiHardwareIntegrated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsSarControlledByHardware(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Antennas(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HysteresisTimerPeriod(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TransmissionStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransmissionStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnableBackoffAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DisableBackoffAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>>(&self, antennas: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), antennas.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RevertSarToHardwareControlAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTransmissionStateChangedHysteresisAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timerperiod: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), timerperiod.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsTransmittingAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn StartTransmissionStateMonitoring(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn StopTransmissionStateMonitoring(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandSarManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSarManager;{e5b26833-967e-40c9-a485-19c0dd209e22})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3853674547, 38526, 16585, [164, 133, 25, 192, 221, 32, 158, 34]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandSarManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSarManager";
}
impl ::std::convert::From<MobileBroadbandSarManager> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandSarManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandSarManager> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandSarManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandSarManager> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandSarManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandSarManager> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandSarManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandSarManager {}
unsafe impl ::std::marker::Sync for MobileBroadbandSarManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandSlotInfo(::windows::runtime::IInspectable);
impl MobileBroadbandSlotInfo {
    pub fn Index(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn State(&self) -> ::windows::runtime::Result<MobileBroadbandSlotState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandSlotState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSlotState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandSlotInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSlotInfo;{bd350b32-882e-542a-b17d-0bb1b49bae9e})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3174370098, 34862, 21546, [177, 125, 11, 177, 180, 155, 174, 158]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandSlotInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfo";
}
impl ::std::convert::From<MobileBroadbandSlotInfo> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandSlotInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandSlotInfo> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandSlotInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandSlotInfo> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandSlotInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandSlotInfo> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandSlotInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandSlotInfo {}
unsafe impl ::std::marker::Sync for MobileBroadbandSlotInfo {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandSlotInfoChangedEventArgs(::windows::runtime::IInspectable);
impl MobileBroadbandSlotInfoChangedEventArgs {
    pub fn SlotInfo(&self) -> ::windows::runtime::Result<MobileBroadbandSlotInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSlotInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandSlotInfoChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSlotInfoChangedEventArgs;{3158839f-950c-54ce-a48d-ba4529b48f0f})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(827884447, 38156, 21710, [164, 141, 186, 69, 41, 180, 143, 15]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandSlotInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfoChangedEventArgs";
}
impl ::std::convert::From<MobileBroadbandSlotInfoChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandSlotInfoChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandSlotInfoChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandSlotInfoChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandSlotInfoChangedEventArgs {}
unsafe impl ::std::marker::Sync for MobileBroadbandSlotInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandSlotManager(::windows::runtime::IInspectable);
impl MobileBroadbandSlotManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SlotInfos(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>>(result__)
        }
    }
    pub fn CurrentSlotIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetCurrentSlot(&self, slotindex: i32) -> ::windows::runtime::Result<MobileBroadbandModemStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandModemStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), slotindex, &mut result__).from_abi::<MobileBroadbandModemStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCurrentSlotAsync(&self, slotindex: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), slotindex, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SlotInfoChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSlotInfoChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CurrentSlotIndexChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentSlotIndexChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandSlotManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSlotManager;{eba07cd6-2019-5f81-a294-cc364a11d0b2})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3953163478, 8217, 24449, [162, 148, 204, 54, 74, 17, 208, 178]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandSlotManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotManager";
}
impl ::std::convert::From<MobileBroadbandSlotManager> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandSlotManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandSlotManager> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandSlotManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandSlotManager> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandSlotManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandSlotManager> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandSlotManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandSlotManager {}
unsafe impl ::std::marker::Sync for MobileBroadbandSlotManager {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandSlotState(pub i32);
impl MobileBroadbandSlotState {
    pub const Unmanaged: MobileBroadbandSlotState = MobileBroadbandSlotState(0i32);
    pub const Unknown: MobileBroadbandSlotState = MobileBroadbandSlotState(1i32);
    pub const OffEmpty: MobileBroadbandSlotState = MobileBroadbandSlotState(2i32);
    pub const Off: MobileBroadbandSlotState = MobileBroadbandSlotState(3i32);
    pub const Empty: MobileBroadbandSlotState = MobileBroadbandSlotState(4i32);
    pub const NotReady: MobileBroadbandSlotState = MobileBroadbandSlotState(5i32);
    pub const Active: MobileBroadbandSlotState = MobileBroadbandSlotState(6i32);
    pub const Error: MobileBroadbandSlotState = MobileBroadbandSlotState(7i32);
    pub const ActiveEsim: MobileBroadbandSlotState = MobileBroadbandSlotState(8i32);
    pub const ActiveEsimNoProfile: MobileBroadbandSlotState = MobileBroadbandSlotState(9i32);
}
impl ::std::convert::From<i32> for MobileBroadbandSlotState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandSlotState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandSlotState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandSlotState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(::windows::runtime::IInspectable);
impl MobileBroadbandTransmissionStateChangedEventArgs {
    pub fn IsTransmitting(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandTransmissionStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandTransmissionStateChangedEventArgs;{612e3875-040a-4f99-a4f9-61d7c32da129})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1630419061, 1034, 20377, [164, 249, 97, 215, 195, 45, 161, 41]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandTransmissionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandTransmissionStateChangedEventArgs";
}
impl ::std::convert::From<MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandTransmissionStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for MobileBroadbandTransmissionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandUicc(::windows::runtime::IInspectable);
impl MobileBroadbandUicc {
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetUiccAppsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandUicc {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUicc;{e634f691-525a-4ce2-8fce-aa4162579154})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3862230673, 21082, 19682, [143, 206, 170, 65, 98, 87, 145, 84]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandUicc {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUicc";
}
impl ::std::convert::From<MobileBroadbandUicc> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandUicc) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandUicc> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandUicc) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandUicc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandUicc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandUicc> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandUicc) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandUicc> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandUicc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandUicc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandUicc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandUicc {}
unsafe impl ::std::marker::Sync for MobileBroadbandUicc {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandUiccApp(::windows::runtime::IInspectable);
impl MobileBroadbandUiccApp {
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<UiccAppKind> {
        let this = self;
        unsafe {
            let mut result__: UiccAppKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UiccAppKind>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetRecordDetailsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, uiccfilepath: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), uiccfilepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadRecordAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, uiccfilepath: Param0, recordindex: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), uiccfilepath.into_param().abi(), recordindex, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandUiccApp {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccApp;{4d170556-98a1-43dd-b2ec-50c90cf248df})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1293354326, 39073, 17373, [178, 236, 80, 201, 12, 242, 72, 223]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandUiccApp {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccApp";
}
impl ::std::convert::From<MobileBroadbandUiccApp> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandUiccApp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandUiccApp> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandUiccApp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandUiccApp> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandUiccApp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandUiccApp> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandUiccApp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandUiccApp {}
unsafe impl ::std::marker::Sync for MobileBroadbandUiccApp {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(0i32);
    pub const InvalidUiccFilePath: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(1i32);
    pub const AccessConditionNotHeld: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(2i32);
    pub const UiccBusy: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(3i32);
}
impl ::std::convert::From<i32> for MobileBroadbandUiccAppOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MobileBroadbandUiccAppOperationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandUiccAppOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandUiccAppReadRecordResult(::windows::runtime::IInspectable);
impl MobileBroadbandUiccAppReadRecordResult {
    pub fn Status(&self) -> ::windows::runtime::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandUiccAppOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUiccAppOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandUiccAppReadRecordResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppReadRecordResult;{64c95285-358e-47c5-8249-695f383b2bdb})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1690915461, 13710, 18373, [130, 73, 105, 95, 56, 59, 43, 219]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandUiccAppReadRecordResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppReadRecordResult";
}
impl ::std::convert::From<MobileBroadbandUiccAppReadRecordResult> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandUiccAppReadRecordResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandUiccAppReadRecordResult> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandUiccAppReadRecordResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandUiccAppReadRecordResult> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandUiccAppReadRecordResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandUiccAppReadRecordResult> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandUiccAppReadRecordResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandUiccAppReadRecordResult {}
unsafe impl ::std::marker::Sync for MobileBroadbandUiccAppReadRecordResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(::windows::runtime::IInspectable);
impl MobileBroadbandUiccAppRecordDetailsResult {
    pub fn Status(&self) -> ::windows::runtime::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandUiccAppOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUiccAppOperationStatus>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<UiccAppRecordKind> {
        let this = self;
        unsafe {
            let mut result__: UiccAppRecordKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UiccAppRecordKind>(result__)
        }
    }
    pub fn RecordCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn RecordSize(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ReadAccessCondition(&self) -> ::windows::runtime::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__: UiccAccessCondition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UiccAccessCondition>(result__)
        }
    }
    pub fn WriteAccessCondition(&self) -> ::windows::runtime::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__: UiccAccessCondition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UiccAccessCondition>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandUiccAppRecordDetailsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppRecordDetailsResult;{d919682f-be14-4934-981d-2f57b9ed83e6})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3642320943, 48660, 18740, [152, 29, 47, 87, 185, 237, 131, 230]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandUiccAppRecordDetailsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppRecordDetailsResult";
}
impl ::std::convert::From<MobileBroadbandUiccAppRecordDetailsResult> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandUiccAppRecordDetailsResult> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandUiccAppRecordDetailsResult> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandUiccAppRecordDetailsResult> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandUiccAppRecordDetailsResult {}
unsafe impl ::std::marker::Sync for MobileBroadbandUiccAppRecordDetailsResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MobileBroadbandUiccAppsResult(::windows::runtime::IInspectable);
impl MobileBroadbandUiccAppsResult {
    pub fn Status(&self) -> ::windows::runtime::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandUiccAppOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUiccAppOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UiccApps(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MobileBroadbandUiccAppsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppsResult;{744930eb-8157-4a41-8494-6bf54c9b1d2b})");
}
unsafe impl ::windows::runtime::Interface for MobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950953707, 33111, 19009, [132, 148, 107, 245, 76, 155, 29, 43]);
}
impl ::windows::runtime::RuntimeName for MobileBroadbandUiccAppsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppsResult";
}
impl ::std::convert::From<MobileBroadbandUiccAppsResult> for ::windows::runtime::IUnknown {
    fn from(value: MobileBroadbandUiccAppsResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MobileBroadbandUiccAppsResult> for ::windows::runtime::IUnknown {
    fn from(value: &MobileBroadbandUiccAppsResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MobileBroadbandUiccAppsResult> for ::windows::runtime::IInspectable {
    fn from(value: MobileBroadbandUiccAppsResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MobileBroadbandUiccAppsResult> for ::windows::runtime::IInspectable {
    fn from(value: &MobileBroadbandUiccAppsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MobileBroadbandUiccAppsResult {}
unsafe impl ::std::marker::Sync for MobileBroadbandUiccAppsResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkDeviceStatus(pub i32);
impl NetworkDeviceStatus {
    pub const DeviceNotReady: NetworkDeviceStatus = NetworkDeviceStatus(0i32);
    pub const DeviceReady: NetworkDeviceStatus = NetworkDeviceStatus(1i32);
    pub const SimNotInserted: NetworkDeviceStatus = NetworkDeviceStatus(2i32);
    pub const BadSim: NetworkDeviceStatus = NetworkDeviceStatus(3i32);
    pub const DeviceHardwareFailure: NetworkDeviceStatus = NetworkDeviceStatus(4i32);
    pub const AccountNotActivated: NetworkDeviceStatus = NetworkDeviceStatus(5i32);
    pub const DeviceLocked: NetworkDeviceStatus = NetworkDeviceStatus(6i32);
    pub const DeviceBlocked: NetworkDeviceStatus = NetworkDeviceStatus(7i32);
}
impl ::std::convert::From<i32> for NetworkDeviceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkDeviceStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkDeviceStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkDeviceStatus;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: NetworkOperatorDataUsageNotificationKind = NetworkOperatorDataUsageNotificationKind(0i32);
}
impl ::std::convert::From<i32> for NetworkOperatorDataUsageNotificationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkOperatorDataUsageNotificationKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorDataUsageNotificationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorDataUsageNotificationKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkOperatorDataUsageTriggerDetails(::windows::runtime::IInspectable);
impl NetworkOperatorDataUsageTriggerDetails {
    pub fn NotificationKind(&self) -> ::windows::runtime::Result<NetworkOperatorDataUsageNotificationKind> {
        let this = self;
        unsafe {
            let mut result__: NetworkOperatorDataUsageNotificationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkOperatorDataUsageNotificationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorDataUsageTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorDataUsageTriggerDetails;{50e3126d-a465-4eeb-9317-28a167630cea})");
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1357058669, 42085, 20203, [147, 23, 40, 161, 103, 99, 12, 234]);
}
impl ::windows::runtime::RuntimeName for NetworkOperatorDataUsageTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorDataUsageTriggerDetails";
}
impl ::std::convert::From<NetworkOperatorDataUsageTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorDataUsageTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorDataUsageTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorDataUsageTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NetworkOperatorDataUsageTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorDataUsageTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorDataUsageTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorDataUsageTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NetworkOperatorDataUsageTriggerDetails {}
unsafe impl ::std::marker::Sync for NetworkOperatorDataUsageTriggerDetails {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(0i32);
    pub const Cdma: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(1i32);
    pub const Ussd: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(2i32);
    pub const DataPlanThresholdReached: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(3i32);
    pub const DataPlanReset: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(4i32);
    pub const DataPlanDeleted: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(5i32);
    pub const ProfileConnected: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(6i32);
    pub const ProfileDisconnected: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(7i32);
    pub const RegisteredRoaming: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(8i32);
    pub const RegisteredHome: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(9i32);
    pub const TetheringEntitlementCheck: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(10i32);
    pub const TetheringOperationalStateChanged: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(11i32);
    pub const TetheringNumberOfClientsChanged: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(12i32);
}
impl ::std::convert::From<i32> for NetworkOperatorEventMessageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkOperatorEventMessageType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorEventMessageType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorEventMessageType;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkOperatorNotificationEventDetails(::windows::runtime::IInspectable);
impl NetworkOperatorNotificationEventDetails {
    pub fn NotificationType(&self) -> ::windows::runtime::Result<NetworkOperatorEventMessageType> {
        let this = self;
        unsafe {
            let mut result__: NetworkOperatorEventMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkOperatorEventMessageType>(result__)
        }
    }
    pub fn NetworkAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn EncodingType(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RuleId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Sms")]
    pub fn SmsMessage(&self) -> ::windows::runtime::Result<super::super::Devices::Sms::ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sms::ISmsMessage>(result__)
        }
    }
    pub fn AuthorizeTethering<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, allow: bool, entitlementfailurereason: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INetworkOperatorTetheringEntitlementCheck>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), allow, entitlementfailurereason.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorNotificationEventDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorNotificationEventDetails;{bc68a9d1-82e1-4488-9f2c-1276c2468fac})");
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3160975825, 33505, 17544, [159, 44, 18, 118, 194, 70, 143, 172]);
}
impl ::windows::runtime::RuntimeName for NetworkOperatorNotificationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorNotificationEventDetails";
}
impl ::std::convert::From<NetworkOperatorNotificationEventDetails> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorNotificationEventDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorNotificationEventDetails> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorNotificationEventDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NetworkOperatorNotificationEventDetails> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorNotificationEventDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorNotificationEventDetails> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorNotificationEventDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NetworkOperatorNotificationEventDetails {}
unsafe impl ::std::marker::Sync for NetworkOperatorNotificationEventDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkOperatorTetheringAccessPointConfiguration(::windows::runtime::IInspectable);
impl NetworkOperatorTetheringAccessPointConfiguration {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkOperatorTetheringAccessPointConfiguration, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Ssid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetSsid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Passphrase(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetPassphrase<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsBandSupported(&self, band: TetheringWiFiBand) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), band, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsBandSupportedAsync(&self, band: TetheringWiFiBand) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), band, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Band(&self) -> ::windows::runtime::Result<TetheringWiFiBand> {
        let this = &::windows::runtime::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__: TetheringWiFiBand = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TetheringWiFiBand>(result__)
        }
    }
    pub fn SetBand(&self, value: TetheringWiFiBand) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorTetheringAccessPointConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringAccessPointConfiguration;{0bcc0284-412e-403d-acc6-b757e34774a4})");
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(197919364, 16686, 16445, [172, 198, 183, 87, 227, 71, 116, 164]);
}
impl ::windows::runtime::RuntimeName for NetworkOperatorTetheringAccessPointConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringAccessPointConfiguration";
}
impl ::std::convert::From<NetworkOperatorTetheringAccessPointConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringAccessPointConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NetworkOperatorTetheringAccessPointConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringAccessPointConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NetworkOperatorTetheringAccessPointConfiguration {}
unsafe impl ::std::marker::Sync for NetworkOperatorTetheringAccessPointConfiguration {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkOperatorTetheringClient(::windows::runtime::IInspectable);
impl NetworkOperatorTetheringClient {
    pub fn MacAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::HostName>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorTetheringClient {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringClient;{709d254c-595f-4847-bb30-646935542918})");
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1889346892, 22879, 18503, [187, 48, 100, 105, 53, 84, 41, 24]);
}
impl ::windows::runtime::RuntimeName for NetworkOperatorTetheringClient {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringClient";
}
impl ::std::convert::From<NetworkOperatorTetheringClient> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorTetheringClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringClient> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorTetheringClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NetworkOperatorTetheringClient> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorTetheringClient) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringClient> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorTetheringClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NetworkOperatorTetheringClient {}
unsafe impl ::std::marker::Sync for NetworkOperatorTetheringClient {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkOperatorTetheringManager(::windows::runtime::IInspectable);
impl NetworkOperatorTetheringManager {
    pub fn MaxClientCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ClientCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TetheringOperationalState(&self) -> ::windows::runtime::Result<TetheringOperationalState> {
        let this = self;
        unsafe {
            let mut result__: TetheringOperationalState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TetheringOperationalState>(result__)
        }
    }
    pub fn GetCurrentAccessPointConfiguration(&self) -> ::windows::runtime::Result<NetworkOperatorTetheringAccessPointConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NetworkOperatorTetheringAccessPointConfiguration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConfigureAccessPointAsync<'a, Param0: ::windows::runtime::IntoParam<'a, NetworkOperatorTetheringAccessPointConfiguration>>(&self, configuration: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTetheringAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopTetheringAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTetheringClients(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>> {
        let this = &::windows::runtime::Interface::cast::<INetworkOperatorTetheringClientManager>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>>(result__)
        }
    }
    pub fn GetTetheringCapability<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkaccountid: Param0) -> ::windows::runtime::Result<TetheringCapability> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__: TetheringCapability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<TetheringCapability>(result__)
        })
    }
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkaccountid: Param0) -> ::windows::runtime::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<NetworkOperatorTetheringManager>(result__)
        })
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn GetTetheringCapabilityFromConnectionProfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::Connectivity::ConnectionProfile>>(profile: Param0) -> ::windows::runtime::Result<TetheringCapability> {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__: TetheringCapability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<TetheringCapability>(result__)
        })
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::Connectivity::ConnectionProfile>>(profile: Param0) -> ::windows::runtime::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<NetworkOperatorTetheringManager>(result__)
        })
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfileWithTargetAdapter<'a, Param0: ::windows::runtime::IntoParam<'a, super::Connectivity::ConnectionProfile>, Param1: ::windows::runtime::IntoParam<'a, super::Connectivity::NetworkAdapter>>(profile: Param0, adapter: Param1) -> ::windows::runtime::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), profile.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<NetworkOperatorTetheringManager>(result__)
        })
    }
    pub fn IsNoConnectionsTimeoutEnabled() -> ::windows::runtime::Result<bool> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn EnableNoConnectionsTimeout() -> ::windows::runtime::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn EnableNoConnectionsTimeoutAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn DisableNoConnectionsTimeout() -> ::windows::runtime::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn DisableNoConnectionsTimeoutAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn INetworkOperatorTetheringManagerStatics<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn INetworkOperatorTetheringManagerStatics2<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn INetworkOperatorTetheringManagerStatics3<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn INetworkOperatorTetheringManagerStatics4<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorTetheringManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringManager;{d45a8da0-0e86-4d98-8ba4-dd70d4b764d3})");
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3562704288, 3718, 19864, [139, 164, 221, 112, 212, 183, 100, 211]);
}
impl ::windows::runtime::RuntimeName for NetworkOperatorTetheringManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringManager";
}
impl ::std::convert::From<NetworkOperatorTetheringManager> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorTetheringManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringManager> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorTetheringManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NetworkOperatorTetheringManager> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorTetheringManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringManager> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorTetheringManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NetworkOperatorTetheringOperationResult(::windows::runtime::IInspectable);
impl NetworkOperatorTetheringOperationResult {
    pub fn Status(&self) -> ::windows::runtime::Result<TetheringOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: TetheringOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TetheringOperationStatus>(result__)
        }
    }
    pub fn AdditionalErrorMessage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NetworkOperatorTetheringOperationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringOperationResult;{ebd203a1-01ba-476d-b4b3-bf3d12c8f80c})");
}
unsafe impl ::windows::runtime::Interface for NetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3956409249, 442, 18285, [180, 179, 191, 61, 18, 200, 248, 12]);
}
impl ::windows::runtime::RuntimeName for NetworkOperatorTetheringOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringOperationResult";
}
impl ::std::convert::From<NetworkOperatorTetheringOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: NetworkOperatorTetheringOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &NetworkOperatorTetheringOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NetworkOperatorTetheringOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: NetworkOperatorTetheringOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NetworkOperatorTetheringOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: &NetworkOperatorTetheringOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct NetworkOperatorsFdnContract(pub u8);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NetworkRegistrationState(pub i32);
impl NetworkRegistrationState {
    pub const None: NetworkRegistrationState = NetworkRegistrationState(0i32);
    pub const Deregistered: NetworkRegistrationState = NetworkRegistrationState(1i32);
    pub const Searching: NetworkRegistrationState = NetworkRegistrationState(2i32);
    pub const Home: NetworkRegistrationState = NetworkRegistrationState(3i32);
    pub const Roaming: NetworkRegistrationState = NetworkRegistrationState(4i32);
    pub const Partner: NetworkRegistrationState = NetworkRegistrationState(5i32);
    pub const Denied: NetworkRegistrationState = NetworkRegistrationState(6i32);
}
impl ::std::convert::From<i32> for NetworkRegistrationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NetworkRegistrationState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NetworkRegistrationState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkRegistrationState;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: ProfileMediaType = ProfileMediaType(0i32);
    pub const Wwan: ProfileMediaType = ProfileMediaType(1i32);
}
impl ::std::convert::From<i32> for ProfileMediaType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProfileMediaType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProfileMediaType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ProfileMediaType;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct ProfileUsage {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::DateTime,
}
#[cfg(feature = "Foundation")]
impl ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::std::default::Default for ProfileUsage {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::fmt::Debug for ProfileUsage {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ProfileUsage").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::cmp::PartialEq for ProfileUsage {
    fn eq(&self, other: &Self) -> bool {
        self.UsageInMegabytes == other.UsageInMegabytes && self.LastSyncTime == other.LastSyncTime
    }
}
#[cfg(feature = "Foundation")]
impl ::std::cmp::Eq for ProfileUsage {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Abi for ProfileUsage {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for ProfileUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ProfileUsage;u4;struct(Windows.Foundation.DateTime;i8))");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProvisionFromXmlDocumentResults(::windows::runtime::IInspectable);
impl ProvisionFromXmlDocumentResults {
    pub fn AllElementsProvisioned(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ProvisionResultsXml(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProvisionFromXmlDocumentResults {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ProvisionFromXmlDocumentResults;{217700e0-8203-11df-adb9-f4ce462d9137})");
}
unsafe impl ::windows::runtime::Interface for ProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33283, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
impl ::windows::runtime::RuntimeName for ProvisionFromXmlDocumentResults {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionFromXmlDocumentResults";
}
impl ::std::convert::From<ProvisionFromXmlDocumentResults> for ::windows::runtime::IUnknown {
    fn from(value: ProvisionFromXmlDocumentResults) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProvisionFromXmlDocumentResults> for ::windows::runtime::IUnknown {
    fn from(value: &ProvisionFromXmlDocumentResults) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProvisionFromXmlDocumentResults> for ::windows::runtime::IInspectable {
    fn from(value: ProvisionFromXmlDocumentResults) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProvisionFromXmlDocumentResults> for ::windows::runtime::IInspectable {
    fn from(value: &ProvisionFromXmlDocumentResults) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProvisionedProfile(::windows::runtime::IInspectable);
impl ProvisionedProfile {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn UpdateCost(&self, value: super::Connectivity::NetworkCostType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateUsage<'a, Param0: ::windows::runtime::IntoParam<'a, ProfileUsage>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProvisionedProfile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ProvisionedProfile;{217700e0-8202-11df-adb9-f4ce462d9137})");
}
unsafe impl ::windows::runtime::Interface for ProvisionedProfile {
    type Vtable = IProvisionedProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33282, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
impl ::windows::runtime::RuntimeName for ProvisionedProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionedProfile";
}
impl ::std::convert::From<ProvisionedProfile> for ::windows::runtime::IUnknown {
    fn from(value: ProvisionedProfile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProvisionedProfile> for ::windows::runtime::IUnknown {
    fn from(value: &ProvisionedProfile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProvisionedProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProvisionedProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProvisionedProfile> for ::windows::runtime::IInspectable {
    fn from(value: ProvisionedProfile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProvisionedProfile> for ::windows::runtime::IInspectable {
    fn from(value: &ProvisionedProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProvisionedProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProvisionedProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProvisioningAgent(::windows::runtime::IInspectable);
impl ProvisioningAgent {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProvisioningAgent, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProvisionFromXmlDocumentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, provisioningxmldocument: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), provisioningxmldocument.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>>(result__)
        }
    }
    pub fn GetProvisionedProfile<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, mediatype: ProfileMediaType, profilename: Param1) -> ::windows::runtime::Result<ProvisionedProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mediatype, profilename.into_param().abi(), &mut result__).from_abi::<ProvisionedProfile>(result__)
        }
    }
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkaccountid: Param0) -> ::windows::runtime::Result<ProvisioningAgent> {
        Self::IProvisioningAgentStaticMethods(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<ProvisioningAgent>(result__)
        })
    }
    pub fn IProvisioningAgentStaticMethods<R, F: FnOnce(&IProvisioningAgentStaticMethods) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProvisioningAgent, IProvisioningAgentStaticMethods> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProvisioningAgent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ProvisioningAgent;{217700e0-8201-11df-adb9-f4ce462d9137})");
}
unsafe impl ::windows::runtime::Interface for ProvisioningAgent {
    type Vtable = IProvisioningAgent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561447136, 33281, 4575, [173, 185, 244, 206, 70, 45, 145, 55]);
}
impl ::windows::runtime::RuntimeName for ProvisioningAgent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisioningAgent";
}
impl ::std::convert::From<ProvisioningAgent> for ::windows::runtime::IUnknown {
    fn from(value: ProvisioningAgent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProvisioningAgent> for ::windows::runtime::IUnknown {
    fn from(value: &ProvisioningAgent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProvisioningAgent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProvisioningAgent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProvisioningAgent> for ::windows::runtime::IInspectable {
    fn from(value: ProvisioningAgent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProvisioningAgent> for ::windows::runtime::IInspectable {
    fn from(value: &ProvisioningAgent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProvisioningAgent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProvisioningAgent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TetheringCapability(pub i32);
impl TetheringCapability {
    pub const Enabled: TetheringCapability = TetheringCapability(0i32);
    pub const DisabledByGroupPolicy: TetheringCapability = TetheringCapability(1i32);
    pub const DisabledByHardwareLimitation: TetheringCapability = TetheringCapability(2i32);
    pub const DisabledByOperator: TetheringCapability = TetheringCapability(3i32);
    pub const DisabledBySku: TetheringCapability = TetheringCapability(4i32);
    pub const DisabledByRequiredAppNotInstalled: TetheringCapability = TetheringCapability(5i32);
    pub const DisabledDueToUnknownCause: TetheringCapability = TetheringCapability(6i32);
    pub const DisabledBySystemCapability: TetheringCapability = TetheringCapability(7i32);
}
impl ::std::convert::From<i32> for TetheringCapability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TetheringCapability {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TetheringCapability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringCapability;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TetheringEntitlementCheckTriggerDetails(::windows::runtime::IInspectable);
impl TetheringEntitlementCheckTriggerDetails {
    pub fn NetworkAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn AllowTethering(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn DenyTethering<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, entitlementfailurereason: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), entitlementfailurereason.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TetheringEntitlementCheckTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.TetheringEntitlementCheckTriggerDetails;{03c65e9d-5926-41f3-a94e-b50926fc421b})");
}
unsafe impl ::windows::runtime::Interface for TetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(63331997, 22822, 16883, [169, 78, 181, 9, 38, 252, 66, 27]);
}
impl ::windows::runtime::RuntimeName for TetheringEntitlementCheckTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.TetheringEntitlementCheckTriggerDetails";
}
impl ::std::convert::From<TetheringEntitlementCheckTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: TetheringEntitlementCheckTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TetheringEntitlementCheckTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &TetheringEntitlementCheckTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TetheringEntitlementCheckTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: TetheringEntitlementCheckTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TetheringEntitlementCheckTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &TetheringEntitlementCheckTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TetheringEntitlementCheckTriggerDetails {}
unsafe impl ::std::marker::Sync for TetheringEntitlementCheckTriggerDetails {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: TetheringOperationStatus = TetheringOperationStatus(0i32);
    pub const Unknown: TetheringOperationStatus = TetheringOperationStatus(1i32);
    pub const MobileBroadbandDeviceOff: TetheringOperationStatus = TetheringOperationStatus(2i32);
    pub const WiFiDeviceOff: TetheringOperationStatus = TetheringOperationStatus(3i32);
    pub const EntitlementCheckTimeout: TetheringOperationStatus = TetheringOperationStatus(4i32);
    pub const EntitlementCheckFailure: TetheringOperationStatus = TetheringOperationStatus(5i32);
    pub const OperationInProgress: TetheringOperationStatus = TetheringOperationStatus(6i32);
    pub const BluetoothDeviceOff: TetheringOperationStatus = TetheringOperationStatus(7i32);
    pub const NetworkLimitedConnectivity: TetheringOperationStatus = TetheringOperationStatus(8i32);
}
impl ::std::convert::From<i32> for TetheringOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TetheringOperationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TetheringOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationStatus;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: TetheringOperationalState = TetheringOperationalState(0i32);
    pub const On: TetheringOperationalState = TetheringOperationalState(1i32);
    pub const Off: TetheringOperationalState = TetheringOperationalState(2i32);
    pub const InTransition: TetheringOperationalState = TetheringOperationalState(3i32);
}
impl ::std::convert::From<i32> for TetheringOperationalState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TetheringOperationalState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TetheringOperationalState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationalState;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: TetheringWiFiBand = TetheringWiFiBand(0i32);
    pub const TwoPointFourGigahertz: TetheringWiFiBand = TetheringWiFiBand(1i32);
    pub const FiveGigahertz: TetheringWiFiBand = TetheringWiFiBand(2i32);
}
impl ::std::convert::From<i32> for TetheringWiFiBand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TetheringWiFiBand {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TetheringWiFiBand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringWiFiBand;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: UiccAccessCondition = UiccAccessCondition(0i32);
    pub const Pin1: UiccAccessCondition = UiccAccessCondition(1i32);
    pub const Pin2: UiccAccessCondition = UiccAccessCondition(2i32);
    pub const Pin3: UiccAccessCondition = UiccAccessCondition(3i32);
    pub const Pin4: UiccAccessCondition = UiccAccessCondition(4i32);
    pub const Administrative5: UiccAccessCondition = UiccAccessCondition(5i32);
    pub const Administrative6: UiccAccessCondition = UiccAccessCondition(6i32);
    pub const NeverAllowed: UiccAccessCondition = UiccAccessCondition(7i32);
}
impl ::std::convert::From<i32> for UiccAccessCondition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UiccAccessCondition {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UiccAccessCondition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAccessCondition;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: UiccAppKind = UiccAppKind(0i32);
    pub const MF: UiccAppKind = UiccAppKind(1i32);
    pub const MFSim: UiccAppKind = UiccAppKind(2i32);
    pub const MFRuim: UiccAppKind = UiccAppKind(3i32);
    pub const USim: UiccAppKind = UiccAppKind(4i32);
    pub const CSim: UiccAppKind = UiccAppKind(5i32);
    pub const ISim: UiccAppKind = UiccAppKind(6i32);
}
impl ::std::convert::From<i32> for UiccAppKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UiccAppKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UiccAppKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppKind;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: UiccAppRecordKind = UiccAppRecordKind(0i32);
    pub const Transparent: UiccAppRecordKind = UiccAppRecordKind(1i32);
    pub const RecordOriented: UiccAppRecordKind = UiccAppRecordKind(2i32);
}
impl ::std::convert::From<i32> for UiccAppRecordKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UiccAppRecordKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UiccAppRecordKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppRecordKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UssdMessage(::windows::runtime::IInspectable);
impl UssdMessage {
    pub fn DataCodingScheme(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetDataCodingScheme(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetPayload(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn SetPayload(&self, value: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn PayloadAsText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetPayloadAsText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CreateMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(messagetext: Param0) -> ::windows::runtime::Result<UssdMessage> {
        Self::IUssdMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), messagetext.into_param().abi(), &mut result__).from_abi::<UssdMessage>(result__)
        })
    }
    pub fn IUssdMessageFactory<R, F: FnOnce(&IUssdMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UssdMessage, IUssdMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UssdMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.UssdMessage;{2f9acf82-2004-4d5d-bf81-2aba1b4be4a8})");
}
unsafe impl ::windows::runtime::Interface for UssdMessage {
    type Vtable = IUssdMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 8196, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
impl ::windows::runtime::RuntimeName for UssdMessage {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdMessage";
}
impl ::std::convert::From<UssdMessage> for ::windows::runtime::IUnknown {
    fn from(value: UssdMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UssdMessage> for ::windows::runtime::IUnknown {
    fn from(value: &UssdMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UssdMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UssdMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UssdMessage> for ::windows::runtime::IInspectable {
    fn from(value: UssdMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UssdMessage> for ::windows::runtime::IInspectable {
    fn from(value: &UssdMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UssdMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UssdMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UssdMessage {}
unsafe impl ::std::marker::Sync for UssdMessage {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UssdReply(::windows::runtime::IInspectable);
impl UssdReply {
    pub fn ResultCode(&self) -> ::windows::runtime::Result<UssdResultCode> {
        let this = self;
        unsafe {
            let mut result__: UssdResultCode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UssdResultCode>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::runtime::Result<UssdMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UssdMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UssdReply {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.UssdReply;{2f9acf82-2005-4d5d-bf81-2aba1b4be4a8})");
}
unsafe impl ::windows::runtime::Interface for UssdReply {
    type Vtable = IUssdReply_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 8197, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
impl ::windows::runtime::RuntimeName for UssdReply {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdReply";
}
impl ::std::convert::From<UssdReply> for ::windows::runtime::IUnknown {
    fn from(value: UssdReply) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UssdReply> for ::windows::runtime::IUnknown {
    fn from(value: &UssdReply) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UssdReply {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UssdReply {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UssdReply> for ::windows::runtime::IInspectable {
    fn from(value: UssdReply) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UssdReply> for ::windows::runtime::IInspectable {
    fn from(value: &UssdReply) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UssdReply {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UssdReply {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: UssdResultCode = UssdResultCode(0i32);
    pub const ActionRequired: UssdResultCode = UssdResultCode(1i32);
    pub const Terminated: UssdResultCode = UssdResultCode(2i32);
    pub const OtherLocalClient: UssdResultCode = UssdResultCode(3i32);
    pub const OperationNotSupported: UssdResultCode = UssdResultCode(4i32);
    pub const NetworkTimeout: UssdResultCode = UssdResultCode(5i32);
}
impl ::std::convert::From<i32> for UssdResultCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UssdResultCode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UssdResultCode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UssdResultCode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UssdSession(::windows::runtime::IInspectable);
impl UssdSession {
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAndGetReplyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, UssdMessage>>(&self, message: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UssdReply>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UssdReply>>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkaccountid: Param0) -> ::windows::runtime::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<UssdSession>(result__)
        })
    }
    pub fn CreateFromNetworkInterfaceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkinterfaceid: Param0) -> ::windows::runtime::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), networkinterfaceid.into_param().abi(), &mut result__).from_abi::<UssdSession>(result__)
        })
    }
    pub fn IUssdSessionStatics<R, F: FnOnce(&IUssdSessionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UssdSession, IUssdSessionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UssdSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.UssdSession;{2f9acf82-2002-4d5d-bf81-2aba1b4be4a8})");
}
unsafe impl ::windows::runtime::Interface for UssdSession {
    type Vtable = IUssdSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798674818, 8194, 19805, [191, 129, 42, 186, 27, 75, 228, 168]);
}
impl ::windows::runtime::RuntimeName for UssdSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdSession";
}
impl ::std::convert::From<UssdSession> for ::windows::runtime::IUnknown {
    fn from(value: UssdSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UssdSession> for ::windows::runtime::IUnknown {
    fn from(value: &UssdSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UssdSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UssdSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UssdSession> for ::windows::runtime::IInspectable {
    fn from(value: UssdSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UssdSession> for ::windows::runtime::IInspectable {
    fn from(value: &UssdSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UssdSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UssdSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
