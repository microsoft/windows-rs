#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAvailabilityStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa76582c9_06a2_4273_a803_834c9f87fbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAvailabilityStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataBufferUnprotectResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efd0e90_fa9a_46a4_a377_01cebf1e74d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataBufferUnprotectResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataBufferUnprotectStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UnprotectedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UnprotectedBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataProtectionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f13237d_b42e_4a88_9480_0f240924c876);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectStorageItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, availability: UserDataAvailability, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectStorageItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetStorageItemProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetStorageItemProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unprotectedbuffer: ::windows::core::RawPtr, availability: UserDataAvailability, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectBufferAsync: usize,
    pub IsContinuedDataAvailabilityExpected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IUserDataProtectionManagerStatics {
    type Vtable = IUserDataProtectionManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x977780e8_6dce_4fae_af85_782ac2cf4572);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryGetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub TryGetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TryGetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataStorageItemProtectionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b6680f6_e87f_40a1_b19d_a6187a0c662f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataStorageItemProtectionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAvailability) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct UserDataAvailabilityStateChangedEventArgs(::windows::core::IUnknown);
impl UserDataAvailabilityStateChangedEventArgs {
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
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
unsafe impl ::windows::core::Interface for UserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAvailabilityStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAvailabilityStateChangedEventArgs {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs";
}
impl ::core::convert::From<UserDataAvailabilityStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDataAvailabilityStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAvailabilityStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDataAvailabilityStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAvailabilityStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDataAvailabilityStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAvailabilityStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDataAvailabilityStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAvailabilityStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAvailabilityStateChangedEventArgs {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataBufferUnprotectResult(::windows::core::IUnknown);
impl UserDataBufferUnprotectResult {
    #[doc = "*Required features: `\"Security_DataProtection\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<UserDataBufferUnprotectStatus> {
        let this = self;
        unsafe {
            let mut result__: UserDataBufferUnprotectStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataBufferUnprotectStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn UnprotectedBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectedBuffer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
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
unsafe impl ::windows::core::Interface for UserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataBufferUnprotectResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataBufferUnprotectResult {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataBufferUnprotectResult";
}
impl ::core::convert::From<UserDataBufferUnprotectResult> for ::windows::core::IUnknown {
    fn from(value: UserDataBufferUnprotectResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataBufferUnprotectResult> for ::windows::core::IUnknown {
    fn from(value: &UserDataBufferUnprotectResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataBufferUnprotectResult> for ::windows::core::IInspectable {
    fn from(value: UserDataBufferUnprotectResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataBufferUnprotectResult> for ::windows::core::IInspectable {
    fn from(value: &UserDataBufferUnprotectResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataBufferUnprotectResult {}
unsafe impl ::core::marker::Sync for UserDataBufferUnprotectResult {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct UserDataProtectionManager(::windows::core::IUnknown);
impl UserDataProtectionManager {
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectStorageItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(&self, storageitem: Param0, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectStorageItemAsync)(::core::mem::transmute_copy(this), storageitem.into_param().abi(), availability, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetStorageItemProtectionInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(&self, storageitem: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStorageItemProtectionInfoAsync)(::core::mem::transmute_copy(this), storageitem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, unprotectedbuffer: Param0, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectBufferAsync)(::core::mem::transmute_copy(this), unprotectedbuffer.into_param().abi(), availability, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, protectedbuffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectBufferAsync)(::core::mem::transmute_copy(this), protectedbuffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`*"]
    pub fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsContinuedDataAvailabilityExpected)(::core::mem::transmute_copy(this), availability, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataAvailabilityStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataAvailabilityStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataAvailabilityStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataAvailabilityStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`*"]
    pub fn TryGetDefault() -> ::windows::core::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataProtectionManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_DataProtection\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TryGetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataProtectionManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataProtectionManagerStatics<R, F: FnOnce(&IUserDataProtectionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataProtectionManager, IUserDataProtectionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for UserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataProtectionManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataProtectionManager {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataProtectionManager";
}
impl ::core::convert::From<UserDataProtectionManager> for ::windows::core::IUnknown {
    fn from(value: UserDataProtectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataProtectionManager> for ::windows::core::IUnknown {
    fn from(value: &UserDataProtectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataProtectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataProtectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataProtectionManager> for ::windows::core::IInspectable {
    fn from(value: UserDataProtectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataProtectionManager> for ::windows::core::IInspectable {
    fn from(value: &UserDataProtectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataProtectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataProtectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataProtectionManager {}
unsafe impl ::core::marker::Sync for UserDataProtectionManager {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataStorageItemProtectionInfo(::windows::core::IUnknown);
impl UserDataStorageItemProtectionInfo {
    #[doc = "*Required features: `\"Security_DataProtection\"`*"]
    pub fn Availability(&self) -> ::windows::core::Result<UserDataAvailability> {
        let this = self;
        unsafe {
            let mut result__: UserDataAvailability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Availability)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAvailability>(result__)
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
unsafe impl ::windows::core::Interface for UserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataStorageItemProtectionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataStorageItemProtectionInfo {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataStorageItemProtectionInfo";
}
impl ::core::convert::From<UserDataStorageItemProtectionInfo> for ::windows::core::IUnknown {
    fn from(value: UserDataStorageItemProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataStorageItemProtectionInfo> for ::windows::core::IUnknown {
    fn from(value: &UserDataStorageItemProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataStorageItemProtectionInfo> for ::windows::core::IInspectable {
    fn from(value: UserDataStorageItemProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataStorageItemProtectionInfo> for ::windows::core::IInspectable {
    fn from(value: &UserDataStorageItemProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataStorageItemProtectionInfo {}
unsafe impl ::core::marker::Sync for UserDataStorageItemProtectionInfo {}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
