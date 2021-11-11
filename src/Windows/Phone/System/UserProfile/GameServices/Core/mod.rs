#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
pub struct GameService {}
impl GameService {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn ServiceUri() -> ::windows::core::Result<super::super::super::super::super::Foundation::Uri> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetGamerProfileAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetInstalledGameItemsAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetPartnerTokenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::super::Foundation::Uri>>(audienceuri: Param0) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), audienceuri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetPrivilegesAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
    pub fn GrantAchievement(achievementid: u32) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), achievementid).ok() })
    }
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
    pub fn GrantAvatarAward(avatarawardid: u32) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), avatarawardid).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Storage_Streams`*"]
    pub fn PostResult<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::super::super::Storage::Streams::IBuffer>>(gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: Param4) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), gamevariant, scorekind, scorevalue, gameoutcome, buffer.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn NotifyPartnerTokenExpired<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::super::Foundation::Uri>>(audienceuri: Param0) -> ::windows::core::Result<()> {
        Self::IGameService2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audienceuri.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
    pub fn GetAuthenticationStatus() -> ::windows::core::Result<u32> {
        Self::IGameService2(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IGameService<R, F: FnOnce(&IGameService) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameService, IGameService> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGameService2<R, F: FnOnce(&IGameService2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameService, IGameService2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: GameServiceGameOutcome = GameServiceGameOutcome(0i32);
    pub const Win: GameServiceGameOutcome = GameServiceGameOutcome(1i32);
    pub const Loss: GameServiceGameOutcome = GameServiceGameOutcome(2i32);
    pub const Tie: GameServiceGameOutcome = GameServiceGameOutcome(3i32);
}
impl ::core::convert::From<i32> for GameServiceGameOutcome {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameServiceGameOutcome {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
}
impl ::windows::core::DefaultType for GameServiceGameOutcome {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameServicePropertyCollection(pub ::windows::core::IInspectable);
impl GameServicePropertyCollection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetPropertyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection;{07e57fc8-debb-4609-9cc8-529d16bc2bd9})");
}
unsafe impl ::windows::core::Interface for GameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
impl ::windows::core::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
impl ::core::convert::From<GameServicePropertyCollection> for ::windows::core::IUnknown {
    fn from(value: GameServicePropertyCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameServicePropertyCollection> for ::windows::core::IUnknown {
    fn from(value: &GameServicePropertyCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameServicePropertyCollection> for ::windows::core::IInspectable {
    fn from(value: GameServicePropertyCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameServicePropertyCollection> for ::windows::core::IInspectable {
    fn from(value: &GameServicePropertyCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameServicePropertyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameServicePropertyCollection {}
unsafe impl ::core::marker::Sync for GameServicePropertyCollection {}
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: GameServiceScoreKind = GameServiceScoreKind(0i32);
    pub const Time: GameServiceScoreKind = GameServiceScoreKind(1i32);
}
impl ::core::convert::From<i32> for GameServiceScoreKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameServiceScoreKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
}
impl ::windows::core::DefaultType for GameServiceScoreKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameService {
    type Vtable = IGameService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2d5098_48a9_4efc_afd6_8e6da09003fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audienceuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, achievementid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, avatarawardid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameService2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameService2 {
    type Vtable = IGameService2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2364ef6_ea17_4be5_8d8a_c860885e051f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audienceuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
