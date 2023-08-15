#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAvailabilityStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IUserDataAvailabilityStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataAvailabilityStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa76582c9_06a2_4273_a803_834c9f87fbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAvailabilityStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataBufferUnprotectResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
}
impl ::core::clone::Clone for IUserDataBufferUnprotectResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataBufferUnprotectResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8efd0e90_fa9a_46a4_a377_01cebf1e74d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataBufferUnprotectResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataBufferUnprotectStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UnprotectedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UnprotectedBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataProtectionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
}
impl ::core::clone::Clone for IUserDataProtectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataProtectionManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f13237d_b42e_4a88_9480_0f240924c876);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectStorageItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectStorageItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetStorageItemProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetStorageItemProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unprotectedbuffer: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectBufferAsync: usize,
    pub IsContinuedDataAvailabilityExpected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataAvailabilityStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataAvailabilityStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataProtectionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataProtectionManagerStatics {
    type Vtable = IUserDataProtectionManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IUserDataProtectionManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataProtectionManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x977780e8_6dce_4fae_af85_782ac2cf4572);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryGetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub TryGetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TryGetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataStorageItemProtectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
}
impl ::core::clone::Clone for IUserDataStorageItemProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataStorageItemProtectionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b6680f6_e87f_40a1_b19d_a6187a0c662f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataStorageItemProtectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAvailability) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataAvailabilityStateChangedEventArgs(::windows_core::IUnknown);
impl UserDataAvailabilityStateChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UserDataAvailabilityStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAvailabilityStateChangedEventArgs {}
impl ::core::fmt::Debug for UserDataAvailabilityStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAvailabilityStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataAvailabilityStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs;{a76582c9-06a2-4273-a803-834c9f87fbeb})");
}
impl ::core::clone::Clone for UserDataAvailabilityStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataAvailabilityStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IUserDataAvailabilityStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAvailabilityStateChangedEventArgs {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(UserDataAvailabilityStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserDataAvailabilityStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAvailabilityStateChangedEventArgs {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataBufferUnprotectResult(::windows_core::IUnknown);
impl UserDataBufferUnprotectResult {
    pub fn Status(&self) -> ::windows_core::Result<UserDataBufferUnprotectStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn UnprotectedBuffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectedBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UserDataBufferUnprotectResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataBufferUnprotectResult {}
impl ::core::fmt::Debug for UserDataBufferUnprotectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataBufferUnprotectResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataBufferUnprotectResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataBufferUnprotectResult;{8efd0e90-fa9a-46a4-a377-01cebf1e74d8})");
}
impl ::core::clone::Clone for UserDataBufferUnprotectResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataBufferUnprotectResult {
    const IID: ::windows_core::GUID = <IUserDataBufferUnprotectResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataBufferUnprotectResult {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataBufferUnprotectResult";
}
::windows_core::imp::interface_hierarchy!(UserDataBufferUnprotectResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserDataBufferUnprotectResult {}
unsafe impl ::core::marker::Sync for UserDataBufferUnprotectResult {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataProtectionManager(::windows_core::IUnknown);
impl UserDataProtectionManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectStorageItemAsync<P0>(&self, storageitem: P0, availability: UserDataAvailability) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectStorageItemAsync)(::windows_core::Interface::as_raw(this), storageitem.try_into_param()?.abi(), availability, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetStorageItemProtectionInfoAsync<P0>(&self, storageitem: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStorageItemProtectionInfoAsync)(::windows_core::Interface::as_raw(this), storageitem.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectBufferAsync<P0>(&self, unprotectedbuffer: P0, availability: UserDataAvailability) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectBufferAsync)(::windows_core::Interface::as_raw(this), unprotectedbuffer.try_into_param()?.abi(), availability, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectBufferAsync<P0>(&self, protectedbuffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectBufferAsync)(::windows_core::Interface::as_raw(this), protectedbuffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContinuedDataAvailabilityExpected)(::windows_core::Interface::as_raw(this), availability, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataAvailabilityStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataAvailabilityStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataAvailabilityStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDataAvailabilityStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryGetDefault() -> ::windows_core::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TryGetForUser<P0>(user: P0) -> ::windows_core::Result<UserDataProtectionManager>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataProtectionManagerStatics<R, F: FnOnce(&IUserDataProtectionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserDataProtectionManager, IUserDataProtectionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for UserDataProtectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataProtectionManager {}
impl ::core::fmt::Debug for UserDataProtectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataProtectionManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataProtectionManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataProtectionManager;{1f13237d-b42e-4a88-9480-0f240924c876})");
}
impl ::core::clone::Clone for UserDataProtectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataProtectionManager {
    const IID: ::windows_core::GUID = <IUserDataProtectionManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataProtectionManager {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataProtectionManager";
}
::windows_core::imp::interface_hierarchy!(UserDataProtectionManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserDataProtectionManager {}
unsafe impl ::core::marker::Sync for UserDataProtectionManager {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataStorageItemProtectionInfo(::windows_core::IUnknown);
impl UserDataStorageItemProtectionInfo {
    pub fn Availability(&self) -> ::windows_core::Result<UserDataAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Availability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UserDataStorageItemProtectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataStorageItemProtectionInfo {}
impl ::core::fmt::Debug for UserDataStorageItemProtectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataStorageItemProtectionInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataStorageItemProtectionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataStorageItemProtectionInfo;{5b6680f6-e87f-40a1-b19d-a6187a0c662f})");
}
impl ::core::clone::Clone for UserDataStorageItemProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataStorageItemProtectionInfo {
    const IID: ::windows_core::GUID = <IUserDataStorageItemProtectionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataStorageItemProtectionInfo {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataStorageItemProtectionInfo";
}
::windows_core::imp::interface_hierarchy!(UserDataStorageItemProtectionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserDataStorageItemProtectionInfo {}
unsafe impl ::core::marker::Sync for UserDataStorageItemProtectionInfo {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserDataAvailability(pub i32);
impl UserDataAvailability {
    pub const Always: Self = Self(0i32);
    pub const AfterFirstUnlock: Self = Self(1i32);
    pub const WhileUnlocked: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAvailability {}
impl ::core::clone::Clone for UserDataAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAvailability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserDataAvailability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserDataAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAvailability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataAvailability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataAvailability;i4)");
}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserDataBufferUnprotectStatus(pub i32);
impl UserDataBufferUnprotectStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataBufferUnprotectStatus {}
impl ::core::clone::Clone for UserDataBufferUnprotectStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataBufferUnprotectStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserDataBufferUnprotectStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserDataBufferUnprotectStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataBufferUnprotectStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataBufferUnprotectStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataBufferUnprotectStatus;i4)");
}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserDataStorageItemProtectionStatus(pub i32);
impl UserDataStorageItemProtectionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NotProtectable: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataStorageItemProtectionStatus {}
impl ::core::clone::Clone for UserDataStorageItemProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataStorageItemProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserDataStorageItemProtectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserDataStorageItemProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataStorageItemProtectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataStorageItemProtectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataStorageItemProtectionStatus;i4)");
}
