#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaylist(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaylist {
    type Vtable = IPlaylist_Vtbl;
}
impl ::core::clone::Clone for IPlaylist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaylist {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x803736f5_cf44_4d97_83b3_7a089e9ab663);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaylist_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savelocation: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: super::super::Storage::NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savelocation: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsWithFormatAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaylistStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaylistStatics {
    type Vtable = IPlaylistStatics_Vtbl;
}
impl ::core::clone::Clone for IPlaylistStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaylistStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5c331cd_81f9_4ff3_95b9_70b6ff046b68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaylistStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadAsync: usize,
}
#[doc = "*Required features: `\"Media_Playlists\"`*"]
#[repr(transparent)]
pub struct Playlist(::windows_core::IUnknown);
impl Playlist {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Playlist, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveAsAsync<P0>(&self, savelocation: P0, desiredname: &::windows_core::HSTRING, option: super::super::Storage::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsAsync)(::windows_core::Interface::as_raw(this), savelocation.try_into_param()?.abi(), ::core::mem::transmute_copy(desiredname), option, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveAsWithFormatAsync<P0>(&self, savelocation: P0, desiredname: &::windows_core::HSTRING, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsWithFormatAsync)(::windows_core::Interface::as_raw(this), savelocation.try_into_param()?.abi(), ::core::mem::transmute_copy(desiredname), option, playlistformat, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadAsync<P0>(file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Playlist>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IPlaylistStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaylistStatics<R, F: FnOnce(&IPlaylistStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Playlist, IPlaylistStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for Playlist {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Playlists.Playlist;{803736f5-cf44-4d97-83b3-7a089e9ab663})");
}
impl ::core::clone::Clone for Playlist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Playlist {
    type Vtable = IPlaylist_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Playlist {
    const IID: ::windows_core::GUID = <IPlaylist as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Playlist {
    const NAME: &'static str = "Windows.Media.Playlists.Playlist";
}
::windows_core::imp::interface_hierarchy!(Playlist, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for PlaylistFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlaylistFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaylistFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlaylistFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playlists.PlaylistFormat;i4)");
}
