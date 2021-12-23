#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
pub struct GameService {}
impl GameService {
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceUri() -> ::windows::core::Result<super::super::super::super::super::Foundation::Uri> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::Uri>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetGamerProfileAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInstalledGameItemsAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPartnerTokenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::super::Foundation::Uri>>(audienceuri: Param0) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), audienceuri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPrivilegesAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
    pub fn GrantAchievement(achievementid: u32) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), achievementid).ok() })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
    pub fn GrantAvatarAward(avatarawardid: u32) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), avatarawardid).ok() })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PostResult<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::super::super::Storage::Streams::IBuffer>>(gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: Param4) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), gamevariant, scorekind, scorevalue, gameoutcome, buffer.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyPartnerTokenExpired<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::super::Foundation::Uri>>(audienceuri: Param0) -> ::windows::core::Result<()> {
        Self::IGameService2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audienceuri.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
    pub fn GetAuthenticationStatus() -> ::windows::core::Result<u32> {
        Self::IGameService2(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameService<R, F: FnOnce(&IGameService) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameService, IGameService> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IGameService2<R, F: FnOnce(&IGameService2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameService, IGameService2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
#[repr(transparent)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: Self = Self(0i32);
    pub const Win: Self = Self(1i32);
    pub const Loss: Self = Self(2i32);
    pub const Tie: Self = Self(3i32);
}
impl ::core::marker::Copy for GameServiceGameOutcome {}
impl ::core::clone::Clone for GameServiceGameOutcome {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GameServiceGameOutcome {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GameServiceGameOutcome {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameServiceGameOutcome {}
impl ::core::fmt::Debug for GameServiceGameOutcome {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceGameOutcome").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
}
impl ::windows::core::DefaultType for GameServiceGameOutcome {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
#[repr(transparent)]
pub struct GameServicePropertyCollection(::windows::core::IUnknown);
impl GameServicePropertyCollection {
    #[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameServicePropertyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameServicePropertyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameServicePropertyCollection {}
impl ::core::fmt::Debug for GameServicePropertyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServicePropertyCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection;{07e57fc8-debb-4609-9cc8-529d16bc2bd9})");
}
unsafe impl ::windows::core::Interface for GameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
impl ::windows::core::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
impl ::core::convert::From<GameServicePropertyCollection> for ::windows::core::IUnknown {
    fn from(value: GameServicePropertyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameServicePropertyCollection> for ::windows::core::IUnknown {
    fn from(value: &GameServicePropertyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameServicePropertyCollection> for ::windows::core::IInspectable {
    fn from(value: GameServicePropertyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameServicePropertyCollection> for ::windows::core::IInspectable {
    fn from(value: &GameServicePropertyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameServicePropertyCollection {}
unsafe impl ::core::marker::Sync for GameServicePropertyCollection {}
#[doc = "*Required features: 'Phone_System_UserProfile_GameServices_Core'*"]
#[repr(transparent)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: Self = Self(0i32);
    pub const Time: Self = Self(1i32);
}
impl ::core::marker::Copy for GameServiceScoreKind {}
impl ::core::clone::Clone for GameServiceScoreKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GameServiceScoreKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GameServiceScoreKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameServiceScoreKind {}
impl ::core::fmt::Debug for GameServiceScoreKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceScoreKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
}
impl ::windows::core::DefaultType for GameServiceScoreKind {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameService {
    type Vtable = IGameServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2d5098_48a9_4efc_afd6_8e6da09003fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, achievementid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, avatarawardid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameService2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameService2 {
    type Vtable = IGameService2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2364ef6_ea17_4be5_8d8a_c860885e051f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameServicePropertyCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServicePropertyCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
