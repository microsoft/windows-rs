#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
pub struct GameService {}
impl GameService {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn ServiceUri() -> ::windows::runtime::Result<super::super::super::super::super::Foundation::Uri> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetGamerProfileAsync() -> ::windows::runtime::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetInstalledGameItemsAsync() -> ::windows::runtime::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetPartnerTokenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::super::Foundation::Uri>>(audienceuri: Param0) -> ::windows::runtime::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), audienceuri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetPrivilegesAsync() -> ::windows::runtime::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
    pub fn GrantAchievement(achievementid: u32) -> ::windows::runtime::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), achievementid).ok() })
    }
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
    pub fn GrantAvatarAward(avatarawardid: u32) -> ::windows::runtime::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), avatarawardid).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Storage_Streams`*"]
    pub fn PostResult<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::super::super::Storage::Streams::IBuffer>>(gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: Param4) -> ::windows::runtime::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), gamevariant, scorekind, scorevalue, gameoutcome, buffer.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn NotifyPartnerTokenExpired<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::super::Foundation::Uri>>(audienceuri: Param0) -> ::windows::runtime::Result<()> {
        Self::IGameService2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), audienceuri.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
    pub fn GetAuthenticationStatus() -> ::windows::runtime::Result<u32> {
        Self::IGameService2(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IGameService<R, F: FnOnce(&IGameService) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameService, IGameService> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGameService2<R, F: FnOnce(&IGameService2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameService, IGameService2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: GameServiceGameOutcome = GameServiceGameOutcome(0i32);
    pub const Win: GameServiceGameOutcome = GameServiceGameOutcome(1i32);
    pub const Loss: GameServiceGameOutcome = GameServiceGameOutcome(2i32);
    pub const Tie: GameServiceGameOutcome = GameServiceGameOutcome(3i32);
}
impl ::std::convert::From<i32> for GameServiceGameOutcome {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameServiceGameOutcome {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
}
impl ::windows::runtime::DefaultType for GameServiceGameOutcome {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameServicePropertyCollection(::windows::runtime::IInspectable);
impl GameServicePropertyCollection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`, `Foundation`*"]
    pub fn GetPropertyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection;{07e57fc8-debb-4609-9cc8-529d16bc2bd9})");
}
unsafe impl ::windows::runtime::Interface for GameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(132480968, 57019, 17929, [156, 200, 82, 157, 22, 188, 43, 217]);
}
impl ::windows::runtime::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
unsafe impl ::std::marker::Send for GameServicePropertyCollection {}
unsafe impl ::std::marker::Sync for GameServicePropertyCollection {}
#[doc = "*Required features: `Phone_System_UserProfile_GameServices_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: GameServiceScoreKind = GameServiceScoreKind(0i32);
    pub const Time: GameServiceScoreKind = GameServiceScoreKind(1i32);
}
impl ::std::convert::From<i32> for GameServiceScoreKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameServiceScoreKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
}
impl ::windows::runtime::DefaultType for GameServiceScoreKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameService(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameService {
    type Vtable = IGameService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(774721688, 18601, 20220, [175, 214, 142, 109, 160, 144, 3, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audienceuri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, achievementid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, avatarawardid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameService2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameService2 {
    type Vtable = IGameService2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3526774518, 59927, 19429, [141, 138, 200, 96, 136, 94, 5, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audienceuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(132480968, 57019, 17929, [156, 200, 82, 157, 22, 188, 43, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
