#[doc(hidden)]
#[repr(transparent)]
pub struct IGameService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameService {
    type Vtable = IGameService_Vtbl;
}
impl ::core::clone::Clone for IGameService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e2d5098_48a9_4efc_afd6_8e6da09003fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServiceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceUri: usize,
    #[cfg(feature = "Foundation")]
    pub GetGamerProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGamerProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInstalledGameItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInstalledGameItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPartnerTokenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPartnerTokenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPrivilegesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPrivilegesAsync: usize,
    pub GrantAchievement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, achievementid: u32) -> ::windows::core::HRESULT,
    pub GrantAvatarAward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, avatarawardid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PostResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PostResult: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameService2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameService2 {
    type Vtable = IGameService2_Vtbl;
}
impl ::core::clone::Clone for IGameService2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameService2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2364ef6_ea17_4be5_8d8a_c860885e051f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub NotifyPartnerTokenExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyPartnerTokenExpired: usize,
    pub GetAuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameServicePropertyCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_Vtbl;
}
impl ::core::clone::Clone for IGameServicePropertyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameServicePropertyCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
}
#[doc = "*Required features: `\"Phone_System_UserProfile_GameServices_Core\"`*"]
pub struct GameService;
impl GameService {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceUri() -> ::windows::core::Result<super::super::super::super::super::Foundation::Uri> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ServiceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetGamerProfileAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>();
            (::windows::core::Interface::vtable(this).GetGamerProfileAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInstalledGameItemsAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>();
            (::windows::core::Interface::vtable(this).GetInstalledGameItemsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPartnerTokenAsync(audienceuri: &super::super::super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetPartnerTokenAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(audienceuri), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPrivilegesAsync() -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetPrivilegesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GrantAchievement(achievementid: u32) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).GrantAchievement)(::windows::core::Interface::as_raw(this), achievementid).ok() })
    }
    pub fn GrantAvatarAward(avatarawardid: u32) -> ::windows::core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).GrantAvatarAward)(::windows::core::Interface::as_raw(this), avatarawardid).ok() })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PostResult<P0>(gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::super::super::Storage::Streams::IBuffer>,
    {
        Self::IGameService(|this| unsafe { (::windows::core::Interface::vtable(this).PostResult)(::windows::core::Interface::as_raw(this), gamevariant, scorekind, scorevalue, gameoutcome, buffer.try_into_param()?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyPartnerTokenExpired(audienceuri: &super::super::super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        Self::IGameService2(|this| unsafe { (::windows::core::Interface::vtable(this).NotifyPartnerTokenExpired)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(audienceuri)).ok() })
    }
    pub fn GetAuthenticationStatus() -> ::windows::core::Result<u32> {
        Self::IGameService2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetAuthenticationStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameService<R, F: FnOnce(&IGameService) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GameService, IGameService> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGameService2<R, F: FnOnce(&IGameService2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GameService, IGameService2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[doc = "*Required features: `\"Phone_System_UserProfile_GameServices_Core\"`*"]
#[repr(transparent)]
pub struct GameServicePropertyCollection(::windows::core::IUnknown);
impl GameServicePropertyCollection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).GetPropertyAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection;{07e57fc8-debb-4609-9cc8-529d16bc2bd9})");
}
impl ::core::clone::Clone for GameServicePropertyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GameServicePropertyCollection {
    const IID: ::windows::core::GUID = <IGameServicePropertyCollection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
::windows::imp::interface_hierarchy!(GameServicePropertyCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameServicePropertyCollection {}
unsafe impl ::core::marker::Sync for GameServicePropertyCollection {}
#[doc = "*Required features: `\"Phone_System_UserProfile_GameServices_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for GameServiceGameOutcome {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GameServiceGameOutcome {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GameServiceGameOutcome {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceGameOutcome").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
}
#[doc = "*Required features: `\"Phone_System_UserProfile_GameServices_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for GameServiceScoreKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GameServiceScoreKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GameServiceScoreKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceScoreKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
