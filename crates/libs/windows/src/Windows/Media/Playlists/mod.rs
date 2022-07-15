#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaylist(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaylist {
    type Vtable = IPlaylist_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x803736f5_cf44_4d97_83b3_7a089e9ab663);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaylist_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savelocation: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: super::super::Storage::NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savelocation: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsWithFormatAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaylistStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaylistStatics {
    type Vtable = IPlaylistStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5c331cd_81f9_4ff3_95b9_70b6ff046b68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaylistStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadAsync: usize,
}
#[doc = "*Required features: `\"Media_Playlists\"`*"]
#[repr(transparent)]
pub struct Playlist(::windows::core::IUnknown);
impl Playlist {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Playlist, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveAsAsync<'a, P0, E0>(&self, savelocation: P0, desiredname: &::windows::core::HSTRING, option: super::super::Storage::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsAsync)(::windows::core::Interface::as_raw(this), savelocation.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(desiredname), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveAsWithFormatAsync<'a, P0, E0>(&self, savelocation: P0, desiredname: &::windows::core::HSTRING, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsWithFormatAsync)(::windows::core::Interface::as_raw(this), savelocation.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(desiredname), option, playlistformat, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadAsync<'a, P0, E0>(file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Playlist>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPlaylistStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadAsync)(::windows::core::Interface::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Playlist>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaylistStatics<R, F: FnOnce(&IPlaylistStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Playlist, IPlaylistStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Playlist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Playlist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Playlist {}
impl ::core::fmt::Debug for Playlist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Playlist").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Playlist {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playlists.Playlist;{803736f5-cf44-4d97-83b3-7a089e9ab663})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Playlist {
    type Vtable = IPlaylist_Vtbl;
    const IID: ::windows::core::GUID = <IPlaylist as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Playlist {
    const NAME: &'static str = "Windows.Media.Playlists.Playlist";
}
impl ::core::convert::From<Playlist> for ::windows::core::IUnknown {
    fn from(value: Playlist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Playlist> for ::windows::core::IUnknown {
    fn from(value: &Playlist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Playlist> for &::windows::core::IUnknown {
    fn from(value: &Playlist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Playlist> for ::windows::core::IInspectable {
    fn from(value: Playlist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Playlist> for ::windows::core::IInspectable {
    fn from(value: &Playlist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Playlist> for &::windows::core::IInspectable {
    fn from(value: &Playlist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Playlists\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlaylistFormat(pub i32);
impl PlaylistFormat {
    pub const WindowsMedia: Self = Self(0i32);
    pub const Zune: Self = Self(1i32);
    pub const M3u: Self = Self(2i32);
}
impl ::core::marker::Copy for PlaylistFormat {}
impl ::core::clone::Clone for PlaylistFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlaylistFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlaylistFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaylistFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaylistFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaylistFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playlists.PlaylistFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
