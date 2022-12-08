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
unsafe impl ::windows::core::Vtable for IEndpointPair {
    type Vtable = IEndpointPair_Vtbl;
}
unsafe impl ::windows::core::Interface for IEndpointPair {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPair_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPairFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEndpointPairFactory {
    type Vtable = IEndpointPairFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IEndpointPairFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb609d971_64e0_442b_aa6f_cc8c8f181f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPairFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateEndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: *mut ::core::ffi::c_void, localservicename: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostName(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHostName {
    type Vtable = IHostName_Vtbl;
}
unsafe impl ::windows::core::Interface for IHostName {
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
    pub RawName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHostNameFactory {
    type Vtable = IHostNameFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHostNameFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458c23ed_712f_4576_adf1_c20b2c643558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHostNameStatics {
    type Vtable = IHostNameStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHostNameStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf68cd4bf_a388_4e8b_91ea_54dd6dd901c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: *mut ::core::ffi::c_void, value2: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct EndpointPair(::windows::core::IUnknown);
impl EndpointPair {
    pub fn LocalHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalHostName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLocalHostName(&self, value: &HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocalHostName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LocalServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalServiceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLocalServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocalServiceName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteHostName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRemoteHostName(&self, value: &HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteHostName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteServiceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRemoteServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteServiceName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateEndpointPair(localhostname: &HostName, localservicename: &::windows::core::HSTRING, remotehostname: &HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<EndpointPair> {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateEndpointPair)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localhostname), ::core::mem::transmute_copy(localservicename), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEndpointPairFactory<R, F: FnOnce(&IEndpointPairFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<EndpointPair, IEndpointPairFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for EndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for EndpointPair {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.EndpointPair;{33a0aa36-f8fa-4b30-b856-76517c3bd06d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EndpointPair {
    type Vtable = IEndpointPair_Vtbl;
}
unsafe impl ::windows::core::Interface for EndpointPair {
    const IID: ::windows::core::GUID = <IEndpointPair as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
::windows::core::interface_hierarchy!(EndpointPair, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IPInformation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RawName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanonicalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanonicalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEqual(&self, hostname: &HostName) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEqual)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateHostName(hostname: &::windows::core::HSTRING) -> ::windows::core::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateHostName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Compare(value1: &::windows::core::HSTRING, value2: &::windows::core::HSTRING) -> ::windows::core::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compare)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value1), ::core::mem::transmute_copy(value2), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HostName, IHostNameFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HostName, IHostNameStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HostName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HostName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.HostName;{bf8ecaad-ed96-49a7-9084-d416cae88dcb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HostName {
    type Vtable = IHostName_Vtbl;
}
unsafe impl ::windows::core::Interface for HostName {
    const IID: ::windows::core::GUID = <IHostName as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
::windows::core::interface_hierarchy!(HostName, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HostName> for super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HostName) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HostName> for super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HostName) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HostName> for ::windows::core::InParam<super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HostName) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
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
unsafe impl ::windows::core::Abi for DomainNameType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DomainNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainNameType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DomainNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for HostNameSortOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for HostNameSortOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameSortOptions").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for HostNameSortOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameSortOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for HostNameType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HostNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HostNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
