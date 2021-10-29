#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataAvailabilityStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2808447689, 1698, 17011, [168, 3, 131, 76, 159, 135, 251, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAvailabilityStateChangedEventArgs_abi(
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
pub struct IUserDataBufferUnprotectResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2398949008, 64154, 18084, [163, 119, 1, 206, 191, 30, 116, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataBufferUnprotectResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataBufferUnprotectStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataProtectionManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(521347965, 46126, 19080, [148, 128, 15, 36, 9, 36, 200, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storageitem: ::windows::runtime::RawPtr, availability: UserDataAvailability, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storageitem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unprotectedbuffer: ::windows::runtime::RawPtr, availability: UserDataAvailability, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectedbuffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availability: UserDataAvailability, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataProtectionManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataProtectionManagerStatics {
    type Vtable = IUserDataProtectionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541191400, 28110, 20398, [175, 133, 120, 42, 194, 207, 69, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataProtectionManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataStorageItemProtectionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1533444342, 59519, 16545, [177, 157, 166, 24, 122, 12, 102, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataStorageItemProtectionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataAvailability) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAvailability(pub i32);
impl UserDataAvailability {
    pub const Always: UserDataAvailability = UserDataAvailability(0i32);
    pub const AfterFirstUnlock: UserDataAvailability = UserDataAvailability(1i32);
    pub const WhileUnlocked: UserDataAvailability = UserDataAvailability(2i32);
}
impl ::std::convert::From<i32> for UserDataAvailability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAvailability {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAvailability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataAvailability;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAvailabilityStateChangedEventArgs(::windows::runtime::IInspectable);
impl UserDataAvailabilityStateChangedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAvailabilityStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs;{a76582c9-06a2-4273-a803-834c9f87fbeb})");
}
unsafe impl ::windows::runtime::Interface for UserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2808447689, 1698, 17011, [168, 3, 131, 76, 159, 135, 251, 235]);
}
impl ::windows::runtime::RuntimeName for UserDataAvailabilityStateChangedEventArgs {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs";
}
impl ::std::convert::From<UserDataAvailabilityStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAvailabilityStateChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAvailabilityStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAvailabilityStateChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAvailabilityStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAvailabilityStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAvailabilityStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAvailabilityStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAvailabilityStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataAvailabilityStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for UserDataAvailabilityStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataBufferUnprotectResult(::windows::runtime::IInspectable);
impl UserDataBufferUnprotectResult {
    pub fn Status(&self) -> ::windows::runtime::Result<UserDataBufferUnprotectStatus> {
        let this = self;
        unsafe {
            let mut result__: UserDataBufferUnprotectStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataBufferUnprotectStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn UnprotectedBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataBufferUnprotectResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataBufferUnprotectResult;{8efd0e90-fa9a-46a4-a377-01cebf1e74d8})");
}
unsafe impl ::windows::runtime::Interface for UserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2398949008, 64154, 18084, [163, 119, 1, 206, 191, 30, 116, 216]);
}
impl ::windows::runtime::RuntimeName for UserDataBufferUnprotectResult {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataBufferUnprotectResult";
}
impl ::std::convert::From<UserDataBufferUnprotectResult> for ::windows::runtime::IUnknown {
    fn from(value: UserDataBufferUnprotectResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataBufferUnprotectResult> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataBufferUnprotectResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataBufferUnprotectResult> for ::windows::runtime::IInspectable {
    fn from(value: UserDataBufferUnprotectResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataBufferUnprotectResult> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataBufferUnprotectResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataBufferUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataBufferUnprotectResult {}
unsafe impl ::std::marker::Sync for UserDataBufferUnprotectResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataBufferUnprotectStatus(pub i32);
impl UserDataBufferUnprotectStatus {
    pub const Succeeded: UserDataBufferUnprotectStatus = UserDataBufferUnprotectStatus(0i32);
    pub const Unavailable: UserDataBufferUnprotectStatus = UserDataBufferUnprotectStatus(1i32);
}
impl ::std::convert::From<i32> for UserDataBufferUnprotectStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataBufferUnprotectStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataBufferUnprotectStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataBufferUnprotectStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataProtectionManager(::windows::runtime::IInspectable);
impl UserDataProtectionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectStorageItemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(&self, storageitem: Param0, availability: UserDataAvailability) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), storageitem.into_param().abi(), availability, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetStorageItemProtectionInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(&self, storageitem: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), storageitem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectBufferAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, unprotectedbuffer: Param0, availability: UserDataAvailability) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), unprotectedbuffer.into_param().abi(), availability, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectBufferAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, protectedbuffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), protectedbuffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>(result__)
        }
    }
    pub fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), availability, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DataAvailabilityStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataAvailabilityStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn TryGetDefault() -> ::windows::runtime::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataProtectionManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn TryGetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataProtectionManager>(result__)
        })
    }
    pub fn IUserDataProtectionManagerStatics<R, F: FnOnce(&IUserDataProtectionManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataProtectionManager, IUserDataProtectionManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataProtectionManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataProtectionManager;{1f13237d-b42e-4a88-9480-0f240924c876})");
}
unsafe impl ::windows::runtime::Interface for UserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(521347965, 46126, 19080, [148, 128, 15, 36, 9, 36, 200, 118]);
}
impl ::windows::runtime::RuntimeName for UserDataProtectionManager {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataProtectionManager";
}
impl ::std::convert::From<UserDataProtectionManager> for ::windows::runtime::IUnknown {
    fn from(value: UserDataProtectionManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataProtectionManager> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataProtectionManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataProtectionManager> for ::windows::runtime::IInspectable {
    fn from(value: UserDataProtectionManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataProtectionManager> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataProtectionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataProtectionManager {}
unsafe impl ::std::marker::Sync for UserDataProtectionManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataStorageItemProtectionInfo(::windows::runtime::IInspectable);
impl UserDataStorageItemProtectionInfo {
    pub fn Availability(&self) -> ::windows::runtime::Result<UserDataAvailability> {
        let this = self;
        unsafe {
            let mut result__: UserDataAvailability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAvailability>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataStorageItemProtectionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.DataProtection.UserDataStorageItemProtectionInfo;{5b6680f6-e87f-40a1-b19d-a6187a0c662f})");
}
unsafe impl ::windows::runtime::Interface for UserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1533444342, 59519, 16545, [177, 157, 166, 24, 122, 12, 102, 47]);
}
impl ::windows::runtime::RuntimeName for UserDataStorageItemProtectionInfo {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataStorageItemProtectionInfo";
}
impl ::std::convert::From<UserDataStorageItemProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: UserDataStorageItemProtectionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataStorageItemProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataStorageItemProtectionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataStorageItemProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: UserDataStorageItemProtectionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataStorageItemProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataStorageItemProtectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataStorageItemProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataStorageItemProtectionInfo {}
unsafe impl ::std::marker::Sync for UserDataStorageItemProtectionInfo {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataStorageItemProtectionStatus(pub i32);
impl UserDataStorageItemProtectionStatus {
    pub const Succeeded: UserDataStorageItemProtectionStatus = UserDataStorageItemProtectionStatus(0i32);
    pub const NotProtectable: UserDataStorageItemProtectionStatus = UserDataStorageItemProtectionStatus(1i32);
    pub const DataUnavailable: UserDataStorageItemProtectionStatus = UserDataStorageItemProtectionStatus(2i32);
}
impl ::std::convert::From<i32> for UserDataStorageItemProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataStorageItemProtectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataStorageItemProtectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataStorageItemProtectionStatus;i4)");
}
