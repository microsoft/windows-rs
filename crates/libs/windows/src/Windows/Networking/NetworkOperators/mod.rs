#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const NewRadioNonStandalone: Self = Self(64u32);
    pub const NewRadioStandalone: Self = Self(128u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for DataClasses {}
impl ::core::clone::Clone for DataClasses {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataClasses {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataClasses {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataClasses {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataClasses").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DataClasses {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DataClasses {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DataClasses {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DataClasses {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DataClasses {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DataClasses {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.DataClasses;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESim(::windows::core::IUnknown);
impl ESim {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailableMemoryInBytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailableMemoryInBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Eid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Eid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn FirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirmwareVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MobileBroadbandModemDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MobileBroadbandModemDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Policy(&self) -> ::windows::core::Result<ESimPolicy> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Policy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ESimState> {
        let this = self;
        unsafe {
            let mut result__: ESimState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProfiles)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ESimProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, profileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteProfileAsync)(::core::mem::transmute_copy(this), profileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadProfileMetadataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activationcode: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadProfileMetadataAsync)(::core::mem::transmute_copy(this), activationcode.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResetAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProfileChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESim, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProfileChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProfileChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProfileChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Discover(&self) -> ::windows::core::Result<ESimDiscoverResult> {
        let this = &::windows::core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Discover)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimDiscoverResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DiscoverWithServerAddressAndMatchingId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, serveraddress: Param0, matchingid: Param1) -> ::windows::core::Result<ESimDiscoverResult> {
        let this = &::windows::core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscoverWithServerAddressAndMatchingId)(::core::mem::transmute_copy(this), serveraddress.into_param().abi(), matchingid.into_param().abi(), &mut result__).from_abi::<ESimDiscoverResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DiscoverAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &::windows::core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscoverAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DiscoverWithServerAddressAndMatchingIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, serveraddress: Param0, matchingid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &::windows::core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscoverWithServerAddressAndMatchingIdAsync)(::core::mem::transmute_copy(this), serveraddress.into_param().abi(), matchingid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for ESim {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESim {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESim {}
impl ::core::fmt::Debug for ESim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESim").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESim {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESim;{6f6e6e26-f123-437d-8ced-dc1d2bc0c3a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESim {
    type Vtable = IESim_Vtbl;
    const IID: ::windows::core::GUID = <IESim as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESim {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESim";
}
impl ::core::convert::From<ESim> for ::windows::core::IUnknown {
    fn from(value: ESim) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESim> for ::windows::core::IUnknown {
    fn from(value: &ESim) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESim {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESim {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESim> for ::windows::core::IInspectable {
    fn from(value: ESim) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESim> for ::windows::core::IInspectable {
    fn from(value: &ESim) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESim {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESim {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESim {}
unsafe impl ::core::marker::Sync for ESim {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimAddedEventArgs(::windows::core::IUnknown);
impl ESimAddedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ESim(&self) -> ::windows::core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ESim)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESim>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimAddedEventArgs {}
impl ::core::fmt::Debug for ESimAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimAddedEventArgs;{38bd0a58-4d5a-4d08-8da7-e73eff369ddd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IESimAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimAddedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimAddedEventArgs";
}
impl ::core::convert::From<ESimAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ESimAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ESimAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ESimAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ESimAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimAddedEventArgs {}
unsafe impl ::core::marker::Sync for ESimAddedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: Self = Self(0i32);
    pub const OnAction: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimAuthenticationPreference {}
impl ::core::clone::Clone for ESimAuthenticationPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimAuthenticationPreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimAuthenticationPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimAuthenticationPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimAuthenticationPreference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimAuthenticationPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimAuthenticationPreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimDiscoverEvent(::windows::core::IUnknown);
impl ESimDiscoverEvent {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MatchingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MatchingId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RspServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RspServerAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimDiscoverEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimDiscoverEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimDiscoverEvent {}
impl ::core::fmt::Debug for ESimDiscoverEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimDiscoverEvent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimDiscoverEvent;{e59ac3e3-39bc-5f6f-9321-0d4a182d261b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_Vtbl;
    const IID: ::windows::core::GUID = <IESimDiscoverEvent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimDiscoverEvent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverEvent";
}
impl ::core::convert::From<ESimDiscoverEvent> for ::windows::core::IUnknown {
    fn from(value: ESimDiscoverEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimDiscoverEvent> for ::windows::core::IUnknown {
    fn from(value: &ESimDiscoverEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimDiscoverEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimDiscoverEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimDiscoverEvent> for ::windows::core::IInspectable {
    fn from(value: ESimDiscoverEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimDiscoverEvent> for ::windows::core::IInspectable {
    fn from(value: &ESimDiscoverEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimDiscoverEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimDiscoverEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimDiscoverEvent {}
unsafe impl ::core::marker::Sync for ESimDiscoverEvent {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimDiscoverResult(::windows::core::IUnknown);
impl ESimDiscoverResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Events(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Events)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ESimDiscoverResultKind> {
        let this = self;
        unsafe {
            let mut result__: ESimDiscoverResultKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimDiscoverResultKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProfileMetadata(&self) -> ::windows::core::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProfileMetadata)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileMetadata>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimOperationResult>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimDiscoverResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimDiscoverResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimDiscoverResult {}
impl ::core::fmt::Debug for ESimDiscoverResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimDiscoverResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimDiscoverResult;{56b4bb5e-ab2f-5ac6-b359-dd5a8e237926})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimDiscoverResult {
    type Vtable = IESimDiscoverResult_Vtbl;
    const IID: ::windows::core::GUID = <IESimDiscoverResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimDiscoverResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverResult";
}
impl ::core::convert::From<ESimDiscoverResult> for ::windows::core::IUnknown {
    fn from(value: ESimDiscoverResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimDiscoverResult> for ::windows::core::IUnknown {
    fn from(value: &ESimDiscoverResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimDiscoverResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimDiscoverResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimDiscoverResult> for ::windows::core::IInspectable {
    fn from(value: ESimDiscoverResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimDiscoverResult> for ::windows::core::IInspectable {
    fn from(value: &ESimDiscoverResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimDiscoverResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimDiscoverResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimDiscoverResult {}
unsafe impl ::core::marker::Sync for ESimDiscoverResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: Self = Self(0i32);
    pub const Events: Self = Self(1i32);
    pub const ProfileMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimDiscoverResultKind {}
impl ::core::clone::Clone for ESimDiscoverResultKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimDiscoverResultKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimDiscoverResultKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimDiscoverResultKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverResultKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimDiscoverResultKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimDiscoverResultKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimDownloadProfileMetadataResult(::windows::core::IUnknown);
impl ESimDownloadProfileMetadataResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimOperationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProfileMetadata(&self) -> ::windows::core::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProfileMetadata)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileMetadata>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimDownloadProfileMetadataResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimDownloadProfileMetadataResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimDownloadProfileMetadataResult {}
impl ::core::fmt::Debug for ESimDownloadProfileMetadataResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDownloadProfileMetadataResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimDownloadProfileMetadataResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimDownloadProfileMetadataResult;{c4234d9e-5ad6-426d-8d00-4434f449afec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_Vtbl;
    const IID: ::windows::core::GUID = <IESimDownloadProfileMetadataResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimDownloadProfileMetadataResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDownloadProfileMetadataResult";
}
impl ::core::convert::From<ESimDownloadProfileMetadataResult> for ::windows::core::IUnknown {
    fn from(value: ESimDownloadProfileMetadataResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimDownloadProfileMetadataResult> for ::windows::core::IUnknown {
    fn from(value: &ESimDownloadProfileMetadataResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimDownloadProfileMetadataResult> for ::windows::core::IInspectable {
    fn from(value: ESimDownloadProfileMetadataResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimDownloadProfileMetadataResult> for ::windows::core::IInspectable {
    fn from(value: &ESimDownloadProfileMetadataResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimDownloadProfileMetadataResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimDownloadProfileMetadataResult {}
unsafe impl ::core::marker::Sync for ESimDownloadProfileMetadataResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct ESimManager {}
impl ESimManager {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ServiceInfo() -> ::windows::core::Result<ESimServiceInfo> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimServiceInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn TryCreateESimWatcher() -> ::windows::core::Result<ESimWatcher> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateESimWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceInfoChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IESimManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveServiceInfoChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IESimManagerStatics<R, F: FnOnce(&IESimManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ESimManager, IESimManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ESimManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimManager";
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimOperationResult(::windows::core::IUnknown);
impl ESimOperationResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<ESimOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: ESimOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimOperationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimOperationResult {}
impl ::core::fmt::Debug for ESimOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimOperationResult;{a67b63b1-309b-4e77-9e7e-cd93f1ddc7b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimOperationResult {
    type Vtable = IESimOperationResult_Vtbl;
    const IID: ::windows::core::GUID = <IESimOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimOperationResult";
}
impl ::core::convert::From<ESimOperationResult> for ::windows::core::IUnknown {
    fn from(value: ESimOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimOperationResult> for ::windows::core::IUnknown {
    fn from(value: &ESimOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimOperationResult> for ::windows::core::IInspectable {
    fn from(value: ESimOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimOperationResult> for ::windows::core::IInspectable {
    fn from(value: &ESimOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimOperationResult {}
unsafe impl ::core::marker::Sync for ESimOperationResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAuthorized: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const PolicyViolation: Self = Self(3i32);
    pub const InsufficientSpaceOnCard: Self = Self(4i32);
    pub const ServerFailure: Self = Self(5i32);
    pub const ServerNotReachable: Self = Self(6i32);
    pub const TimeoutWaitingForUserConsent: Self = Self(7i32);
    pub const IncorrectConfirmationCode: Self = Self(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: Self = Self(9i32);
    pub const CardRemoved: Self = Self(10i32);
    pub const CardBusy: Self = Self(11i32);
    pub const Other: Self = Self(12i32);
    pub const CardGeneralFailure: Self = Self(13i32);
    pub const ConfirmationCodeMissing: Self = Self(14i32);
    pub const InvalidMatchingId: Self = Self(15i32);
    pub const NoEligibleProfileForThisDevice: Self = Self(16i32);
    pub const OperationAborted: Self = Self(17i32);
    pub const EidMismatch: Self = Self(18i32);
    pub const ProfileNotAvailableForNewBinding: Self = Self(19i32);
    pub const ProfileNotReleasedByOperator: Self = Self(20i32);
    pub const OperationProhibitedByProfileClass: Self = Self(21i32);
    pub const ProfileNotPresent: Self = Self(22i32);
    pub const NoCorrespondingRequest: Self = Self(23i32);
    pub const TimeoutWaitingForResponse: Self = Self(24i32);
    pub const IccidAlreadyExists: Self = Self(25i32);
    pub const ProfileProcessingError: Self = Self(26i32);
    pub const ServerNotTrusted: Self = Self(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: Self = Self(28i32);
}
impl ::core::marker::Copy for ESimOperationStatus {}
impl ::core::clone::Clone for ESimOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimPolicy(::windows::core::IUnknown);
impl ESimPolicy {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ShouldEnableManagingUi(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldEnableManagingUi)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimPolicy {}
impl ::core::fmt::Debug for ESimPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimPolicy;{41e1b99d-cf7e-4315-882b-6f1e74b0d38f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimPolicy {
    type Vtable = IESimPolicy_Vtbl;
    const IID: ::windows::core::GUID = <IESimPolicy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimPolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimPolicy";
}
impl ::core::convert::From<ESimPolicy> for ::windows::core::IUnknown {
    fn from(value: ESimPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimPolicy> for ::windows::core::IUnknown {
    fn from(value: &ESimPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimPolicy> for ::windows::core::IInspectable {
    fn from(value: ESimPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimPolicy> for ::windows::core::IInspectable {
    fn from(value: &ESimPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimPolicy {}
unsafe impl ::core::marker::Sync for ESimPolicy {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimProfile(::windows::core::IUnknown);
impl ESimProfile {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Class(&self) -> ::windows::core::Result<ESimProfileClass> {
        let this = self;
        unsafe {
            let mut result__: ESimProfileClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Class)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileClass>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Nickname)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Policy(&self) -> ::windows::core::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Policy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfilePolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderIcon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ESimProfileState> {
        let this = self;
        unsafe {
            let mut result__: ESimProfileState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNicknameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, newnickname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetNicknameAsync)(::core::mem::transmute_copy(this), newnickname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimProfile {}
impl ::core::fmt::Debug for ESimProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimProfile;{ee1e7880-06a9-4027-b4f8-ddb23d7810e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimProfile {
    type Vtable = IESimProfile_Vtbl;
    const IID: ::windows::core::GUID = <IESimProfile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfile";
}
impl ::core::convert::From<ESimProfile> for ::windows::core::IUnknown {
    fn from(value: ESimProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimProfile> for ::windows::core::IUnknown {
    fn from(value: &ESimProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimProfile> for ::windows::core::IInspectable {
    fn from(value: ESimProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimProfile> for ::windows::core::IInspectable {
    fn from(value: &ESimProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimProfile {}
unsafe impl ::core::marker::Sync for ESimProfile {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: Self = Self(0i32);
    pub const Test: Self = Self(1i32);
    pub const Provisioning: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimProfileClass {}
impl ::core::clone::Clone for ESimProfileClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimProfileClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimProfileClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimProfileClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimProfileClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct ESimProfileInstallProgress {
    pub TotalSizeInBytes: i32,
    pub InstalledSizeInBytes: i32,
}
impl ::core::marker::Copy for ESimProfileInstallProgress {}
impl ::core::clone::Clone for ESimProfileInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ESimProfileInstallProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ESimProfileInstallProgress").field("TotalSizeInBytes", &self.TotalSizeInBytes).field("InstalledSizeInBytes", &self.InstalledSizeInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for ESimProfileInstallProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ESimProfileInstallProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ESimProfileInstallProgress;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ESimProfileInstallProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ESimProfileInstallProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for ESimProfileInstallProgress {}
impl ::core::default::Default for ESimProfileInstallProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimProfileMetadata(::windows::core::IUnknown);
impl ESimProfileMetadata {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsConfirmationCodeRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConfirmationCodeRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Policy(&self) -> ::windows::core::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Policy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfilePolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderIcon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ESimProfileMetadataState> {
        let this = self;
        unsafe {
            let mut result__: ESimProfileMetadataState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimProfileMetadataState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DenyInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DenyInstallAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConfirmInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConfirmInstallAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConfirmInstallWithConfirmationCodeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, confirmationcode: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConfirmInstallWithConfirmationCodeAsync)(::core::mem::transmute_copy(this), confirmationcode.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PostponeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PostponeInstallAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ESimOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ESimProfileMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimProfileMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimProfileMetadata {}
impl ::core::fmt::Debug for ESimProfileMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimProfileMetadata {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimProfileMetadata;{ed25831f-90db-498d-a7b4-ebce807d3c23})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimProfileMetadata {
    type Vtable = IESimProfileMetadata_Vtbl;
    const IID: ::windows::core::GUID = <IESimProfileMetadata as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimProfileMetadata {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfileMetadata";
}
impl ::core::convert::From<ESimProfileMetadata> for ::windows::core::IUnknown {
    fn from(value: ESimProfileMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimProfileMetadata> for ::windows::core::IUnknown {
    fn from(value: &ESimProfileMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimProfileMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimProfileMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimProfileMetadata> for ::windows::core::IInspectable {
    fn from(value: ESimProfileMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimProfileMetadata> for ::windows::core::IInspectable {
    fn from(value: &ESimProfileMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimProfileMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimProfileMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimProfileMetadata {}
unsafe impl ::core::marker::Sync for ESimProfileMetadata {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: Self = Self(0i32);
    pub const WaitingForInstall: Self = Self(1i32);
    pub const Downloading: Self = Self(2i32);
    pub const Installing: Self = Self(3i32);
    pub const Expired: Self = Self(4i32);
    pub const RejectingDownload: Self = Self(5i32);
    pub const NoLongerAvailable: Self = Self(6i32);
    pub const DeniedByPolicy: Self = Self(7i32);
}
impl ::core::marker::Copy for ESimProfileMetadataState {}
impl ::core::clone::Clone for ESimProfileMetadataState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimProfileMetadataState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimProfileMetadataState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimProfileMetadataState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileMetadataState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimProfileMetadataState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileMetadataState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimProfilePolicy(::windows::core::IUnknown);
impl ESimProfilePolicy {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CanDelete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDelete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CanDisable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDisable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsManagedByEnterprise(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsManagedByEnterprise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimProfilePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimProfilePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimProfilePolicy {}
impl ::core::fmt::Debug for ESimProfilePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfilePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimProfilePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimProfilePolicy;{e6dd0f1d-9c5c-46c5-a289-a948999bf062})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimProfilePolicy {
    type Vtable = IESimProfilePolicy_Vtbl;
    const IID: ::windows::core::GUID = <IESimProfilePolicy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimProfilePolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfilePolicy";
}
impl ::core::convert::From<ESimProfilePolicy> for ::windows::core::IUnknown {
    fn from(value: ESimProfilePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimProfilePolicy> for ::windows::core::IUnknown {
    fn from(value: &ESimProfilePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimProfilePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimProfilePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimProfilePolicy> for ::windows::core::IInspectable {
    fn from(value: ESimProfilePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimProfilePolicy> for ::windows::core::IInspectable {
    fn from(value: &ESimProfilePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimProfilePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimProfilePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimProfilePolicy {}
unsafe impl ::core::marker::Sync for ESimProfilePolicy {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const Deleted: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimProfileState {}
impl ::core::clone::Clone for ESimProfileState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimProfileState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimProfileState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimProfileState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimProfileState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimRemovedEventArgs(::windows::core::IUnknown);
impl ESimRemovedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ESim(&self) -> ::windows::core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ESim)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESim>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimRemovedEventArgs {}
impl ::core::fmt::Debug for ESimRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimRemovedEventArgs;{dec5277b-2fd9-4ed9-8376-d9b5e41278a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IESimRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimRemovedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimRemovedEventArgs";
}
impl ::core::convert::From<ESimRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ESimRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ESimRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ESimRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ESimRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimRemovedEventArgs {}
unsafe impl ::core::marker::Sync for ESimRemovedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimServiceInfo(::windows::core::IUnknown);
impl ESimServiceInfo {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AuthenticationPreference(&self) -> ::windows::core::Result<ESimAuthenticationPreference> {
        let this = self;
        unsafe {
            let mut result__: ESimAuthenticationPreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationPreference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimAuthenticationPreference>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsESimUiEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsESimUiEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimServiceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimServiceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimServiceInfo {}
impl ::core::fmt::Debug for ESimServiceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimServiceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimServiceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimServiceInfo;{f16aabcf-7f59-4a51-8494-bd89d5ff50ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimServiceInfo {
    type Vtable = IESimServiceInfo_Vtbl;
    const IID: ::windows::core::GUID = <IESimServiceInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimServiceInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimServiceInfo";
}
impl ::core::convert::From<ESimServiceInfo> for ::windows::core::IUnknown {
    fn from(value: ESimServiceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimServiceInfo> for ::windows::core::IUnknown {
    fn from(value: &ESimServiceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimServiceInfo> for ::windows::core::IInspectable {
    fn from(value: ESimServiceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimServiceInfo> for ::windows::core::IInspectable {
    fn from(value: &ESimServiceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimServiceInfo {}
unsafe impl ::core::marker::Sync for ESimServiceInfo {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Removed: Self = Self(2i32);
    pub const Busy: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimState {}
impl ::core::clone::Clone for ESimState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimUpdatedEventArgs(::windows::core::IUnknown);
impl ESimUpdatedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ESim(&self) -> ::windows::core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ESim)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESim>(result__)
        }
    }
}
impl ::core::clone::Clone for ESimUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimUpdatedEventArgs {}
impl ::core::fmt::Debug for ESimUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimUpdatedEventArgs;{4c125cec-508d-4b88-83cb-68bef8168d12})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IESimUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimUpdatedEventArgs";
}
impl ::core::convert::From<ESimUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ESimUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ESimUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ESimUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ESimUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for ESimUpdatedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimWatcher(::windows::core::IUnknown);
impl ESimWatcher {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<ESimWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: ESimWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ESimWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Updated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ESimWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ESimWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimWatcher {}
impl ::core::fmt::Debug for ESimWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ESimWatcher;{c1f84ceb-a28d-4fbf-9771-6e31b81ccf22})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ESimWatcher {
    type Vtable = IESimWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IESimWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ESimWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimWatcher";
}
impl ::core::convert::From<ESimWatcher> for ::windows::core::IUnknown {
    fn from(value: ESimWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimWatcher> for ::windows::core::IUnknown {
    fn from(value: &ESimWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ESimWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ESimWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ESimWatcher> for ::windows::core::IInspectable {
    fn from(value: ESimWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ESimWatcher> for ::windows::core::IInspectable {
    fn from(value: &ESimWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ESimWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ESimWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ESimWatcher {}
unsafe impl ::core::marker::Sync for ESimWatcher {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
}
impl ::core::marker::Copy for ESimWatcherStatus {}
impl ::core::clone::Clone for ESimWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESimWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESimWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ESimWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct FdnAccessManager {}
impl FdnAccessManager {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnlockAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contactlistid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFdnAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestUnlockAsync)(::core::mem::transmute_copy(this), contactlistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFdnAccessManagerStatics<R, F: FnOnce(&IFdnAccessManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FdnAccessManager, IFdnAccessManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for FdnAccessManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.FdnAccessManager";
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct HotspotAuthenticationContext(::windows::core::IUnknown);
impl HotspotAuthenticationContext {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn WirelessNetworkId(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WirelessNetworkId)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAdapter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Connectivity::NetworkAdapter>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RedirectMessageUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RedirectMessageUrl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn RedirectMessageXml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RedirectMessageXml)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationUrl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IssueCredentials<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, username: Param0, password: Param1, extraparameters: Param2, markasmanualconnectonfailure: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).IssueCredentials)(::core::mem::transmute_copy(this), username.into_param().abi(), password.into_param().abi(), extraparameters.into_param().abi(), markasmanualconnectonfailure).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AbortAuthentication(&self, markasmanual: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AbortAuthentication)(::core::mem::transmute_copy(this), markasmanual).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SkipAuthentication(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SkipAuthentication)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn TriggerAttentionRequired<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagerelativeapplicationid: Param0, applicationparameters: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TriggerAttentionRequired)(::core::mem::transmute_copy(this), packagerelativeapplicationid.into_param().abi(), applicationparameters.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IssueCredentialsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, username: Param0, password: Param1, extraparameters: Param2, markasmanualconnectonfailure: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>> {
        let this = &::windows::core::Interface::cast::<IHotspotAuthenticationContext2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IssueCredentialsAsync)(::core::mem::transmute_copy(this), username.into_param().abi(), password.into_param().abi(), extraparameters.into_param().abi(), markasmanualconnectonfailure, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn TryGetAuthenticationContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(eventoken: Param0, context: &mut ::core::option::Option<HotspotAuthenticationContext>) -> ::windows::core::Result<bool> {
        Self::IHotspotAuthenticationContextStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAuthenticationContext)(::core::mem::transmute_copy(this), eventoken.into_param().abi(), context as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHotspotAuthenticationContextStatics<R, F: FnOnce(&IHotspotAuthenticationContextStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HotspotAuthenticationContext, IHotspotAuthenticationContextStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HotspotAuthenticationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HotspotAuthenticationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HotspotAuthenticationContext {}
impl ::core::fmt::Debug for HotspotAuthenticationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HotspotAuthenticationContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.HotspotAuthenticationContext;{e756c791-1003-4de5-83c7-de61d88831d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_Vtbl;
    const IID: ::windows::core::GUID = <IHotspotAuthenticationContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HotspotAuthenticationContext {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationContext";
}
impl ::core::convert::From<HotspotAuthenticationContext> for ::windows::core::IUnknown {
    fn from(value: HotspotAuthenticationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HotspotAuthenticationContext> for ::windows::core::IUnknown {
    fn from(value: &HotspotAuthenticationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HotspotAuthenticationContext> for ::windows::core::IInspectable {
    fn from(value: HotspotAuthenticationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HotspotAuthenticationContext> for ::windows::core::IInspectable {
    fn from(value: &HotspotAuthenticationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HotspotAuthenticationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct HotspotAuthenticationEventDetails(::windows::core::IUnknown);
impl HotspotAuthenticationEventDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn EventToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EventToken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HotspotAuthenticationEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HotspotAuthenticationEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HotspotAuthenticationEventDetails {}
impl ::core::fmt::Debug for HotspotAuthenticationEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HotspotAuthenticationEventDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.HotspotAuthenticationEventDetails;{e756c791-1001-4de5-83c7-de61d88831d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_Vtbl;
    const IID: ::windows::core::GUID = <IHotspotAuthenticationEventDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HotspotAuthenticationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationEventDetails";
}
impl ::core::convert::From<HotspotAuthenticationEventDetails> for ::windows::core::IUnknown {
    fn from(value: HotspotAuthenticationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HotspotAuthenticationEventDetails> for ::windows::core::IUnknown {
    fn from(value: &HotspotAuthenticationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HotspotAuthenticationEventDetails> for ::windows::core::IInspectable {
    fn from(value: HotspotAuthenticationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HotspotAuthenticationEventDetails> for ::windows::core::IInspectable {
    fn from(value: &HotspotAuthenticationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HotspotAuthenticationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: Self = Self(0i32);
    pub const LoginSucceeded: Self = Self(50i32);
    pub const LoginFailed: Self = Self(100i32);
    pub const RadiusServerError: Self = Self(102i32);
    pub const NetworkAdministratorError: Self = Self(105i32);
    pub const LoginAborted: Self = Self(151i32);
    pub const AccessGatewayInternalError: Self = Self(255i32);
}
impl ::core::marker::Copy for HotspotAuthenticationResponseCode {}
impl ::core::clone::Clone for HotspotAuthenticationResponseCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HotspotAuthenticationResponseCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HotspotAuthenticationResponseCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for HotspotAuthenticationResponseCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationResponseCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HotspotAuthenticationResponseCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.HotspotAuthenticationResponseCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct HotspotCredentialsAuthenticationResult(::windows::core::IUnknown);
impl HotspotCredentialsAuthenticationResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn HasNetworkErrorOccurred(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasNetworkErrorOccurred)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ResponseCode(&self) -> ::windows::core::Result<HotspotAuthenticationResponseCode> {
        let this = self;
        unsafe {
            let mut result__: HotspotAuthenticationResponseCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HotspotAuthenticationResponseCode>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LogoffUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogoffUrl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn AuthenticationReplyXml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationReplyXml)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::clone::Clone for HotspotCredentialsAuthenticationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HotspotCredentialsAuthenticationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HotspotCredentialsAuthenticationResult {}
impl ::core::fmt::Debug for HotspotCredentialsAuthenticationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotCredentialsAuthenticationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HotspotCredentialsAuthenticationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.HotspotCredentialsAuthenticationResult;{e756c791-1005-4de5-83c7-de61d88831d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_Vtbl;
    const IID: ::windows::core::GUID = <IHotspotCredentialsAuthenticationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HotspotCredentialsAuthenticationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotCredentialsAuthenticationResult";
}
impl ::core::convert::From<HotspotCredentialsAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: HotspotCredentialsAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HotspotCredentialsAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: &HotspotCredentialsAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HotspotCredentialsAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: HotspotCredentialsAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HotspotCredentialsAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: &HotspotCredentialsAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HotspotCredentialsAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESim(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESim {
    type Vtable = IESim_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f6e6e26_f123_437d_8ced_dc1d2bc0c3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AvailableMemoryInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailableMemoryInBytes: usize,
    pub Eid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MobileBroadbandModemDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfiles: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadProfileMetadataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProfileMetadataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProfileChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProfileChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESim2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESim2 {
    type Vtable = IESim2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4fd0a0_c68f_56eb_b99b_8f34b8100299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Discover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DiscoverWithServerAddressAndMatchingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serveraddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, matchingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DiscoverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiscoverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DiscoverWithServerAddressAndMatchingIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serveraddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, matchingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiscoverWithServerAddressAndMatchingIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38bd0a58_4d5a_4d08_8da7_e73eff369ddd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimAddedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ESim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimDiscoverEvent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe59ac3e3_39bc_5f6f_9321_0d4a182d261b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDiscoverEvent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MatchingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RspServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimDiscoverResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimDiscoverResult {
    type Vtable = IESimDiscoverResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56b4bb5e_ab2f_5ac6_b359_dd5a8e237926);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDiscoverResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Events: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Events: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimDiscoverResultKind) -> ::windows::core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimDownloadProfileMetadataResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4234d9e_5ad6_426d_8d00_4434f449afec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDownloadProfileMetadataResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimManagerStatics {
    type Vtable = IESimManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bfa2c0c_df88_4631_bf04_c12e281b3962);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ServiceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryCreateESimWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceInfoChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimOperationResult {
    type Vtable = IESimOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa67b63b1_309b_4e77_9e7e_cd93f1ddc7b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimOperationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimOperationStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimPolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimPolicy {
    type Vtable = IESimPolicy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e1b99d_cf7e_4315_882b_6f1e74b0d38f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimPolicy_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShouldEnableManagingUi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimProfile(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimProfile {
    type Vtable = IESimProfile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee1e7880_06a9_4027_b4f8_ddb23d7810e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfile_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileClass) -> ::windows::core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetNicknameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newnickname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNicknameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimProfileMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimProfileMetadata {
    type Vtable = IESimProfileMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed25831f_90db_498d_a7b4_ebce807d3c23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfileMetadata_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsConfirmationCodeRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileMetadataState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DenyInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenyInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfirmInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfirmInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfirmInstallWithConfirmationCodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmationcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfirmInstallWithConfirmationCodeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PostponeInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PostponeInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimProfilePolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimProfilePolicy {
    type Vtable = IESimProfilePolicy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6dd0f1d_9c5c_46c5_a289_a948999bf062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfilePolicy_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsManagedByEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdec5277b_2fd9_4ed9_8376_d9b5e41278a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimRemovedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ESim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimServiceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimServiceInfo {
    type Vtable = IESimServiceInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf16aabcf_7f59_4a51_8494_bd89d5ff50ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimServiceInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AuthenticationPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimAuthenticationPreference) -> ::windows::core::HRESULT,
    pub IsESimUiEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c125cec_508d_4b88_83cb_68bef8168d12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ESim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IESimWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IESimWatcher {
    type Vtable = IESimWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1f84ceb_a28d_4fbf_9771_6e31b81ccf22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFdnAccessManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFdnAccessManagerStatics {
    type Vtable = IFdnAccessManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2aa4395_f1e6_4319_aa3e_477ca64b2bdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFdnAccessManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestUnlockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnlockAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHotspotAuthenticationContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_1003_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WirelessNetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    #[cfg(feature = "Foundation")]
    pub RedirectMessageUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedirectMessageUrl: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub RedirectMessageXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    RedirectMessageXml: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationUrl: usize,
    pub IssueCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, extraparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, markasmanualconnectonfailure: bool) -> ::windows::core::HRESULT,
    pub AbortAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markasmanual: bool) -> ::windows::core::HRESULT,
    pub SkipAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TriggerAttentionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagerelativeapplicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHotspotAuthenticationContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHotspotAuthenticationContext2 {
    type Vtable = IHotspotAuthenticationContext2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_1004_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub IssueCredentialsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, extraparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, markasmanualconnectonfailure: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IssueCredentialsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHotspotAuthenticationContextStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHotspotAuthenticationContextStatics {
    type Vtable = IHotspotAuthenticationContextStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_1002_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContextStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryGetAuthenticationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHotspotAuthenticationEventDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_1001_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationEventDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EventToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHotspotCredentialsAuthenticationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_1005_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotCredentialsAuthenticationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HasNetworkErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ResponseCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HotspotAuthenticationResponseCode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LogoffUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LogoffUrl: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub AuthenticationReplyXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    AuthenticationReplyXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownCSimFilePathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownCSimFilePathsStatics {
    type Vtable = IKnownCSimFilePathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb458aeed_49f1_4c22_b073_96d511bf9c35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownCSimFilePathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownRuimFilePathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownRuimFilePathsStatics {
    type Vtable = IKnownRuimFilePathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3883c8b9_ff24_4571_a867_09f960426e14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRuimFilePathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSimFilePathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownSimFilePathsStatics {
    type Vtable = IKnownSimFilePathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80cd1a63_37a5_43d3_80a3_ccd23e8fecee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimFilePathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOns: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUSimFilePathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownUSimFilePathsStatics {
    type Vtable = IKnownUSimFilePathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c34e581_1f1b_43f4_9530_8b092d32d71f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUSimFilePathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOpl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOpl: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFPnn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFPnn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccount(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36c24ccd_cee2_43e0_a603_ee86a36d6570);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ServiceProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurrentNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CurrentDeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccount2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccount2 {
    type Vtable = IMobileBroadbandAccount2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38f52f1c_1136_4257_959f_b658a352b6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking_Connectivity")))]
    GetConnectionProfiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccount3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccount3 {
    type Vtable = IMobileBroadbandAccount3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x092a1e21_9379_4b9b_ad31_d5fee2f748c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AccountExperienceUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountExperienceUrl: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccountEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3853c880_77de_4c04_bead_a123b08c9f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccountStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccountStatics {
    type Vtable = IMobileBroadbandAccountStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7f4d24_afc1_4fc8_ae9a_a9175310faad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableNetworkAccountIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableNetworkAccountIds: usize,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccountUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bc31d88_a6bd_49e1_80ab_6b91354a57d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasDeviceInformationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasNetworkChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAccountWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf3335e_23b5_449f_928d_5e0d3e04471d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AccountAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountAdded: usize,
    #[cfg(feature = "Foundation")]
    pub AccountUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AccountRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandAccountWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAntennaSar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9af4b7e_cbf9_4109_90be_5c06bfd513b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSar_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AntennaIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SarBackoffIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandAntennaSarFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandAntennaSarFactory {
    type Vtable = IMobileBroadbandAntennaSarFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa91e1716_c04d_4a21_8698_1459dc672c6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSarFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWithIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antennaindex: i32, sarbackoffindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellCdma(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0601b3b4_411a_4f2e_8287_76f5650c60cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellCdma_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BaseStationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationId: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationPNCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationPNCode: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLatitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLatitude: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLongitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLongitude: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLastBroadcastGpsTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLastBroadcastGpsTime: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkId: usize,
    #[cfg(feature = "Foundation")]
    pub PilotSignalStrengthInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PilotSignalStrengthInDB: usize,
    #[cfg(feature = "Foundation")]
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellGsm(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc917f06_7ee0_47b8_9e1f_c3b48df9df5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellGsm_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BaseStationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationId: usize,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalStrengthInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalStrengthInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellLte(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9197c87b_2b78_456d_8b53_aaa25d0af741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellLte_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalCellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalCellId: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedPowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedQualityInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
    #[cfg(feature = "Foundation")]
    pub TrackingAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackingAreaCode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellNR(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa13f0deb_66fc_4b4b_83a9_a487a3a5a0a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellNR_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalCellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalCellId: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedPowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedQualityInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInNanoseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInNanoseconds: usize,
    #[cfg(feature = "Foundation")]
    pub TrackingAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackingAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalToNoiseRatioInDB: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellTdscdma(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0eda1655_db0e_4182_8cda_cc419a7bde08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellTdscdma_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub CellParameterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellParameterId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub PathLossInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathLossInDB: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalCodePowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellUmts(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77b4b5ae_49c8_4f15_b285_4c26a7f67215);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellUmts_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub PathLossInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathLossInDB: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryScramblingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryScramblingCode: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalCodePowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalToNoiseRatioInDB: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellsInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89a9562a_e472_4da5_929c_de61711dd261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsCdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsGsm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsLte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsTdscdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsUmts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsUmts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsCdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsGsm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsLte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsTdscdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsUmts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsUmts: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCellsInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCellsInfo2 {
    type Vtable = IMobileBroadbandCellsInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66205912_b89f_4e12_bbb6_d5cf09a820ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsNR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsNR: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsNR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsNR: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf718b184_c370_5fd4_a670_1846cb9bce47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d08168_e381_4c6e_9be8_fe156969a446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NetworkDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkDeviceStatus) -> ::windows::core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirmwareInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sms::CellularClass) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    CellularClass: usize,
    pub DataClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows::core::HRESULT,
    pub CustomDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MobileEquipmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TelephoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TelephoneNumbers: usize,
    pub SubscriberId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandDeviceType) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurrentRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceInformation2 {
    type Vtable = IMobileBroadbandDeviceInformation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e467af1_f932_4737_a722_03ba72370cb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PinManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceInformation3 {
    type Vtable = IMobileBroadbandDeviceInformation3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08bb4bd_5d30_4b5a_92cc_d54df881d49e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SimSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SimPnn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SimGid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceInformation4 {
    type Vtable = IMobileBroadbandDeviceInformation4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x263f3152_7b9d_582c_b17c_f80a60b50031);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SlotManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22be1a52_bd80_40ac_8e1f_2e07836a3dbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceService_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCommands: usize,
    pub OpenDataSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceCommandResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0f46abb_94d6_44b9_a538_f0810b645389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceCommandSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc098a45_913b_4914_b6c3_ae6304593e75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendQueryCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendQueryCommandAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendSetCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendSetCommandAsync: usize,
    pub CloseSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6aa13de_1380_40e3_8618_73cbca48138c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceDataSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdad62333_8bcf_4289_8a37_045c2169486a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteDataAsync: usize,
    pub CloseSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d69b5b_c4ed_45f0_803a_d9417a6d9846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsDataReadSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDataWriteSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a055b70_b9ae_4458_9241_a6a5fbf18a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0356912_e9f9_4f67_a03d_43189a316bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MaxDeviceServiceCommandSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxDeviceServiceDataSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsResetSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCurrentConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentConfigurationAsync: usize,
    pub CurrentNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModem2 {
    type Vtable = IMobileBroadbandModem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12862b28_b9eb_4ee2_bbe3_711f53eea373);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetIsPassthroughEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPassthroughEnabledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsPassthroughEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsPassthroughEnabledAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModem3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModem3 {
    type Vtable = IMobileBroadbandModem3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9fec6ea_2f34_4582_9102_c314d2a87eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TryGetPcoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetPcoAsync: usize,
    pub IsInEmergencyCallMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsInEmergencyCallModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInEmergencyCallModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsInEmergencyCallModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsInEmergencyCallModeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModemConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfce035a3_d6cd_4320_b982_be9d3ec7890f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uicc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HomeProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModemConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModemConfiguration2 {
    type Vtable = IMobileBroadbandModemConfiguration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320ff5c5_e460_42ae_aa51_69621e7a4477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SarManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModemIsolation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5618fec_e661_4330_9bb4_3480212ec354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddAllowedHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddAllowedHostRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, first: ::windows::core::RawPtr, last: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyConfigurationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearConfigurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModemIsolationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModemIsolationFactory {
    type Vtable = IMobileBroadbandModemIsolationFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21d7ec58_c2b1_4c2f_a030_72820a24ecd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolationFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modemdeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rulegroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandModemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandModemStatics {
    type Vtable = IMobileBroadbandModemStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99ed637_d6f1_4a78_8cbc_6421a65063c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandNetwork(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb63928c_0309_4cb6_a8c1_6a5a3c8e1ff6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    pub NetworkRegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkRegistrationState) -> ::windows::core::HRESULT,
    pub RegistrationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PacketAttachNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ActivationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RegisteredDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows::core::HRESULT,
    pub RegisteredProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RegisteredProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShowConnectionUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandNetwork2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandNetwork2 {
    type Vtable = IMobileBroadbandNetwork2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a55db22_62f7_4bdd_ba1d_477441960ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetVoiceCallSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVoiceCallSupportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegistrationUiccApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegistrationUiccApps: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandNetwork3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandNetwork3 {
    type Vtable = IMobileBroadbandNetwork3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33670a8a_c7ef_444c_ab6c_df7ef7a390fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetCellsInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCellsInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandNetworkRegistrationStateChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeaf94e1_960f_49b4_a08d_7d85e968c7ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Network: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89135cff_28b8_46aa_b137_1c4b0f21edfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NetworkRegistrationStateChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NetworkRegistrationStateChanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPco(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4e4fcbe_e3a3_43c5_a87b_6c86d229d7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPco_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x263f5114_64e0_4493_909b_2d14a01962b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UpdatedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPin(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe661d709_e779_45bf_8281_75323df9e321);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPin_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows::core::HRESULT,
    pub LockState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinFormat) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MinLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnblockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinunblockkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnblockAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPinLockStateChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe16673e_1f04_4f95_8b90_e7f559dde7e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PinType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows::core::HRESULT,
    pub PinLockState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd338c091_3e91_4d38_9036_aee83a6e79ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PinLockStateChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PinLockStateChanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPinManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83567edd_6e1f_4b9b_a413_2b1f50cc36df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPins: usize,
    pub GetPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: MobileBroadbandPinType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandPinOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11dddc32_31e7_49f5_b663_123d3bef0362);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinOperationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSuccessful: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandRadioStateChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb054a561_9833_4aed_9717_4348b21a24b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71301ace_093c_42c6_b0db_ad1f75a65445);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RadioStateChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RadioStateChanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandSarManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b26833_967e_40c9_a485_19c0dd209e22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSarManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBackoffEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsWiFiHardwareIntegrated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSarControlledByHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Antennas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Antennas: usize,
    #[cfg(feature = "Foundation")]
    pub HysteresisTimerPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HysteresisTimerPeriod: usize,
    #[cfg(feature = "Foundation")]
    pub TransmissionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransmissionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransmissionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransmissionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub EnableBackoffAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableBackoffAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableBackoffAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableBackoffAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antennas: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetConfigurationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RevertSarToHardwareControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RevertSarToHardwareControlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransmissionStateChangedHysteresisAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timerperiod: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransmissionStateChangedHysteresisAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsTransmittingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsTransmittingAsync: usize,
    pub StartTransmissionStateMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopTransmissionStateMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandSlotInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd350b32_882e_542a_b17d_0bb1b49bae9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandSlotState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3158839f_950c_54ce_a48d_ba4529b48f0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SlotInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandSlotManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba07cd6_2019_5f81_a294_cc364a11d0b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SlotInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SlotInfos: usize,
    pub CurrentSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetCurrentSlot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetCurrentSlotAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCurrentSlotAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SlotInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SlotInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSlotInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSlotInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentSlotIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentSlotIndexChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentSlotIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentSlotIndexChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x612e3875_040a_4f99_a4f9_61d7c32da129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTransmitting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandUicc(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe634f691_525a_4ce2_8fce_aa4162579154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUicc_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SimIccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetUiccAppsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUiccAppsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandUiccApp(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d170556_98a1_43dd_b2ec_50c90cf248df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccApp_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAppKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecordDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiccfilepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecordDetailsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiccfilepath: ::windows::core::RawPtr, recordindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadRecordAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppReadRecordResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c95285_358e_47c5_8249_695f383b2bdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppReadRecordResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd919682f_be14_4934_981d_2f57b9ed83e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAppRecordKind) -> ::windows::core::HRESULT,
    pub RecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub RecordSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ReadAccessCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows::core::HRESULT,
    pub WriteAccessCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x744930eb_8157_4a41_8494_6bf54c9b1d2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppsResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UiccApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UiccApps: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorDataUsageTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e3126d_a465_4eeb_9317_28a167630cea);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorDataUsageTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NotificationKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorDataUsageNotificationKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorNotificationEventDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc68a9d1_82e1_4488_9f2c_1276c2468fac);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationEventDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorEventMessageType) -> ::windows::core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EncodingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RuleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub SmsMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    SmsMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringAccessPointConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bcc0284_412e_403d_acc6_b757e34774a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Passphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPassphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringAccessPointConfiguration2 {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1809142_7238_59a0_928b_74ab46fd64b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBandSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBandSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBandSupportedAsync: usize,
    pub Band: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TetheringWiFiBand) -> ::windows::core::HRESULT,
    pub SetBand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TetheringWiFiBand) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringClient(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x709d254c_595f_4847_bb30_646935542918);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClient_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MacAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringClientManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringClientManager {
    type Vtable = INetworkOperatorTetheringClientManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91b14016_8dca_4225_bbed_eef8b8d718d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClientManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTetheringClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTetheringClients: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringEntitlementCheck(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringEntitlementCheck {
    type Vtable = INetworkOperatorTetheringEntitlementCheck_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0108916d_9e9a_4af6_8da3_60493b19c204);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringEntitlementCheck_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AuthorizeTethering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: bool, entitlementfailurereason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45a8da0_0e86_4d98_8ba4_dd70d4b764d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MaxClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TetheringOperationalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationalState) -> ::windows::core::HRESULT,
    pub GetCurrentAccessPointConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConfigureAccessPointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfigureAccessPointAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTetheringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTetheringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopTetheringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopTetheringAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringManagerStatics {
    type Vtable = INetworkOperatorTetheringManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ebcbacc_f8c3_405c_9964_70a1eeabe194);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetTetheringCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut TetheringCapability) -> ::windows::core::HRESULT,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringManagerStatics2 {
    type Vtable = INetworkOperatorTetheringManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b235412_35f0_49e7_9b08_16d278fbaa42);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub GetTetheringCapabilityFromConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut TetheringCapability) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    GetTetheringCapabilityFromConnectionProfile: usize,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringManagerStatics3 {
    type Vtable = INetworkOperatorTetheringManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fdaadb6_4af9_4f21_9b58_d53e9f24231e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfileWithTargetAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfileWithTargetAdapter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringManagerStatics4 {
    type Vtable = INetworkOperatorTetheringManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b9f9d0_ebff_46a4_a847_d663d8b0977e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsNoConnectionsTimeoutEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub EnableNoConnectionsTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableNoConnectionsTimeoutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableNoConnectionsTimeoutAsync: usize,
    pub DisableNoConnectionsTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisableNoConnectionsTimeoutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableNoConnectionsTimeoutAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorTetheringOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebd203a1_01ba_476d_b4b3_bf3d12c8f80c);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringOperationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationStatus) -> ::windows::core::HRESULT,
    pub AdditionalErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProvisionFromXmlDocumentResults(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217700e0_8203_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisionFromXmlDocumentResults_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllElementsProvisioned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ProvisionResultsXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProvisionedProfile(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProvisionedProfile {
    type Vtable = IProvisionedProfile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217700e0_8202_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisionedProfile_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub UpdateCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Connectivity::NetworkCostType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    UpdateCost: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProfileUsage) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateUsage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProvisioningAgent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProvisioningAgent {
    type Vtable = IProvisioningAgent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217700e0_8201_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningAgent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ProvisionFromXmlDocumentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisioningxmldocument: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionFromXmlDocumentAsync: usize,
    pub GetProvisionedProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ProfileMediaType, profilename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProvisioningAgentStaticMethods(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProvisioningAgentStaticMethods {
    type Vtable = IProvisioningAgentStaticMethods_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217700e0_8101_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningAgentStaticMethods_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITetheringEntitlementCheckTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03c65e9d_5926_41f3_a94e_b50926fc421b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITetheringEntitlementCheckTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AllowTethering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DenyTethering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entitlementfailurereason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUssdMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUssdMessage {
    type Vtable = IUssdMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9acf82_2004_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DataCodingScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetDataCodingScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub GetPayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetPayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub PayloadAsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayloadAsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUssdMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUssdMessageFactory {
    type Vtable = IUssdMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9acf82_1003_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUssdReply(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUssdReply {
    type Vtable = IUssdReply_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9acf82_2005_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdReply_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UssdResultCode) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUssdSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUssdSession {
    type Vtable = IUssdSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9acf82_2002_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SendMessageAndGetReplyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAndGetReplyAsync: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUssdSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUssdSessionStatics {
    type Vtable = IUssdSessionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f9acf82_1001_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdSessionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFromNetworkInterfaceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct KnownCSimFilePaths {}
impl KnownCSimFilePaths {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFSpn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownCSimFilePathsStatics<R, F: FnOnce(&IKnownCSimFilePathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownCSimFilePaths, IKnownCSimFilePathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownCSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownCSimFilePaths";
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct KnownRuimFilePaths {}
impl KnownRuimFilePaths {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFSpn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownRuimFilePathsStatics<R, F: FnOnce(&IKnownRuimFilePathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownRuimFilePaths, IKnownRuimFilePathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownRuimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownRuimFilePaths";
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct KnownSimFilePaths {}
impl KnownSimFilePaths {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOns() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFOns)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFSpn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownSimFilePathsStatics<R, F: FnOnce(&IKnownSimFilePathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownSimFilePaths, IKnownSimFilePathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownSimFilePaths";
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct KnownUSimFilePaths {}
impl KnownUSimFilePaths {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFSpn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOpl() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFOpl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFPnn() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EFPnn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gid2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownUSimFilePathsStatics<R, F: FnOnce(&IKnownUSimFilePathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownUSimFilePaths, IKnownUSimFilePathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownUSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownUSimFilePaths";
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandAccount(::windows::core::IUnknown);
impl MobileBroadbandAccount {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ServiceProviderGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceProviderGuid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ServiceProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceProviderName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentNetwork(&self) -> ::windows::core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentNetwork)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandNetwork>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentDeviceInformation(&self) -> ::windows::core::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentDeviceInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub fn GetConnectionProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConnectionProfiles)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountExperienceUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandAccount3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountExperienceUrl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableNetworkAccountIds() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailableNetworkAccountIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkaccountid: Param0) -> ::windows::core::Result<MobileBroadbandAccount> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromNetworkAccountId)(::core::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandAccount>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandAccountStatics<R, F: FnOnce(&IMobileBroadbandAccountStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandAccount, IMobileBroadbandAccountStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccount {}
impl ::core::fmt::Debug for MobileBroadbandAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandAccount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccount;{36c24ccd-cee2-43e0-a603-ee86a36d6570})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandAccount as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandAccount {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccount";
}
impl ::core::convert::From<MobileBroadbandAccount> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccount> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandAccount> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccount> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandAccountEventArgs(::windows::core::IUnknown);
impl MobileBroadbandAccountEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandAccountEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccountEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccountEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandAccountEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandAccountEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccountEventArgs;{3853c880-77de-4c04-bead-a123b08c9f59})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandAccountEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandAccountEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountEventArgs";
}
impl ::core::convert::From<MobileBroadbandAccountEventArgs> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandAccountEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccountEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandAccountEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandAccountEventArgs> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandAccountEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccountEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandAccountEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandAccountUpdatedEventArgs(::windows::core::IUnknown);
impl MobileBroadbandAccountUpdatedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn HasDeviceInformationChanged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasDeviceInformationChanged)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn HasNetworkChanged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasNetworkChanged)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandAccountUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccountUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccountUpdatedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandAccountUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandAccountUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccountUpdatedEventArgs;{7bc31d88-a6bd-49e1-80ab-6b91354a57d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandAccountUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandAccountUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountUpdatedEventArgs";
}
impl ::core::convert::From<MobileBroadbandAccountUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandAccountUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccountUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandAccountUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandAccountUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandAccountUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccountUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandAccountUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandAccountUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcher(::windows::core::IUnknown);
impl MobileBroadbandAccountWatcher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandAccountWatcher, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountAdded)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountAdded)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountUpdated)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountRemoved)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountRemoved)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MobileBroadbandAccountWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandAccountWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandAccountWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MobileBroadbandAccountWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccountWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccountWatcher {}
impl ::core::fmt::Debug for MobileBroadbandAccountWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandAccountWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcher;{6bf3335e-23b5-449f-928d-5e0d3e04471d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandAccountWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandAccountWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcher";
}
impl ::core::convert::From<MobileBroadbandAccountWatcher> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandAccountWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccountWatcher> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandAccountWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandAccountWatcher> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandAccountWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAccountWatcher> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandAccountWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandAccountWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for MobileBroadbandAccountWatcherStatus {}
impl ::core::clone::Clone for MobileBroadbandAccountWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandAccountWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandAccountWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandAccountWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandAccountWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandAntennaSar(::windows::core::IUnknown);
impl MobileBroadbandAntennaSar {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AntennaIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AntennaIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SarBackoffIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SarBackoffIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateWithIndex(antennaindex: i32, sarbackoffindex: i32) -> ::windows::core::Result<MobileBroadbandAntennaSar> {
        Self::IMobileBroadbandAntennaSarFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithIndex)(::core::mem::transmute_copy(this), antennaindex, sarbackoffindex, &mut result__).from_abi::<MobileBroadbandAntennaSar>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandAntennaSarFactory<R, F: FnOnce(&IMobileBroadbandAntennaSarFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandAntennaSar, IMobileBroadbandAntennaSarFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandAntennaSar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAntennaSar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAntennaSar {}
impl ::core::fmt::Debug for MobileBroadbandAntennaSar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAntennaSar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandAntennaSar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandAntennaSar;{b9af4b7e-cbf9-4109-90be-5c06bfd513b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandAntennaSar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandAntennaSar {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAntennaSar";
}
impl ::core::convert::From<MobileBroadbandAntennaSar> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandAntennaSar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAntennaSar> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandAntennaSar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandAntennaSar> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandAntennaSar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandAntennaSar> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandAntennaSar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandAntennaSar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandAntennaSar {}
unsafe impl ::core::marker::Sync for MobileBroadbandAntennaSar {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellCdma(::windows::core::IUnknown);
impl MobileBroadbandCellCdma {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseStationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationPNCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseStationPNCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLatitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseStationLatitude)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLongitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseStationLongitude)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLastBroadcastGpsTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseStationLastBroadcastGpsTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PilotSignalStrengthInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PilotSignalStrengthInDB)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellCdma {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellCdma {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellCdma {}
impl ::core::fmt::Debug for MobileBroadbandCellCdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellCdma").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellCdma {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellCdma;{0601b3b4-411a-4f2e-8287-76f5650c60cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellCdma as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellCdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellCdma";
}
impl ::core::convert::From<MobileBroadbandCellCdma> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellCdma) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellCdma> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellCdma) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellCdma> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellCdma) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellCdma> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellCdma) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellCdma {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellCdma {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellGsm(::windows::core::IUnknown);
impl MobileBroadbandCellGsm {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseStationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationAreaCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalStrengthInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReceivedSignalStrengthInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimingAdvanceInBitPeriods)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellGsm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellGsm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellGsm {}
impl ::core::fmt::Debug for MobileBroadbandCellGsm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellGsm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellGsm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellGsm;{cc917f06-7ee0-47b8-9e1f-c3b48df9df5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellGsm as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellGsm {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellGsm";
}
impl ::core::convert::From<MobileBroadbandCellGsm> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellGsm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellGsm> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellGsm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellGsm> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellGsm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellGsm> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellGsm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellGsm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellGsm {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellGsm {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellLte(::windows::core::IUnknown);
impl MobileBroadbandCellLte {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalCellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalCellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReferenceSignalReceivedPowerInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReferenceSignalReceivedQualityInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimingAdvanceInBitPeriods)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrackingAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackingAreaCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellLte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellLte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellLte {}
impl ::core::fmt::Debug for MobileBroadbandCellLte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellLte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellLte {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellLte;{9197c87b-2b78-456d-8b53-aaa25d0af741})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellLte as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellLte {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellLte";
}
impl ::core::convert::From<MobileBroadbandCellLte> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellLte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellLte> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellLte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellLte> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellLte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellLte> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellLte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellLte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellLte {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellLte {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellNR(::windows::core::IUnknown);
impl MobileBroadbandCellNR {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalCellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalCellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReferenceSignalReceivedPowerInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReferenceSignalReceivedQualityInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInNanoseconds(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimingAdvanceInNanoseconds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrackingAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackingAreaCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalToNoiseRatioInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalToNoiseRatioInDB)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellNR {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellNR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellNR {}
impl ::core::fmt::Debug for MobileBroadbandCellNR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellNR").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellNR {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellNR;{a13f0deb-66fc-4b4b-83a9-a487a3a5a0a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellNR as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellNR {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellNR";
}
impl ::core::convert::From<MobileBroadbandCellNR> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellNR) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellNR> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellNR) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellNR> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellNR) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellNR> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellNR) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellNR {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellNR {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellNR {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellTdscdma(::windows::core::IUnknown);
impl MobileBroadbandCellTdscdma {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellParameterId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellParameterId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationAreaCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PathLossInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PathLossInDB)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReceivedSignalCodePowerInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimingAdvanceInBitPeriods)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellTdscdma {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellTdscdma {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellTdscdma {}
impl ::core::fmt::Debug for MobileBroadbandCellTdscdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellTdscdma").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellTdscdma {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellTdscdma;{0eda1655-db0e-4182-8cda-cc419a7bde08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellTdscdma as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellTdscdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellTdscdma";
}
impl ::core::convert::From<MobileBroadbandCellTdscdma> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellTdscdma) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellTdscdma> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellTdscdma) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellTdscdma> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellTdscdma) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellTdscdma> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellTdscdma) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellTdscdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellTdscdma {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellTdscdma {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellUmts(::windows::core::IUnknown);
impl MobileBroadbandCellUmts {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationAreaCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PathLossInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PathLossInDB)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryScramblingCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryScramblingCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReceivedSignalCodePowerInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalToNoiseRatioInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalToNoiseRatioInDB)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellUmts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellUmts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellUmts {}
impl ::core::fmt::Debug for MobileBroadbandCellUmts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellUmts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellUmts {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellUmts;{77b4b5ae-49c8-4f15-b285-4c26a7f67215})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellUmts as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellUmts {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellUmts";
}
impl ::core::convert::From<MobileBroadbandCellUmts> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellUmts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellUmts> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellUmts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellUmts> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellUmts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellUmts> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellUmts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellUmts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellUmts {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellUmts {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCellsInfo(::windows::core::IUnknown);
impl MobileBroadbandCellsInfo {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsCdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringCellsCdma)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsGsm(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringCellsGsm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsLte(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringCellsLte)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsTdscdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringCellsTdscdma)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsUmts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringCellsUmts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsCdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServingCellsCdma)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsGsm(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServingCellsGsm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsLte(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServingCellsLte)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsTdscdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServingCellsTdscdma)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsUmts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServingCellsUmts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsNR(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringCellsNR)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsNR(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServingCellsNR)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCellsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellsInfo {}
impl ::core::fmt::Debug for MobileBroadbandCellsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellsInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCellsInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCellsInfo;{89a9562a-e472-4da5-929c-de61711dd261})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCellsInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCellsInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellsInfo";
}
impl ::core::convert::From<MobileBroadbandCellsInfo> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCellsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellsInfo> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCellsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCellsInfo> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCellsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCellsInfo> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCellsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCellsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCellsInfo {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellsInfo {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandCurrentSlotIndexChangedEventArgs(::windows::core::IUnknown);
impl MobileBroadbandCurrentSlotIndexChangedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentSlotIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentSlotIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCurrentSlotIndexChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandCurrentSlotIndexChangedEventArgs;{f718b184-c370-5fd4-a670-1846cb9bce47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandCurrentSlotIndexChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCurrentSlotIndexChangedEventArgs";
}
impl ::core::convert::From<MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandCurrentSlotIndexChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandCurrentSlotIndexChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceInformation(::windows::core::IUnknown);
impl MobileBroadbandDeviceInformation {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkDeviceStatus(&self) -> ::windows::core::Result<NetworkDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__: NetworkDeviceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkDeviceStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NetworkDeviceStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Manufacturer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Model(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Model)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn FirmwareInformation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirmwareInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Devices_Sms\"`*"]
    #[cfg(feature = "Devices_Sms")]
    pub fn CellularClass(&self) -> ::windows::core::Result<super::super::Devices::Sms::CellularClass> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sms::CellularClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellularClass)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sms::CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DataClasses(&self) -> ::windows::core::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__: DataClasses = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataClasses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataClasses>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CustomDataClass(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomDataClass)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MobileEquipmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MobileEquipmentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TelephoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TelephoneNumbers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SubscriberId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SubscriberId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimIccId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceType(&self) -> ::windows::core::Result<MobileBroadbandDeviceType> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentRadioState(&self) -> ::windows::core::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandRadioState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentRadioState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandRadioState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn PinManager(&self) -> ::windows::core::Result<MobileBroadbandPinManager> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinManager)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinManager>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Revision(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Revision)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SerialNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SimSpn(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimSpn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SimPnn(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimPnn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SimGid1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimGid1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SlotManager(&self) -> ::windows::core::Result<MobileBroadbandSlotManager> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandDeviceInformation4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlotManager)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSlotManager>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceInformation {}
impl ::core::fmt::Debug for MobileBroadbandDeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceInformation;{e6d08168-e381-4c6e-9be8-fe156969a446})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceInformation";
}
impl ::core::convert::From<MobileBroadbandDeviceInformation> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceInformation> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceInformation> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceInformation> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceService(::windows::core::IUnknown);
impl MobileBroadbandDeviceService {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceServiceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn OpenDataSession(&self) -> ::windows::core::Result<MobileBroadbandDeviceServiceDataSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenDataSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceServiceDataSession>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn OpenCommandSession(&self) -> ::windows::core::Result<MobileBroadbandDeviceServiceCommandSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenCommandSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceServiceCommandSession>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceService {}
impl ::core::fmt::Debug for MobileBroadbandDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceService;{22be1a52-bd80-40ac-8e1f-2e07836a3dbd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceService as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceService {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceService";
}
impl ::core::convert::From<MobileBroadbandDeviceService> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceService> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceService> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceService> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceService {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceService {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceCommandResult(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceCommandResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn StatusCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceCommandResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceCommandResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceCommandResult {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceCommandResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceCommandResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceCommandResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandResult;{b0f46abb-94d6-44b9-a538-f0810b645389})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceServiceCommandResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceCommandResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandResult";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceCommandResult> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceCommandResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceCommandResult> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceCommandResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceCommandResult> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceCommandResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceCommandResult> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceCommandResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceServiceCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceCommandResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceCommandResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceCommandSession(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceCommandSession {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendQueryCommandAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, commandid: u32, data: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendQueryCommandAsync)(::core::mem::transmute_copy(this), commandid, data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendSetCommandAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, commandid: u32, data: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendSetCommandAsync)(::core::mem::transmute_copy(this), commandid, data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CloseSession(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CloseSession)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceCommandSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceCommandSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceCommandSession {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceCommandSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceCommandSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceCommandSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandSession;{fc098a45-913b-4914-b6c3-ae6304593e75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceServiceCommandSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceCommandSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandSession";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceCommandSession> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceCommandSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceCommandSession> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceCommandSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceCommandSession> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceCommandSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceCommandSession> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceCommandSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceServiceCommandSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceCommandSession {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceCommandSession {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceDataReceivedEventArgs(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceDataReceivedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReceivedData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataReceivedEventArgs;{b6aa13de-1380-40e3-8618-73cbca48138c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceServiceDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataReceivedEventArgs";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceDataSession(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceDataSession {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteDataAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CloseSession(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CloseSession)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataReceived)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataReceived)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceDataSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceDataSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceDataSession {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceDataSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceDataSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceDataSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataSession;{dad62333-8bcf-4289-8a37-045c2169486a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceServiceDataSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceDataSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataSession";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceDataSession> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceDataSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceDataSession> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceDataSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceDataSession> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceDataSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceDataSession> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceDataSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceServiceDataSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceDataSession {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceDataSession {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceInformation(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceInformation {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceServiceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsDataReadSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDataReadSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsDataWriteSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDataWriteSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceInformation {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceInformation;{53d69b5b-c4ed-45f0-803a-d9417a6d9846})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceServiceInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceInformation";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceInformation> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceInformation> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceInformation> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceInformation> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceServiceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceInformation {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceInformation {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceTriggerDetails(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceServiceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReceivedData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceTriggerDetails;{4a055b70-b9ae-4458-9241-a6a5fbf18a0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandDeviceServiceTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceTriggerDetails";
}
impl ::core::convert::From<MobileBroadbandDeviceServiceTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandDeviceServiceTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandDeviceServiceTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandDeviceServiceTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandDeviceServiceTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Embedded: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
    pub const Remote: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandDeviceType {}
impl ::core::clone::Clone for MobileBroadbandDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandModem(::windows::core::IUnknown);
impl MobileBroadbandModem {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentAccount(&self) -> ::windows::core::Result<MobileBroadbandAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandAccount>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandDeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MaxDeviceServiceCommandSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxDeviceServiceCommandSizeInBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MaxDeviceServiceDataSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxDeviceServiceDataSizeInBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetDeviceService<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, deviceserviceid: Param0) -> ::windows::core::Result<MobileBroadbandDeviceService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceService)(::core::mem::transmute_copy(this), deviceserviceid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandDeviceService>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsResetSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsResetSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResetAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentConfigurationAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentNetwork(&self) -> ::windows::core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentNetwork)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandNetwork>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsPassthroughEnabledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsPassthroughEnabledAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsPassthroughEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetIsPassthroughEnabledAsync)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetPcoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetPcoAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsInEmergencyCallMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInEmergencyCallMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsInEmergencyCallModeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInEmergencyCallModeChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsInEmergencyCallModeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsInEmergencyCallModeChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromId)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandModem>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandModem>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandModemStatics<R, F: FnOnce(&IMobileBroadbandModemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandModem, IMobileBroadbandModemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandModem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandModem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandModem {}
impl ::core::fmt::Debug for MobileBroadbandModem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandModem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandModem;{d0356912-e9f9-4f67-a03d-43189a316bf1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandModem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandModem {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModem";
}
impl ::core::convert::From<MobileBroadbandModem> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandModem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandModem> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandModem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandModem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandModem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandModem> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandModem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandModem> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandModem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandModem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandModem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandModem {}
unsafe impl ::core::marker::Sync for MobileBroadbandModem {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandModemConfiguration(::windows::core::IUnknown);
impl MobileBroadbandModemConfiguration {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Uicc(&self) -> ::windows::core::Result<MobileBroadbandUicc> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uicc)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUicc>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn HomeProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HomeProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn HomeProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HomeProviderName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SarManager(&self) -> ::windows::core::Result<MobileBroadbandSarManager> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandModemConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SarManager)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSarManager>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandModemConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandModemConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandModemConfiguration {}
impl ::core::fmt::Debug for MobileBroadbandModemConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandModemConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandModemConfiguration;{fce035a3-d6cd-4320-b982-be9d3ec7890f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandModemConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandModemConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemConfiguration";
}
impl ::core::convert::From<MobileBroadbandModemConfiguration> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandModemConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandModemConfiguration> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandModemConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandModemConfiguration> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandModemConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandModemConfiguration> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandModemConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandModemConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandModemIsolation(::windows::core::IUnknown);
impl MobileBroadbandModemIsolation {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AddAllowedHost<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>>(&self, host: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddAllowedHost)(::core::mem::transmute_copy(this), host.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AddAllowedHostRange<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>, Param1: ::windows::core::IntoParam<'a, super::HostName>>(&self, first: Param0, last: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddAllowedHostRange)(::core::mem::transmute_copy(this), first.into_param().abi(), last.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplyConfigurationAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearConfigurationAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(modemdeviceid: Param0, rulegroupid: Param1) -> ::windows::core::Result<MobileBroadbandModemIsolation> {
        Self::IMobileBroadbandModemIsolationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), modemdeviceid.into_param().abi(), rulegroupid.into_param().abi(), &mut result__).from_abi::<MobileBroadbandModemIsolation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandModemIsolationFactory<R, F: FnOnce(&IMobileBroadbandModemIsolationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MobileBroadbandModemIsolation, IMobileBroadbandModemIsolationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MobileBroadbandModemIsolation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandModemIsolation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandModemIsolation {}
impl ::core::fmt::Debug for MobileBroadbandModemIsolation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemIsolation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandModemIsolation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandModemIsolation;{b5618fec-e661-4330-9bb4-3480212ec354})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandModemIsolation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandModemIsolation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemIsolation";
}
impl ::core::convert::From<MobileBroadbandModemIsolation> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandModemIsolation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandModemIsolation> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandModemIsolation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandModemIsolation> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandModemIsolation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandModemIsolation> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandModemIsolation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandModemIsolation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandModemIsolation {}
unsafe impl ::core::marker::Sync for MobileBroadbandModemIsolation {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const Busy: Self = Self(2i32);
    pub const NoDeviceSupport: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandModemStatus {}
impl ::core::clone::Clone for MobileBroadbandModemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandModemStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandModemStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandModemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandModemStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandModemStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandNetwork(::windows::core::IUnknown);
impl MobileBroadbandNetwork {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAdapter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Connectivity::NetworkAdapter>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkRegistrationState(&self) -> ::windows::core::Result<NetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__: NetworkRegistrationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkRegistrationState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NetworkRegistrationState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RegistrationNetworkError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegistrationNetworkError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn PacketAttachNetworkError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PacketAttachNetworkError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ActivationNetworkError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivationNetworkError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessPointName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RegisteredDataClass(&self) -> ::windows::core::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__: DataClasses = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisteredDataClass)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataClasses>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RegisteredProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisteredProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RegisteredProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisteredProviderName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ShowConnectionUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowConnectionUI)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVoiceCallSupportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVoiceCallSupportAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegistrationUiccApps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegistrationUiccApps)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCellsInfoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>> {
        let this = &::windows::core::Interface::cast::<IMobileBroadbandNetwork3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCellsInfoAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandNetwork {}
impl ::core::fmt::Debug for MobileBroadbandNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandNetwork").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandNetwork {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandNetwork;{cb63928c-0309-4cb6-a8c1-6a5a3c8e1ff6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandNetwork as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandNetwork {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetwork";
}
impl ::core::convert::From<MobileBroadbandNetwork> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandNetwork> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandNetwork> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandNetwork> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandNetworkRegistrationStateChange(::windows::core::IUnknown);
impl MobileBroadbandNetworkRegistrationStateChange {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Network(&self) -> ::windows::core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Network)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandNetwork>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandNetworkRegistrationStateChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandNetworkRegistrationStateChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandNetworkRegistrationStateChange {}
impl ::core::fmt::Debug for MobileBroadbandNetworkRegistrationStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandNetworkRegistrationStateChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandNetworkRegistrationStateChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChange;{beaf94e1-960f-49b4-a08d-7d85e968c7ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandNetworkRegistrationStateChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandNetworkRegistrationStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChange";
}
impl ::core::convert::From<MobileBroadbandNetworkRegistrationStateChange> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandNetworkRegistrationStateChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandNetworkRegistrationStateChange> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandNetworkRegistrationStateChange> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandNetworkRegistrationStateChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandNetworkRegistrationStateChange> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandNetworkRegistrationStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandNetworkRegistrationStateChange {}
unsafe impl ::core::marker::Sync for MobileBroadbandNetworkRegistrationStateChange {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandNetworkRegistrationStateChangeTriggerDetails(::windows::core::IUnknown);
impl MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NetworkRegistrationStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkRegistrationStateChanges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandNetworkRegistrationStateChangeTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChangeTriggerDetails;{89135cff-28b8-46aa-b137-1c4b0f21edfe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChangeTriggerDetails";
}
impl ::core::convert::From<MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandNetworkRegistrationStateChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandNetworkRegistrationStateChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPco(::windows::core::IUnknown);
impl MobileBroadbandPco {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsComplete(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsComplete)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPco {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPco {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPco {}
impl ::core::fmt::Debug for MobileBroadbandPco {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPco").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPco {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPco;{d4e4fcbe-e3a3-43c5-a87b-6c86d229d7fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPco as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPco {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPco";
}
impl ::core::convert::From<MobileBroadbandPco> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPco) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPco> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPco) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPco {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPco {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPco> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPco) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPco> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPco) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPco {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPco {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPco {}
unsafe impl ::core::marker::Sync for MobileBroadbandPco {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTriggerDetails(::windows::core::IUnknown);
impl MobileBroadbandPcoDataChangeTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn UpdatedData(&self) -> ::windows::core::Result<MobileBroadbandPco> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdatedData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPco>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPcoDataChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPcoDataChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPcoDataChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandPcoDataChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPcoDataChangeTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPcoDataChangeTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPcoDataChangeTriggerDetails;{263f5114-64e0-4493-909b-2d14a01962b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPcoDataChangeTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPcoDataChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPcoDataChangeTriggerDetails";
}
impl ::core::convert::From<MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPcoDataChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPcoDataChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPcoDataChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPcoDataChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandPcoDataChangeTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPin(::windows::core::IUnknown);
impl MobileBroadbandPin {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn LockState(&self) -> ::windows::core::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinLockState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LockState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinLockState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Format(&self) -> ::windows::core::Result<MobileBroadbandPinFormat> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Enabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MaxLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MinLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AttemptsRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttemptsRemaining)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, currentpin: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableAsync)(::core::mem::transmute_copy(this), currentpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, currentpin: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableAsync)(::core::mem::transmute_copy(this), currentpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, currentpin: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnterAsync)(::core::mem::transmute_copy(this), currentpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChangeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, currentpin: Param0, newpin: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeAsync)(::core::mem::transmute_copy(this), currentpin.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnblockAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pinunblockkey: Param0, newpin: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnblockAsync)(::core::mem::transmute_copy(this), pinunblockkey.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPin {}
impl ::core::fmt::Debug for MobileBroadbandPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPin;{e661d709-e779-45bf-8281-75323df9e321})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPin as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPin {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPin";
}
impl ::core::convert::From<MobileBroadbandPin> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPin> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPin> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPin> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPin {}
unsafe impl ::core::marker::Sync for MobileBroadbandPin {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Numeric: Self = Self(1i32);
    pub const Alphanumeric: Self = Self(2i32);
}
impl ::core::marker::Copy for MobileBroadbandPinFormat {}
impl ::core::clone::Clone for MobileBroadbandPinFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandPinFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandPinFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandPinFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: Self = Self(0i32);
    pub const Unlocked: Self = Self(1i32);
    pub const PinRequired: Self = Self(2i32);
    pub const PinUnblockKeyRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandPinLockState {}
impl ::core::clone::Clone for MobileBroadbandPinLockState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandPinLockState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandPinLockState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandPinLockState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinLockState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinLockState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChange(::windows::core::IUnknown);
impl MobileBroadbandPinLockStateChange {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn PinType(&self) -> ::windows::core::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn PinLockState(&self) -> ::windows::core::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandPinLockState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinLockState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandPinLockState>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinLockStateChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinLockStateChange {}
impl ::core::fmt::Debug for MobileBroadbandPinLockStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockStateChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinLockStateChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChange;{be16673e-1f04-4f95-8b90-e7f559dde7e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPinLockStateChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPinLockStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChange";
}
impl ::core::convert::From<MobileBroadbandPinLockStateChange> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPinLockStateChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinLockStateChange> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPinLockStateChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPinLockStateChange> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPinLockStateChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinLockStateChange> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPinLockStateChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPinLockStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChange {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChange {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(::windows::core::IUnknown);
impl MobileBroadbandPinLockStateChangeTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PinLockStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinLockStateChanges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinLockStateChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockStateChangeTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinLockStateChangeTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChangeTriggerDetails;{d338c091-3e91-4d38-9036-aee83a6e79ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPinLockStateChangeTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPinLockStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChangeTriggerDetails";
}
impl ::core::convert::From<MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinLockStateChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPinLockStateChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPinLockStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChangeTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinManager(::windows::core::IUnknown);
impl MobileBroadbandPinManager {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPins(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedPins)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetPin(&self, pintype: MobileBroadbandPinType) -> ::windows::core::Result<MobileBroadbandPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPin)(::core::mem::transmute_copy(this), pintype, &mut result__).from_abi::<MobileBroadbandPin>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPinManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinManager {}
impl ::core::fmt::Debug for MobileBroadbandPinManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinManager;{83567edd-6e1f-4b9b-a413-2b1f50cc36df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPinManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPinManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinManager";
}
impl ::core::convert::From<MobileBroadbandPinManager> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPinManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinManager> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPinManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPinManager> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPinManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinManager> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPinManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPinManager {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinManager {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinOperationResult(::windows::core::IUnknown);
impl MobileBroadbandPinOperationResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsSuccessful(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuccessful)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AttemptsRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttemptsRemaining)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandPinOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinOperationResult {}
impl ::core::fmt::Debug for MobileBroadbandPinOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandPinOperationResult;{11dddc32-31e7-49f5-b663-123d3bef0362})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandPinOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPinOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinOperationResult";
}
impl ::core::convert::From<MobileBroadbandPinOperationResult> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandPinOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinOperationResult> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandPinOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandPinOperationResult> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandPinOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandPinOperationResult> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandPinOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandPinOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandPinOperationResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinOperationResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandPinType(pub i32);
impl MobileBroadbandPinType {
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Pin1: Self = Self(2i32);
    pub const Pin2: Self = Self(3i32);
    pub const SimPin: Self = Self(4i32);
    pub const FirstSimPin: Self = Self(5i32);
    pub const NetworkPin: Self = Self(6i32);
    pub const NetworkSubsetPin: Self = Self(7i32);
    pub const ServiceProviderPin: Self = Self(8i32);
    pub const CorporatePin: Self = Self(9i32);
    pub const SubsidyLock: Self = Self(10i32);
}
impl ::core::marker::Copy for MobileBroadbandPinType {}
impl ::core::clone::Clone for MobileBroadbandPinType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandPinType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandPinType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandPinType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandPinType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for MobileBroadbandRadioState {}
impl ::core::clone::Clone for MobileBroadbandRadioState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandRadioState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandRadioState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandRadioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandRadioState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandRadioState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChange(::windows::core::IUnknown);
impl MobileBroadbandRadioStateChange {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RadioState(&self) -> ::windows::core::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandRadioState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RadioState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandRadioState>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandRadioStateChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRadioStateChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRadioStateChange {}
impl ::core::fmt::Debug for MobileBroadbandRadioStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioStateChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandRadioStateChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChange;{b054a561-9833-4aed-9717-4348b21a24b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandRadioStateChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandRadioStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChange";
}
impl ::core::convert::From<MobileBroadbandRadioStateChange> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandRadioStateChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRadioStateChange> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandRadioStateChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandRadioStateChange> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandRadioStateChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRadioStateChange> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandRadioStateChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandRadioStateChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChange {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChange {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTriggerDetails(::windows::core::IUnknown);
impl MobileBroadbandRadioStateChangeTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RadioStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RadioStateChanges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandRadioStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRadioStateChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRadioStateChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandRadioStateChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioStateChangeTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandRadioStateChangeTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChangeTriggerDetails;{71301ace-093c-42c6-b0db-ad1f75a65445})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandRadioStateChangeTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandRadioStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChangeTriggerDetails";
}
impl ::core::convert::From<MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandRadioStateChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandRadioStateChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandRadioStateChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChangeTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandSarManager(::windows::core::IUnknown);
impl MobileBroadbandSarManager {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsBackoffEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBackoffEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsWiFiHardwareIntegrated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsWiFiHardwareIntegrated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsSarControlledByHardware(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSarControlledByHardware)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Antennas(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Antennas)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HysteresisTimerPeriod(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HysteresisTimerPeriod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransmissionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransmissionStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransmissionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTransmissionStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableBackoffAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableBackoffAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableBackoffAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableBackoffAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>>(&self, antennas: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetConfigurationAsync)(::core::mem::transmute_copy(this), antennas.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RevertSarToHardwareControlAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RevertSarToHardwareControlAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTransmissionStateChangedHysteresisAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timerperiod: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetTransmissionStateChangedHysteresisAsync)(::core::mem::transmute_copy(this), timerperiod.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsTransmittingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsTransmittingAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn StartTransmissionStateMonitoring(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartTransmissionStateMonitoring)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn StopTransmissionStateMonitoring(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopTransmissionStateMonitoring)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MobileBroadbandSarManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSarManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSarManager {}
impl ::core::fmt::Debug for MobileBroadbandSarManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSarManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandSarManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSarManager;{e5b26833-967e-40c9-a485-19c0dd209e22})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandSarManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandSarManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSarManager";
}
impl ::core::convert::From<MobileBroadbandSarManager> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandSarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSarManager> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandSarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandSarManager> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandSarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSarManager> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandSarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandSarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandSarManager {}
unsafe impl ::core::marker::Sync for MobileBroadbandSarManager {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandSlotInfo(::windows::core::IUnknown);
impl MobileBroadbandSlotInfo {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Index(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Index)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn State(&self) -> ::windows::core::Result<MobileBroadbandSlotState> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandSlotState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSlotState>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandSlotInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSlotInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSlotInfo {}
impl ::core::fmt::Debug for MobileBroadbandSlotInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandSlotInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSlotInfo;{bd350b32-882e-542a-b17d-0bb1b49bae9e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandSlotInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandSlotInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfo";
}
impl ::core::convert::From<MobileBroadbandSlotInfo> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandSlotInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSlotInfo> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandSlotInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandSlotInfo> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandSlotInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSlotInfo> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandSlotInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandSlotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandSlotInfo {}
unsafe impl ::core::marker::Sync for MobileBroadbandSlotInfo {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandSlotInfoChangedEventArgs(::windows::core::IUnknown);
impl MobileBroadbandSlotInfoChangedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SlotInfo(&self) -> ::windows::core::Result<MobileBroadbandSlotInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlotInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandSlotInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandSlotInfoChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSlotInfoChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSlotInfoChangedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandSlotInfoChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotInfoChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandSlotInfoChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSlotInfoChangedEventArgs;{3158839f-950c-54ce-a48d-ba4529b48f0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandSlotInfoChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandSlotInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfoChangedEventArgs";
}
impl ::core::convert::From<MobileBroadbandSlotInfoChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSlotInfoChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandSlotInfoChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSlotInfoChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandSlotInfoChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandSlotInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandSlotInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandSlotInfoChangedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandSlotManager(::windows::core::IUnknown);
impl MobileBroadbandSlotManager {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SlotInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlotInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CurrentSlotIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentSlotIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetCurrentSlot(&self, slotindex: i32) -> ::windows::core::Result<MobileBroadbandModemStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandModemStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetCurrentSlot)(::core::mem::transmute_copy(this), slotindex, &mut result__).from_abi::<MobileBroadbandModemStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCurrentSlotAsync(&self, slotindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetCurrentSlotAsync)(::core::mem::transmute_copy(this), slotindex, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SlotInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlotInfoChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSlotInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSlotInfoChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentSlotIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentSlotIndexChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentSlotIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCurrentSlotIndexChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MobileBroadbandSlotManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSlotManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSlotManager {}
impl ::core::fmt::Debug for MobileBroadbandSlotManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandSlotManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandSlotManager;{eba07cd6-2019-5f81-a294-cc364a11d0b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandSlotManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandSlotManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotManager";
}
impl ::core::convert::From<MobileBroadbandSlotManager> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandSlotManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSlotManager> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandSlotManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandSlotManager> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandSlotManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandSlotManager> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandSlotManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandSlotManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandSlotManager {}
unsafe impl ::core::marker::Sync for MobileBroadbandSlotManager {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandSlotState(pub i32);
impl MobileBroadbandSlotState {
    pub const Unmanaged: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const OffEmpty: Self = Self(2i32);
    pub const Off: Self = Self(3i32);
    pub const Empty: Self = Self(4i32);
    pub const NotReady: Self = Self(5i32);
    pub const Active: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const ActiveEsim: Self = Self(8i32);
    pub const ActiveEsimNoProfile: Self = Self(9i32);
}
impl ::core::marker::Copy for MobileBroadbandSlotState {}
impl ::core::clone::Clone for MobileBroadbandSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandSlotState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandSlotState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandSlotState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandSlotState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(::windows::core::IUnknown);
impl MobileBroadbandTransmissionStateChangedEventArgs {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsTransmitting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransmitting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandTransmissionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandTransmissionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandTransmissionStateChangedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandTransmissionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandTransmissionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandTransmissionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandTransmissionStateChangedEventArgs;{612e3875-040a-4f99-a4f9-61d7c32da129})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandTransmissionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandTransmissionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandTransmissionStateChangedEventArgs";
}
impl ::core::convert::From<MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandTransmissionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandTransmissionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandTransmissionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandTransmissionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandTransmissionStateChangedEventArgs {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandUicc(::windows::core::IUnknown);
impl MobileBroadbandUicc {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimIccId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetUiccAppsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUiccAppsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandUicc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUicc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUicc {}
impl ::core::fmt::Debug for MobileBroadbandUicc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUicc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandUicc {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUicc;{e634f691-525a-4ce2-8fce-aa4162579154})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandUicc as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandUicc {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUicc";
}
impl ::core::convert::From<MobileBroadbandUicc> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandUicc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUicc> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandUicc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandUicc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandUicc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandUicc> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandUicc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUicc> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandUicc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandUicc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandUicc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandUicc {}
unsafe impl ::core::marker::Sync for MobileBroadbandUicc {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandUiccApp(::windows::core::IUnknown);
impl MobileBroadbandUiccApp {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UiccAppKind> {
        let this = self;
        unsafe {
            let mut result__: UiccAppKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UiccAppKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecordDetailsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, uiccfilepath: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRecordDetailsAsync)(::core::mem::transmute_copy(this), uiccfilepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadRecordAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, uiccfilepath: Param0, recordindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadRecordAsync)(::core::mem::transmute_copy(this), uiccfilepath.into_param().abi(), recordindex, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandUiccApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccApp {}
impl ::core::fmt::Debug for MobileBroadbandUiccApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccApp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandUiccApp {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccApp;{4d170556-98a1-43dd-b2ec-50c90cf248df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandUiccApp as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandUiccApp {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccApp";
}
impl ::core::convert::From<MobileBroadbandUiccApp> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandUiccApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccApp> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandUiccApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandUiccApp> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandUiccApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccApp> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandUiccApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandUiccApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandUiccApp {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccApp {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidUiccFilePath: Self = Self(1i32);
    pub const AccessConditionNotHeld: Self = Self(2i32);
    pub const UiccBusy: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandUiccAppOperationStatus {}
impl ::core::clone::Clone for MobileBroadbandUiccAppOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandUiccAppOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MobileBroadbandUiccAppOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MobileBroadbandUiccAppOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandUiccAppOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandUiccAppReadRecordResult(::windows::core::IUnknown);
impl MobileBroadbandUiccAppReadRecordResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandUiccAppOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUiccAppOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandUiccAppReadRecordResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccAppReadRecordResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccAppReadRecordResult {}
impl ::core::fmt::Debug for MobileBroadbandUiccAppReadRecordResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppReadRecordResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandUiccAppReadRecordResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppReadRecordResult;{64c95285-358e-47c5-8249-695f383b2bdb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandUiccAppReadRecordResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandUiccAppReadRecordResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppReadRecordResult";
}
impl ::core::convert::From<MobileBroadbandUiccAppReadRecordResult> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandUiccAppReadRecordResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccAppReadRecordResult> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandUiccAppReadRecordResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandUiccAppReadRecordResult> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandUiccAppReadRecordResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccAppReadRecordResult> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandUiccAppReadRecordResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandUiccAppReadRecordResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandUiccAppReadRecordResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccAppReadRecordResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(::windows::core::IUnknown);
impl MobileBroadbandUiccAppRecordDetailsResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandUiccAppOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUiccAppOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UiccAppRecordKind> {
        let this = self;
        unsafe {
            let mut result__: UiccAppRecordKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UiccAppRecordKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RecordCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RecordSize(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ReadAccessCondition(&self) -> ::windows::core::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__: UiccAccessCondition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadAccessCondition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UiccAccessCondition>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn WriteAccessCondition(&self) -> ::windows::core::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__: UiccAccessCondition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteAccessCondition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UiccAccessCondition>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandUiccAppRecordDetailsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccAppRecordDetailsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccAppRecordDetailsResult {}
impl ::core::fmt::Debug for MobileBroadbandUiccAppRecordDetailsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppRecordDetailsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandUiccAppRecordDetailsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppRecordDetailsResult;{d919682f-be14-4934-981d-2f57b9ed83e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandUiccAppRecordDetailsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandUiccAppRecordDetailsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppRecordDetailsResult";
}
impl ::core::convert::From<MobileBroadbandUiccAppRecordDetailsResult> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccAppRecordDetailsResult> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandUiccAppRecordDetailsResult> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccAppRecordDetailsResult> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandUiccAppRecordDetailsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandUiccAppRecordDetailsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandUiccAppRecordDetailsResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccAppRecordDetailsResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandUiccAppsResult(::windows::core::IUnknown);
impl MobileBroadbandUiccAppsResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: MobileBroadbandUiccAppOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MobileBroadbandUiccAppOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UiccApps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UiccApps)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>(result__)
        }
    }
}
impl ::core::clone::Clone for MobileBroadbandUiccAppsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccAppsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccAppsResult {}
impl ::core::fmt::Debug for MobileBroadbandUiccAppsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MobileBroadbandUiccAppsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppsResult;{744930eb-8157-4a41-8494-6bf54c9b1d2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_Vtbl;
    const IID: ::windows::core::GUID = <IMobileBroadbandUiccAppsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandUiccAppsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppsResult";
}
impl ::core::convert::From<MobileBroadbandUiccAppsResult> for ::windows::core::IUnknown {
    fn from(value: MobileBroadbandUiccAppsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccAppsResult> for ::windows::core::IUnknown {
    fn from(value: &MobileBroadbandUiccAppsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MobileBroadbandUiccAppsResult> for ::windows::core::IInspectable {
    fn from(value: MobileBroadbandUiccAppsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MobileBroadbandUiccAppsResult> for ::windows::core::IInspectable {
    fn from(value: &MobileBroadbandUiccAppsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MobileBroadbandUiccAppsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MobileBroadbandUiccAppsResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccAppsResult {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkDeviceStatus(pub i32);
impl NetworkDeviceStatus {
    pub const DeviceNotReady: Self = Self(0i32);
    pub const DeviceReady: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceHardwareFailure: Self = Self(4i32);
    pub const AccountNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for NetworkDeviceStatus {}
impl ::core::clone::Clone for NetworkDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NetworkDeviceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkDeviceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkDeviceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkDeviceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: Self = Self(0i32);
}
impl ::core::marker::Copy for NetworkOperatorDataUsageNotificationKind {}
impl ::core::clone::Clone for NetworkOperatorDataUsageNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkOperatorDataUsageNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NetworkOperatorDataUsageNotificationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkOperatorDataUsageNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageNotificationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorDataUsageNotificationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorDataUsageNotificationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTriggerDetails(::windows::core::IUnknown);
impl NetworkOperatorDataUsageTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NotificationKind(&self) -> ::windows::core::Result<NetworkOperatorDataUsageNotificationKind> {
        let this = self;
        unsafe {
            let mut result__: NetworkOperatorDataUsageNotificationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotificationKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NetworkOperatorDataUsageNotificationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkOperatorDataUsageTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorDataUsageTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorDataUsageTriggerDetails {}
impl ::core::fmt::Debug for NetworkOperatorDataUsageTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorDataUsageTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorDataUsageTriggerDetails;{50e3126d-a465-4eeb-9317-28a167630cea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <INetworkOperatorDataUsageTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorDataUsageTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorDataUsageTriggerDetails";
}
impl ::core::convert::From<NetworkOperatorDataUsageTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorDataUsageTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorDataUsageTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorDataUsageTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorDataUsageTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorDataUsageTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorDataUsageTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorDataUsageTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NetworkOperatorDataUsageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkOperatorDataUsageTriggerDetails {}
unsafe impl ::core::marker::Sync for NetworkOperatorDataUsageTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: Self = Self(0i32);
    pub const Cdma: Self = Self(1i32);
    pub const Ussd: Self = Self(2i32);
    pub const DataPlanThresholdReached: Self = Self(3i32);
    pub const DataPlanReset: Self = Self(4i32);
    pub const DataPlanDeleted: Self = Self(5i32);
    pub const ProfileConnected: Self = Self(6i32);
    pub const ProfileDisconnected: Self = Self(7i32);
    pub const RegisteredRoaming: Self = Self(8i32);
    pub const RegisteredHome: Self = Self(9i32);
    pub const TetheringEntitlementCheck: Self = Self(10i32);
    pub const TetheringOperationalStateChanged: Self = Self(11i32);
    pub const TetheringNumberOfClientsChanged: Self = Self(12i32);
}
impl ::core::marker::Copy for NetworkOperatorEventMessageType {}
impl ::core::clone::Clone for NetworkOperatorEventMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkOperatorEventMessageType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NetworkOperatorEventMessageType {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkOperatorEventMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorEventMessageType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorEventMessageType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorEventMessageType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorNotificationEventDetails(::windows::core::IUnknown);
impl NetworkOperatorNotificationEventDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NotificationType(&self) -> ::windows::core::Result<NetworkOperatorEventMessageType> {
        let this = self;
        unsafe {
            let mut result__: NetworkOperatorEventMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotificationType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NetworkOperatorEventMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn EncodingType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn RuleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RuleId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Devices_Sms\"`*"]
    #[cfg(feature = "Devices_Sms")]
    pub fn SmsMessage(&self) -> ::windows::core::Result<super::super::Devices::Sms::ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SmsMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sms::ISmsMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AuthorizeTethering<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, allow: bool, entitlementfailurereason: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INetworkOperatorTetheringEntitlementCheck>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AuthorizeTethering)(::core::mem::transmute_copy(this), allow, entitlementfailurereason.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for NetworkOperatorNotificationEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorNotificationEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorNotificationEventDetails {}
impl ::core::fmt::Debug for NetworkOperatorNotificationEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorNotificationEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorNotificationEventDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorNotificationEventDetails;{bc68a9d1-82e1-4488-9f2c-1276c2468fac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_Vtbl;
    const IID: ::windows::core::GUID = <INetworkOperatorNotificationEventDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorNotificationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorNotificationEventDetails";
}
impl ::core::convert::From<NetworkOperatorNotificationEventDetails> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorNotificationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorNotificationEventDetails> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorNotificationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorNotificationEventDetails> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorNotificationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorNotificationEventDetails> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorNotificationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NetworkOperatorNotificationEventDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkOperatorNotificationEventDetails {}
unsafe impl ::core::marker::Sync for NetworkOperatorNotificationEventDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorTetheringAccessPointConfiguration(::windows::core::IUnknown);
impl NetworkOperatorTetheringAccessPointConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorTetheringAccessPointConfiguration, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ssid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetSsid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSsid)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Passphrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Passphrase)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetPassphrase<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPassphrase)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsBandSupported(&self, band: TetheringWiFiBand) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBandSupported)(::core::mem::transmute_copy(this), band, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsBandSupportedAsync(&self, band: TetheringWiFiBand) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBandSupportedAsync)(::core::mem::transmute_copy(this), band, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Band(&self) -> ::windows::core::Result<TetheringWiFiBand> {
        let this = &::windows::core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__: TetheringWiFiBand = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Band)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TetheringWiFiBand>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetBand(&self, value: TetheringWiFiBand) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBand)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for NetworkOperatorTetheringAccessPointConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringAccessPointConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringAccessPointConfiguration {}
impl ::core::fmt::Debug for NetworkOperatorTetheringAccessPointConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringAccessPointConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorTetheringAccessPointConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringAccessPointConfiguration;{0bcc0284-412e-403d-acc6-b757e34774a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <INetworkOperatorTetheringAccessPointConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorTetheringAccessPointConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringAccessPointConfiguration";
}
impl ::core::convert::From<NetworkOperatorTetheringAccessPointConfiguration> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringAccessPointConfiguration> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorTetheringAccessPointConfiguration> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringAccessPointConfiguration> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorTetheringAccessPointConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NetworkOperatorTetheringAccessPointConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkOperatorTetheringAccessPointConfiguration {}
unsafe impl ::core::marker::Sync for NetworkOperatorTetheringAccessPointConfiguration {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorTetheringClient(::windows::core::IUnknown);
impl NetworkOperatorTetheringClient {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MacAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostNames)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::HostName>>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkOperatorTetheringClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringClient {}
impl ::core::fmt::Debug for NetworkOperatorTetheringClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorTetheringClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringClient;{709d254c-595f-4847-bb30-646935542918})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_Vtbl;
    const IID: ::windows::core::GUID = <INetworkOperatorTetheringClient as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorTetheringClient {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringClient";
}
impl ::core::convert::From<NetworkOperatorTetheringClient> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorTetheringClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringClient> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorTetheringClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorTetheringClient> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorTetheringClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringClient> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorTetheringClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NetworkOperatorTetheringClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkOperatorTetheringClient {}
unsafe impl ::core::marker::Sync for NetworkOperatorTetheringClient {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorTetheringManager(::windows::core::IUnknown);
impl NetworkOperatorTetheringManager {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTetheringClients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>> {
        let this = &::windows::core::Interface::cast::<INetworkOperatorTetheringClientManager>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTetheringClients)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn MaxClientCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxClientCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ClientCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClientCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn TetheringOperationalState(&self) -> ::windows::core::Result<TetheringOperationalState> {
        let this = self;
        unsafe {
            let mut result__: TetheringOperationalState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TetheringOperationalState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TetheringOperationalState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetCurrentAccessPointConfiguration(&self) -> ::windows::core::Result<NetworkOperatorTetheringAccessPointConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentAccessPointConfiguration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NetworkOperatorTetheringAccessPointConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConfigureAccessPointAsync<'a, Param0: ::windows::core::IntoParam<'a, NetworkOperatorTetheringAccessPointConfiguration>>(&self, configuration: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConfigureAccessPointAsync)(::core::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTetheringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTetheringAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopTetheringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopTetheringAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetTetheringCapability<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkaccountid: Param0) -> ::windows::core::Result<TetheringCapability> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__: TetheringCapability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTetheringCapability)(::core::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<TetheringCapability>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkaccountid: Param0) -> ::windows::core::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromNetworkAccountId)(::core::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<NetworkOperatorTetheringManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn GetTetheringCapabilityFromConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, super::Connectivity::ConnectionProfile>>(profile: Param0) -> ::windows::core::Result<TetheringCapability> {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__: TetheringCapability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTetheringCapabilityFromConnectionProfile)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<TetheringCapability>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, super::Connectivity::ConnectionProfile>>(profile: Param0) -> ::windows::core::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromConnectionProfile)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<NetworkOperatorTetheringManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfileWithTargetAdapter<'a, Param0: ::windows::core::IntoParam<'a, super::Connectivity::ConnectionProfile>, Param1: ::windows::core::IntoParam<'a, super::Connectivity::NetworkAdapter>>(profile: Param0, adapter: Param1) -> ::windows::core::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromConnectionProfileWithTargetAdapter)(::core::mem::transmute_copy(this), profile.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<NetworkOperatorTetheringManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn IsNoConnectionsTimeoutEnabled() -> ::windows::core::Result<bool> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsNoConnectionsTimeoutEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn EnableNoConnectionsTimeout() -> ::windows::core::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).EnableNoConnectionsTimeout)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableNoConnectionsTimeoutAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableNoConnectionsTimeoutAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DisableNoConnectionsTimeout() -> ::windows::core::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).DisableNoConnectionsTimeout)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableNoConnectionsTimeoutAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableNoConnectionsTimeoutAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics2<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics3<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics4<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NetworkOperatorTetheringManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringManager {}
impl ::core::fmt::Debug for NetworkOperatorTetheringManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorTetheringManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringManager;{d45a8da0-0e86-4d98-8ba4-dd70d4b764d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_Vtbl;
    const IID: ::windows::core::GUID = <INetworkOperatorTetheringManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorTetheringManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringManager";
}
impl ::core::convert::From<NetworkOperatorTetheringManager> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorTetheringManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringManager> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorTetheringManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorTetheringManager> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorTetheringManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringManager> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorTetheringManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NetworkOperatorTetheringManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorTetheringOperationResult(::windows::core::IUnknown);
impl NetworkOperatorTetheringOperationResult {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<TetheringOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: TetheringOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TetheringOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AdditionalErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdditionalErrorMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkOperatorTetheringOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringOperationResult {}
impl ::core::fmt::Debug for NetworkOperatorTetheringOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkOperatorTetheringOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.NetworkOperatorTetheringOperationResult;{ebd203a1-01ba-476d-b4b3-bf3d12c8f80c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_Vtbl;
    const IID: ::windows::core::GUID = <INetworkOperatorTetheringOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorTetheringOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringOperationResult";
}
impl ::core::convert::From<NetworkOperatorTetheringOperationResult> for ::windows::core::IUnknown {
    fn from(value: NetworkOperatorTetheringOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringOperationResult> for ::windows::core::IUnknown {
    fn from(value: &NetworkOperatorTetheringOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkOperatorTetheringOperationResult> for ::windows::core::IInspectable {
    fn from(value: NetworkOperatorTetheringOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkOperatorTetheringOperationResult> for ::windows::core::IInspectable {
    fn from(value: &NetworkOperatorTetheringOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NetworkOperatorTetheringOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkRegistrationState(pub i32);
impl NetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl ::core::marker::Copy for NetworkRegistrationState {}
impl ::core::clone::Clone for NetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NetworkRegistrationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkRegistrationState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NetworkRegistrationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkRegistrationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: Self = Self(0i32);
    pub const Wwan: Self = Self(1i32);
}
impl ::core::marker::Copy for ProfileMediaType {}
impl ::core::clone::Clone for ProfileMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProfileMediaType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProfileMediaType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProfileMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProfileMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProfileMediaType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ProfileMediaType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct ProfileUsage {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::DateTime,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ProfileUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ProfileUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProfileUsage").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for ProfileUsage {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for ProfileUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ProfileUsage;u4;struct(Windows.Foundation.DateTime;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ProfileUsage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ProfileUsage>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for ProfileUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ProvisionFromXmlDocumentResults(::windows::core::IUnknown);
impl ProvisionFromXmlDocumentResults {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AllElementsProvisioned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllElementsProvisioned)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ProvisionResultsXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProvisionResultsXml)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ProvisionFromXmlDocumentResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProvisionFromXmlDocumentResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvisionFromXmlDocumentResults {}
impl ::core::fmt::Debug for ProvisionFromXmlDocumentResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvisionFromXmlDocumentResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProvisionFromXmlDocumentResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ProvisionFromXmlDocumentResults;{217700e0-8203-11df-adb9-f4ce462d9137})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_Vtbl;
    const IID: ::windows::core::GUID = <IProvisionFromXmlDocumentResults as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProvisionFromXmlDocumentResults {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionFromXmlDocumentResults";
}
impl ::core::convert::From<ProvisionFromXmlDocumentResults> for ::windows::core::IUnknown {
    fn from(value: ProvisionFromXmlDocumentResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvisionFromXmlDocumentResults> for ::windows::core::IUnknown {
    fn from(value: &ProvisionFromXmlDocumentResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProvisionFromXmlDocumentResults> for ::windows::core::IInspectable {
    fn from(value: ProvisionFromXmlDocumentResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvisionFromXmlDocumentResults> for ::windows::core::IInspectable {
    fn from(value: &ProvisionFromXmlDocumentResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProvisionFromXmlDocumentResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ProvisionedProfile(::windows::core::IUnknown);
impl ProvisionedProfile {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn UpdateCost(&self, value: super::Connectivity::NetworkCostType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateCost)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateUsage<'a, Param0: ::windows::core::IntoParam<'a, ProfileUsage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateUsage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ProvisionedProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProvisionedProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvisionedProfile {}
impl ::core::fmt::Debug for ProvisionedProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvisionedProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProvisionedProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ProvisionedProfile;{217700e0-8202-11df-adb9-f4ce462d9137})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProvisionedProfile {
    type Vtable = IProvisionedProfile_Vtbl;
    const IID: ::windows::core::GUID = <IProvisionedProfile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProvisionedProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionedProfile";
}
impl ::core::convert::From<ProvisionedProfile> for ::windows::core::IUnknown {
    fn from(value: ProvisionedProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvisionedProfile> for ::windows::core::IUnknown {
    fn from(value: &ProvisionedProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProvisionedProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProvisionedProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProvisionedProfile> for ::windows::core::IInspectable {
    fn from(value: ProvisionedProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvisionedProfile> for ::windows::core::IInspectable {
    fn from(value: &ProvisionedProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProvisionedProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProvisionedProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ProvisioningAgent(::windows::core::IUnknown);
impl ProvisioningAgent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProvisioningAgent, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProvisionFromXmlDocumentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, provisioningxmldocument: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProvisionFromXmlDocumentAsync)(::core::mem::transmute_copy(this), provisioningxmldocument.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetProvisionedProfile<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, mediatype: ProfileMediaType, profilename: Param1) -> ::windows::core::Result<ProvisionedProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProvisionedProfile)(::core::mem::transmute_copy(this), mediatype, profilename.into_param().abi(), &mut result__).from_abi::<ProvisionedProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkaccountid: Param0) -> ::windows::core::Result<ProvisioningAgent> {
        Self::IProvisioningAgentStaticMethods(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromNetworkAccountId)(::core::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<ProvisioningAgent>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProvisioningAgentStaticMethods<R, F: FnOnce(&IProvisioningAgentStaticMethods) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProvisioningAgent, IProvisioningAgentStaticMethods> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProvisioningAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProvisioningAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvisioningAgent {}
impl ::core::fmt::Debug for ProvisioningAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvisioningAgent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProvisioningAgent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.ProvisioningAgent;{217700e0-8201-11df-adb9-f4ce462d9137})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProvisioningAgent {
    type Vtable = IProvisioningAgent_Vtbl;
    const IID: ::windows::core::GUID = <IProvisioningAgent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProvisioningAgent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisioningAgent";
}
impl ::core::convert::From<ProvisioningAgent> for ::windows::core::IUnknown {
    fn from(value: ProvisioningAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvisioningAgent> for ::windows::core::IUnknown {
    fn from(value: &ProvisioningAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProvisioningAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProvisioningAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProvisioningAgent> for ::windows::core::IInspectable {
    fn from(value: ProvisioningAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProvisioningAgent> for ::windows::core::IInspectable {
    fn from(value: &ProvisioningAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProvisioningAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProvisioningAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TetheringCapability(pub i32);
impl TetheringCapability {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledByGroupPolicy: Self = Self(1i32);
    pub const DisabledByHardwareLimitation: Self = Self(2i32);
    pub const DisabledByOperator: Self = Self(3i32);
    pub const DisabledBySku: Self = Self(4i32);
    pub const DisabledByRequiredAppNotInstalled: Self = Self(5i32);
    pub const DisabledDueToUnknownCause: Self = Self(6i32);
    pub const DisabledBySystemCapability: Self = Self(7i32);
}
impl ::core::marker::Copy for TetheringCapability {}
impl ::core::clone::Clone for TetheringCapability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringCapability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TetheringCapability {
    type Abi = Self;
}
impl ::core::fmt::Debug for TetheringCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringCapability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TetheringCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringCapability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct TetheringEntitlementCheckTriggerDetails(::windows::core::IUnknown);
impl TetheringEntitlementCheckTriggerDetails {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn AllowTethering(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AllowTethering)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DenyTethering<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, entitlementfailurereason: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DenyTethering)(::core::mem::transmute_copy(this), entitlementfailurereason.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for TetheringEntitlementCheckTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TetheringEntitlementCheckTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TetheringEntitlementCheckTriggerDetails {}
impl ::core::fmt::Debug for TetheringEntitlementCheckTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringEntitlementCheckTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TetheringEntitlementCheckTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.TetheringEntitlementCheckTriggerDetails;{03c65e9d-5926-41f3-a94e-b50926fc421b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <ITetheringEntitlementCheckTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TetheringEntitlementCheckTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.TetheringEntitlementCheckTriggerDetails";
}
impl ::core::convert::From<TetheringEntitlementCheckTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: TetheringEntitlementCheckTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TetheringEntitlementCheckTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &TetheringEntitlementCheckTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TetheringEntitlementCheckTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: TetheringEntitlementCheckTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TetheringEntitlementCheckTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &TetheringEntitlementCheckTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TetheringEntitlementCheckTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TetheringEntitlementCheckTriggerDetails {}
unsafe impl ::core::marker::Sync for TetheringEntitlementCheckTriggerDetails {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const MobileBroadbandDeviceOff: Self = Self(2i32);
    pub const WiFiDeviceOff: Self = Self(3i32);
    pub const EntitlementCheckTimeout: Self = Self(4i32);
    pub const EntitlementCheckFailure: Self = Self(5i32);
    pub const OperationInProgress: Self = Self(6i32);
    pub const BluetoothDeviceOff: Self = Self(7i32);
    pub const NetworkLimitedConnectivity: Self = Self(8i32);
}
impl ::core::marker::Copy for TetheringOperationStatus {}
impl ::core::clone::Clone for TetheringOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TetheringOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for TetheringOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TetheringOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const InTransition: Self = Self(3i32);
}
impl ::core::marker::Copy for TetheringOperationalState {}
impl ::core::clone::Clone for TetheringOperationalState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringOperationalState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TetheringOperationalState {
    type Abi = Self;
}
impl ::core::fmt::Debug for TetheringOperationalState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringOperationalState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TetheringOperationalState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationalState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: Self = Self(0i32);
    pub const TwoPointFourGigahertz: Self = Self(1i32);
    pub const FiveGigahertz: Self = Self(2i32);
}
impl ::core::marker::Copy for TetheringWiFiBand {}
impl ::core::clone::Clone for TetheringWiFiBand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringWiFiBand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TetheringWiFiBand {
    type Abi = Self;
}
impl ::core::fmt::Debug for TetheringWiFiBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringWiFiBand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TetheringWiFiBand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringWiFiBand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const Pin1: Self = Self(1i32);
    pub const Pin2: Self = Self(2i32);
    pub const Pin3: Self = Self(3i32);
    pub const Pin4: Self = Self(4i32);
    pub const Administrative5: Self = Self(5i32);
    pub const Administrative6: Self = Self(6i32);
    pub const NeverAllowed: Self = Self(7i32);
}
impl ::core::marker::Copy for UiccAccessCondition {}
impl ::core::clone::Clone for UiccAccessCondition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UiccAccessCondition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UiccAccessCondition {
    type Abi = Self;
}
impl ::core::fmt::Debug for UiccAccessCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAccessCondition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UiccAccessCondition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAccessCondition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: Self = Self(0i32);
    pub const MF: Self = Self(1i32);
    pub const MFSim: Self = Self(2i32);
    pub const MFRuim: Self = Self(3i32);
    pub const USim: Self = Self(4i32);
    pub const CSim: Self = Self(5i32);
    pub const ISim: Self = Self(6i32);
}
impl ::core::marker::Copy for UiccAppKind {}
impl ::core::clone::Clone for UiccAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UiccAppKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UiccAppKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UiccAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAppKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UiccAppKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: Self = Self(0i32);
    pub const Transparent: Self = Self(1i32);
    pub const RecordOriented: Self = Self(2i32);
}
impl ::core::marker::Copy for UiccAppRecordKind {}
impl ::core::clone::Clone for UiccAppRecordKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UiccAppRecordKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UiccAppRecordKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UiccAppRecordKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAppRecordKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UiccAppRecordKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppRecordKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UssdMessage(::windows::core::IUnknown);
impl UssdMessage {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn DataCodingScheme(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataCodingScheme)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetDataCodingScheme(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataCodingScheme)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn GetPayload(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPayload)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetPayload(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPayload)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn PayloadAsText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PayloadAsText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn SetPayloadAsText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPayloadAsText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(messagetext: Param0) -> ::windows::core::Result<UssdMessage> {
        Self::IUssdMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMessage)(::core::mem::transmute_copy(this), messagetext.into_param().abi(), &mut result__).from_abi::<UssdMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUssdMessageFactory<R, F: FnOnce(&IUssdMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UssdMessage, IUssdMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UssdMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UssdMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UssdMessage {}
impl ::core::fmt::Debug for UssdMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UssdMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.UssdMessage;{2f9acf82-2004-4d5d-bf81-2aba1b4be4a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UssdMessage {
    type Vtable = IUssdMessage_Vtbl;
    const IID: ::windows::core::GUID = <IUssdMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UssdMessage {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdMessage";
}
impl ::core::convert::From<UssdMessage> for ::windows::core::IUnknown {
    fn from(value: UssdMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UssdMessage> for ::windows::core::IUnknown {
    fn from(value: &UssdMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UssdMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UssdMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UssdMessage> for ::windows::core::IInspectable {
    fn from(value: UssdMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UssdMessage> for ::windows::core::IInspectable {
    fn from(value: &UssdMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UssdMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UssdMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UssdMessage {}
unsafe impl ::core::marker::Sync for UssdMessage {}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UssdReply(::windows::core::IUnknown);
impl UssdReply {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn ResultCode(&self) -> ::windows::core::Result<UssdResultCode> {
        let this = self;
        unsafe {
            let mut result__: UssdResultCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResultCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UssdResultCode>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<UssdMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UssdMessage>(result__)
        }
    }
}
impl ::core::clone::Clone for UssdReply {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UssdReply {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UssdReply {}
impl ::core::fmt::Debug for UssdReply {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdReply").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UssdReply {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.UssdReply;{2f9acf82-2005-4d5d-bf81-2aba1b4be4a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UssdReply {
    type Vtable = IUssdReply_Vtbl;
    const IID: ::windows::core::GUID = <IUssdReply as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UssdReply {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdReply";
}
impl ::core::convert::From<UssdReply> for ::windows::core::IUnknown {
    fn from(value: UssdReply) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UssdReply> for ::windows::core::IUnknown {
    fn from(value: &UssdReply) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UssdReply {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UssdReply {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UssdReply> for ::windows::core::IInspectable {
    fn from(value: UssdReply) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UssdReply> for ::windows::core::IInspectable {
    fn from(value: &UssdReply) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UssdReply {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UssdReply {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: Self = Self(0i32);
    pub const ActionRequired: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const OtherLocalClient: Self = Self(3i32);
    pub const OperationNotSupported: Self = Self(4i32);
    pub const NetworkTimeout: Self = Self(5i32);
}
impl ::core::marker::Copy for UssdResultCode {}
impl ::core::clone::Clone for UssdResultCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UssdResultCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UssdResultCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for UssdResultCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdResultCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UssdResultCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UssdResultCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UssdSession(::windows::core::IUnknown);
impl UssdSession {
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAndGetReplyAsync<'a, Param0: ::windows::core::IntoParam<'a, UssdMessage>>(&self, message: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UssdReply>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendMessageAndGetReplyAsync)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UssdReply>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateFromNetworkAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkaccountid: Param0) -> ::windows::core::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromNetworkAccountId)(::core::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<UssdSession>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
    pub fn CreateFromNetworkInterfaceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(networkinterfaceid: Param0) -> ::windows::core::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromNetworkInterfaceId)(::core::mem::transmute_copy(this), networkinterfaceid.into_param().abi(), &mut result__).from_abi::<UssdSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUssdSessionStatics<R, F: FnOnce(&IUssdSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UssdSession, IUssdSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UssdSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UssdSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UssdSession {}
impl ::core::fmt::Debug for UssdSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UssdSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.NetworkOperators.UssdSession;{2f9acf82-2002-4d5d-bf81-2aba1b4be4a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UssdSession {
    type Vtable = IUssdSession_Vtbl;
    const IID: ::windows::core::GUID = <IUssdSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UssdSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdSession";
}
impl ::core::convert::From<UssdSession> for ::windows::core::IUnknown {
    fn from(value: UssdSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UssdSession> for ::windows::core::IUnknown {
    fn from(value: &UssdSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UssdSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UssdSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UssdSession> for ::windows::core::IInspectable {
    fn from(value: UssdSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UssdSession> for ::windows::core::IInspectable {
    fn from(value: &UssdSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UssdSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UssdSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
