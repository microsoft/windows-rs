#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
pub struct GameList {}
impl GameList {
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsyncPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GameAdded<'a, Param0: ::windows::core::IntoParam<'a, GameListChangedEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GameRemoved<'a, Param0: ::windows::core::IntoParam<'a, GameListRemovedEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GameUpdated<'a, Param0: ::windows::core::IntoParam<'a, GameListChangedEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MergeEntriesAsync<'a, Param0: ::windows::core::IntoParam<'a, GameListEntry>, Param1: ::windows::core::IntoParam<'a, GameListEntry>>(left: Param0, right: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameListEntry>> {
        Self::IGameListStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), left.into_param().abi(), right.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameListEntry>>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn UnmergeEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, GameListEntry>>(mergedentry: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mergedentry.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameListStatics<R, F: FnOnce(&IGameListStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameList, IGameListStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IGameListStatics2<R, F: FnOnce(&IGameListStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameList, IGameListStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameList {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameList";
}
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameListCategory(pub i32);
impl GameListCategory {
    pub const Candidate: Self = Self(0i32);
    pub const ConfirmedBySystem: Self = Self(1i32);
    pub const ConfirmedByUser: Self = Self(2i32);
}
impl ::core::marker::Copy for GameListCategory {}
impl ::core::clone::Clone for GameListCategory {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GameListCategory {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GameListCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListCategory {}
impl ::core::fmt::Debug for GameListCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameListCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Preview.GamesEnumeration.GameListCategory;i4)");
}
impl ::windows::core::DefaultType for GameListCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameListChangedEventHandler(pub ::windows::core::IUnknown);
impl GameListChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<GameListEntry>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = GameListChangedEventHandlerBox::<F> { vtable: &GameListChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, GameListEntry>>(&self, game: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), game.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct GameListChangedEventHandlerBox<F: FnMut(&::core::option::Option<GameListEntry>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const GameListChangedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<GameListEntry>) -> ::windows::core::Result<()> + 'static> GameListChangedEventHandlerBox<F> {
    const VTABLE: GameListChangedEventHandlerVtbl = GameListChangedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<GameListChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, game: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&game as *const <GameListEntry as ::windows::core::Abi>::Abi as *const <GameListEntry as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for GameListChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameListChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListChangedEventHandler {}
impl ::core::fmt::Debug for GameListChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for GameListChangedEventHandler {
    type Vtable = GameListChangedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25f6a421_d8f5_4d91_b40e_53d5e86fde64);
}
unsafe impl ::windows::core::RuntimeType for GameListChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{25f6a421-d8f5-4d91-b40e-53d5e86fde64}");
}
#[repr(C)]
#[doc(hidden)]
pub struct GameListChangedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, game: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameListEntry(::windows::core::IUnknown);
impl GameListEntry {
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'ApplicationModel'*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::AppDisplayInfo>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn Category(&self) -> ::windows::core::Result<GameListCategory> {
        let this = self;
        unsafe {
            let mut result__: GameListCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameListCategory>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCategoryAsync(&self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn LaunchableState(&self) -> ::windows::core::Result<GameListEntryLaunchableState> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: GameListEntryLaunchableState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameListEntryLaunchableState>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn LauncherExecutable(&self) -> ::windows::core::Result<super::super::super::Storage::IStorageFile> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn LaunchParameters(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SetLauncherExecutableFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, executablefile: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), executablefile.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SetLauncherExecutableFileWithParamsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, executablefile: Param0, launchparams: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), executablefile.into_param().abi(), launchparams.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTitleIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn GameModeConfiguration(&self) -> ::windows::core::Result<GameModeConfiguration> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameModeConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for GameListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListEntry {}
impl ::core::fmt::Debug for GameListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Preview.GamesEnumeration.GameListEntry;{735924d3-811f-4494-b69c-c641a0c61543})");
}
unsafe impl ::windows::core::Interface for GameListEntry {
    type Vtable = IGameListEntryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735924d3_811f_4494_b69c_c641a0c61543);
}
impl ::windows::core::RuntimeName for GameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameListEntry";
}
impl ::core::convert::From<GameListEntry> for ::windows::core::IUnknown {
    fn from(value: GameListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameListEntry> for ::windows::core::IUnknown {
    fn from(value: &GameListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameListEntry> for ::windows::core::IInspectable {
    fn from(value: GameListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameListEntry> for ::windows::core::IInspectable {
    fn from(value: &GameListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<GameListEntry> for IGameListEntry {
    type Error = ::windows::core::Error;
    fn try_from(value: GameListEntry) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GameListEntry> for IGameListEntry {
    type Error = ::windows::core::Error;
    fn try_from(value: &GameListEntry) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameListEntry> for GameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, IGameListEntry> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameListEntry> for &GameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, IGameListEntry> {
        ::core::convert::TryInto::<IGameListEntry>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GameListEntry {}
unsafe impl ::core::marker::Sync for GameListEntry {}
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameListEntryLaunchableState(pub i32);
impl GameListEntryLaunchableState {
    pub const NotLaunchable: Self = Self(0i32);
    pub const ByLastRunningFullPath: Self = Self(1i32);
    pub const ByUserProvidedPath: Self = Self(2i32);
    pub const ByTile: Self = Self(3i32);
}
impl ::core::marker::Copy for GameListEntryLaunchableState {}
impl ::core::clone::Clone for GameListEntryLaunchableState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GameListEntryLaunchableState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GameListEntryLaunchableState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListEntryLaunchableState {}
impl ::core::fmt::Debug for GameListEntryLaunchableState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListEntryLaunchableState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameListEntryLaunchableState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Preview.GamesEnumeration.GameListEntryLaunchableState;i4)");
}
impl ::windows::core::DefaultType for GameListEntryLaunchableState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameListRemovedEventHandler(pub ::windows::core::IUnknown);
impl GameListRemovedEventHandler {
    pub fn new<F: FnMut(&::windows::core::HSTRING) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = GameListRemovedEventHandlerBox::<F> { vtable: &GameListRemovedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, identifier: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), identifier.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct GameListRemovedEventHandlerBox<F: FnMut(&::windows::core::HSTRING) -> ::windows::core::Result<()> + 'static> {
    vtable: *const GameListRemovedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::windows::core::HSTRING) -> ::windows::core::Result<()> + 'static> GameListRemovedEventHandlerBox<F> {
    const VTABLE: GameListRemovedEventHandlerVtbl = GameListRemovedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<GameListRemovedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&identifier as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for GameListRemovedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameListRemovedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListRemovedEventHandler {}
impl ::core::fmt::Debug for GameListRemovedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListRemovedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for GameListRemovedEventHandler {
    type Vtable = GameListRemovedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c5648f_6c8f_4712_9b38_474bc22e76d8);
}
unsafe impl ::windows::core::RuntimeType for GameListRemovedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{10c5648f-6c8f-4712-9b38-474bc22e76d8}");
}
#[repr(C)]
#[doc(hidden)]
pub struct GameListRemovedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameModeConfiguration(::windows::core::IUnknown);
impl GameModeConfiguration {
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PercentGpuTimeAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPercentGpuTimeAllocatedToGame<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PercentGpuMemoryAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPercentGpuMemoryAllocatedToGame<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PercentGpuMemoryAllocatedToSystemCompositor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPercentGpuMemoryAllocatedToSystemCompositor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxCpuCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxCpuCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CpuExclusivityMaskLow(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCpuExclusivityMaskLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CpuExclusivityMaskHigh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCpuExclusivityMaskHigh<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn AffinitizeToExclusiveCpus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn SetAffinitizeToExclusiveCpus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for GameModeConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameModeConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameModeConfiguration {}
impl ::core::fmt::Debug for GameModeConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameModeConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameModeConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration;{78e591af-b142-4ef0-8830-55bc2be4f5ea})");
}
unsafe impl ::windows::core::Interface for GameModeConfiguration {
    type Vtable = IGameModeConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78e591af_b142_4ef0_8830_55bc2be4f5ea);
}
impl ::windows::core::RuntimeName for GameModeConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration";
}
impl ::core::convert::From<GameModeConfiguration> for ::windows::core::IUnknown {
    fn from(value: GameModeConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameModeConfiguration> for ::windows::core::IUnknown {
    fn from(value: &GameModeConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameModeConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameModeConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameModeConfiguration> for ::windows::core::IInspectable {
    fn from(value: GameModeConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameModeConfiguration> for ::windows::core::IInspectable {
    fn from(value: &GameModeConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameModeConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameModeConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameModeConfiguration {}
unsafe impl ::core::marker::Sync for GameModeConfiguration {}
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct GameModeUserConfiguration(::windows::core::IUnknown);
impl GameModeUserConfiguration {
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GamingRelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn GetDefault() -> ::windows::core::Result<GameModeUserConfiguration> {
        Self::IGameModeUserConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameModeUserConfiguration>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameModeUserConfigurationStatics<R, F: FnOnce(&IGameModeUserConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameModeUserConfiguration, IGameModeUserConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GameModeUserConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameModeUserConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameModeUserConfiguration {}
impl ::core::fmt::Debug for GameModeUserConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameModeUserConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameModeUserConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration;{72d34af4-756b-470f-a0c2-ba62a90795db})");
}
unsafe impl ::windows::core::Interface for GameModeUserConfiguration {
    type Vtable = IGameModeUserConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d34af4_756b_470f_a0c2_ba62a90795db);
}
impl ::windows::core::RuntimeName for GameModeUserConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration";
}
impl ::core::convert::From<GameModeUserConfiguration> for ::windows::core::IUnknown {
    fn from(value: GameModeUserConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameModeUserConfiguration> for ::windows::core::IUnknown {
    fn from(value: &GameModeUserConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameModeUserConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameModeUserConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameModeUserConfiguration> for ::windows::core::IInspectable {
    fn from(value: GameModeUserConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameModeUserConfiguration> for ::windows::core::IInspectable {
    fn from(value: &GameModeUserConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameModeUserConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameModeUserConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameModeUserConfiguration {}
unsafe impl ::core::marker::Sync for GameModeUserConfiguration {}
#[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
#[repr(transparent)]
pub struct IGameListEntry(::windows::core::IUnknown);
impl IGameListEntry {
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'ApplicationModel'*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::AppDisplayInfo>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration'*"]
    pub fn Category(&self) -> ::windows::core::Result<GameListCategory> {
        let this = self;
        unsafe {
            let mut result__: GameListCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameListCategory>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_Preview_GamesEnumeration', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCategoryAsync(&self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::convert::From<IGameListEntry> for ::windows::core::IInspectable {
    fn from(value: IGameListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameListEntry> for ::windows::core::IInspectable {
    fn from(value: &IGameListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IGameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGameListEntry> for ::windows::core::IUnknown {
    fn from(value: IGameListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameListEntry> for ::windows::core::IUnknown {
    fn from(value: &IGameListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IGameListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameListEntry {}
impl ::core::fmt::Debug for IGameListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGameListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{735924d3-811f-4494-b69c-c641a0c61543}");
}
unsafe impl ::windows::core::Interface for IGameListEntry {
    type Vtable = IGameListEntryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735924d3_811f_4494_b69c_c641a0c61543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListEntryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameListCategory) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GameListCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameListEntry2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameListEntry2 {
    type Vtable = IGameListEntry2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84a8f8b_8749_4a25_90d3_f6c5a427886d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListEntry2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameListEntryLaunchableState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablefile: ::windows::core::RawPtr, launchparams: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameListStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameListStatics {
    type Vtable = IGameListStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ddd0f6f_9c66_4b05_945c_d6ed78491b8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameListStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameListStatics2 {
    type Vtable = IGameListStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x395f2098_ea1a_45aa_9268_a83905686f27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: ::windows::core::RawPtr, right: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mergedentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameModeConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameModeConfiguration {
    type Vtable = IGameModeConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78e591af_b142_4ef0_8830_55bc2be4f5ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameModeUserConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameModeUserConfiguration {
    type Vtable = IGameModeUserConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d34af4_756b_470f_a0c2_ba62a90795db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeUserConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameModeUserConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameModeUserConfigurationStatics {
    type Vtable = IGameModeUserConfigurationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e50d97c_66ea_478e_a4a1_f57c0e8d00e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeUserConfigurationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
