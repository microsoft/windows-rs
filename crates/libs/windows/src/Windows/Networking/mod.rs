#[cfg(feature = "Networking_BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Networking_Connectivity")]
pub mod Connectivity;
#[cfg(feature = "Networking_NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Networking_Proximity")]
pub mod Proximity;
#[cfg(feature = "Networking_PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "Networking_ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Networking_Sockets")]
pub mod Sockets;
#[cfg(feature = "Networking_Vpn")]
pub mod Vpn;
#[cfg(feature = "Networking_XboxLive")]
pub mod XboxLive;
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPair(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEndpointPair {
    type Vtable = IEndpointPair_Vtbl;
}
impl ::core::clone::Clone for IEndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEndpointPair {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPair_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPairFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEndpointPairFactory {
    type Vtable = IEndpointPairFactory_Vtbl;
}
impl ::core::clone::Clone for IEndpointPairFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEndpointPairFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb609d971_64e0_442b_aa6f_cc8c8f181f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPairFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateEndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostName(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostName {
    type Vtable = IHostName_Vtbl;
}
impl ::core::clone::Clone for IHostName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8ecaad_ed96_49a7_9084_d416cae88dcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostName_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub IPInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    IPInformation: usize,
    pub RawName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostNameFactory {
    type Vtable = IHostNameFactory_Vtbl;
}
impl ::core::clone::Clone for IHostNameFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostNameFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458c23ed_712f_4576_adf1_c20b2c643558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostNameStatics {
    type Vtable = IHostNameStatics_Vtbl;
}
impl ::core::clone::Clone for IHostNameStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostNameStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf68cd4bf_a388_4e8b_91ea_54dd6dd901c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value2: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct EndpointPair(::windows::core::IUnknown);
impl EndpointPair {
    pub fn LocalHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HostName>();
            (::windows::core::Interface::vtable(this).LocalHostName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLocalHostName(&self, value: &HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalHostName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LocalServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalServiceName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLocalServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalServiceName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HostName>();
            (::windows::core::Interface::vtable(this).RemoteHostName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteHostName(&self, value: &HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteHostName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemoteServiceName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteServiceName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateEndpointPair(localhostname: &HostName, localservicename: &::windows::core::HSTRING, remotehostname: &HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<EndpointPair> {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EndpointPair>();
            (::windows::core::Interface::vtable(this).CreateEndpointPair)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localhostname), ::core::mem::transmute_copy(localservicename), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEndpointPairFactory<R, F: FnOnce(&IEndpointPairFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EndpointPair, IEndpointPairFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EndpointPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EndpointPair {}
impl ::core::fmt::Debug for EndpointPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointPair").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EndpointPair {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.EndpointPair;{33a0aa36-f8fa-4b30-b856-76517c3bd06d})");
}
impl ::core::clone::Clone for EndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EndpointPair {
    type Vtable = IEndpointPair_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EndpointPair {
    const IID: ::windows::core::GUID = <IEndpointPair as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
::windows::imp::interface_hierarchy!(EndpointPair, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EndpointPair {}
unsafe impl ::core::marker::Sync for EndpointPair {}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct HostName(::windows::core::IUnknown);
impl HostName {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn IPInformation(&self) -> ::windows::core::Result<Connectivity::IPInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Connectivity::IPInformation>();
            (::windows::core::Interface::vtable(this).IPInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RawName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanonicalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CanonicalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HostNameType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEqual(&self, hostname: &HostName) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEqual)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(hostname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateHostName(hostname: &::windows::core::HSTRING) -> ::windows::core::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HostName>();
            (::windows::core::Interface::vtable(this).CreateHostName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(hostname), &mut result__).from_abi(result__)
        })
    }
    pub fn Compare(value1: &::windows::core::HSTRING, value2: &::windows::core::HSTRING) -> ::windows::core::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Compare)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), ::core::mem::transmute_copy(value2), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HostName, IHostNameFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HostName, IHostNameStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HostName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HostName {}
impl ::core::fmt::Debug for HostName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostName").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HostName {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.HostName;{bf8ecaad-ed96-49a7-9084-d416cae88dcb})");
}
impl ::core::clone::Clone for HostName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HostName {
    type Vtable = IHostName_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HostName {
    const IID: ::windows::core::GUID = <IHostName as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
::windows::imp::interface_hierarchy!(HostName, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::Foundation::IStringable> for HostName {}
unsafe impl ::core::marker::Send for HostName {}
unsafe impl ::core::marker::Sync for HostName {}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl ::core::marker::Copy for DomainNameType {}
impl ::core::clone::Clone for DomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DomainNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DomainNameType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DomainNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainNameType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DomainNameType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl ::core::marker::Copy for HostNameSortOptions {}
impl ::core::clone::Clone for HostNameSortOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HostNameSortOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HostNameSortOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HostNameSortOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameSortOptions").field(&self.0).finish()
    }
}
impl HostNameSortOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for HostNameSortOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HostNameSortOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HostNameSortOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HostNameSortOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HostNameSortOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for HostNameSortOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameSortOptions;u4)");
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for HostNameType {}
impl ::core::clone::Clone for HostNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HostNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HostNameType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HostNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HostNameType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
