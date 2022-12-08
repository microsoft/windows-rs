#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAvailabilityStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataAvailabilityStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa76582c9_06a2_4273_a803_834c9f87fbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAvailabilityStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataBufferUnprotectResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataBufferUnprotectResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efd0e90_fa9a_46a4_a377_01cebf1e74d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataBufferUnprotectResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataBufferUnprotectStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UnprotectedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UnprotectedBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataProtectionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataProtectionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f13237d_b42e_4a88_9480_0f240924c876);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectStorageItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectStorageItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetStorageItemProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetStorageItemProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unprotectedbuffer: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectBufferAsync: usize,
    pub IsContinuedDataAvailabilityExpected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataAvailabilityStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataAvailabilityStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataProtectionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataProtectionManagerStatics {
    type Vtable = IUserDataProtectionManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataProtectionManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x977780e8_6dce_4fae_af85_782ac2cf4572);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryGetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub TryGetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TryGetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataStorageItemProtectionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataStorageItemProtectionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b6680f6_e87f_40a1_b19d_a6187a0c662f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataStorageItemProtectionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAvailability) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataAvailabilityStateChangedEventArgs(::windows::core::IUnknown);
impl UserDataAvailabilityStateChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAvailabilityStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for UserDataAvailabilityStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs;{a76582c9-06a2-4273-a803-834c9f87fbeb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataAvailabilityStateChangedEventArgs {
    const IID: ::windows::core::GUID = <IUserDataAvailabilityStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAvailabilityStateChangedEventArgs {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs";
}
::windows::core::interface_hierarchy!(UserDataAvailabilityStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataAvailabilityStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAvailabilityStateChangedEventArgs {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataBufferUnprotectResult(::windows::core::IUnknown);
impl UserDataBufferUnprotectResult {
    pub fn Status(&self) -> ::windows::core::Result<UserDataBufferUnprotectStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn UnprotectedBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnprotectedBuffer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataBufferUnprotectResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for UserDataBufferUnprotectResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataBufferUnprotectResult;{8efd0e90-fa9a-46a4-a377-01cebf1e74d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataBufferUnprotectResult {
    const IID: ::windows::core::GUID = <IUserDataBufferUnprotectResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataBufferUnprotectResult {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataBufferUnprotectResult";
}
::windows::core::interface_hierarchy!(UserDataBufferUnprotectResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataBufferUnprotectResult {}
unsafe impl ::core::marker::Sync for UserDataBufferUnprotectResult {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataProtectionManager(::windows::core::IUnknown);
impl UserDataProtectionManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectStorageItemAsync<P0, E0>(&self, storageitem: P0, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageItem>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectStorageItemAsync)(::windows::core::Vtable::as_raw(this), storageitem.try_into().map_err(|e| e.into())?.abi(), availability, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetStorageItemProtectionInfoAsync<P0, E0>(&self, storageitem: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageItem>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStorageItemProtectionInfoAsync)(::windows::core::Vtable::as_raw(this), storageitem.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectBufferAsync<P0, E0>(&self, unprotectedbuffer: P0, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectBufferAsync)(::windows::core::Vtable::as_raw(this), unprotectedbuffer.try_into().map_err(|e| e.into())?.abi(), availability, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectBufferAsync<P0, E0>(&self, protectedbuffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnprotectBufferAsync)(::windows::core::Vtable::as_raw(this), protectedbuffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsContinuedDataAvailabilityExpected)(::windows::core::Vtable::as_raw(this), availability, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataAvailabilityStateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataAvailabilityStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataAvailabilityStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDataAvailabilityStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn TryGetDefault() -> ::windows::core::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TryGetForUser(user: &super::super::System::User) -> ::windows::core::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataProtectionManagerStatics<R, F: FnOnce(&IUserDataProtectionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserDataProtectionManager, IUserDataProtectionManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserDataProtectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for UserDataProtectionManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataProtectionManager;{1f13237d-b42e-4a88-9480-0f240924c876})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataProtectionManager {
    const IID: ::windows::core::GUID = <IUserDataProtectionManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataProtectionManager {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataProtectionManager";
}
::windows::core::interface_hierarchy!(UserDataProtectionManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataProtectionManager {}
unsafe impl ::core::marker::Sync for UserDataProtectionManager {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataStorageItemProtectionInfo(::windows::core::IUnknown);
impl UserDataStorageItemProtectionInfo {
    pub fn Availability(&self) -> ::windows::core::Result<UserDataAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Availability)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataStorageItemProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for UserDataStorageItemProtectionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataStorageItemProtectionInfo;{5b6680f6-e87f-40a1-b19d-a6187a0c662f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataStorageItemProtectionInfo {
    const IID: ::windows::core::GUID = <IUserDataStorageItemProtectionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataStorageItemProtectionInfo {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataStorageItemProtectionInfo";
}
::windows::core::interface_hierarchy!(UserDataStorageItemProtectionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Abi for UserDataAvailability {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataAvailability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for UserDataBufferUnprotectStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataBufferUnprotectStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataBufferUnprotectStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataBufferUnprotectStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataBufferUnprotectStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for UserDataStorageItemProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataStorageItemProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataStorageItemProtectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataStorageItemProtectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataStorageItemProtectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
