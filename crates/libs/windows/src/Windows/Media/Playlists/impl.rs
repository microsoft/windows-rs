#[cfg(feature = "implement_exclusive")]
pub trait IPlaylistImpl: Sized {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFile>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsAsync(&self, savelocation: &::core::option::Option<super::super::Storage::IStorageFolder>, desiredname: &::windows::core::HSTRING, option: super::super::Storage::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn SaveAsWithFormatAsync(&self, savelocation: &::core::option::Option<super::super::Storage::IStorageFolder>, desiredname: &::windows::core::HSTRING, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaylist {
    const NAME: &'static str = "Windows.Media.Playlists.IPlaylist";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaylistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaylistImpl, const OFFSET: isize>() -> IPlaylistVtbl {
        unsafe extern "system" fn Files<Impl: IPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsAsync<Impl: IPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, savelocation: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: super::super::Storage::NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsAsync(&*(&savelocation as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsWithFormatAsync<Impl: IPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, savelocation: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsWithFormatAsync(&*(&savelocation as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option, playlistformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaylist>, ::windows::core::GetTrustLevel, Files::<Impl, OFFSET>, SaveAsync::<Impl, OFFSET>, SaveAsAsync::<Impl, OFFSET>, SaveAsWithFormatAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaylistStaticsImpl: Sized {
    fn LoadAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Playlist>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaylistStatics {
    const NAME: &'static str = "Windows.Media.Playlists.IPlaylistStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaylistStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaylistStaticsImpl, const OFFSET: isize>() -> IPlaylistStaticsVtbl {
        unsafe extern "system" fn LoadAsync<Impl: IPlaylistStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaylistStatics>, ::windows::core::GetTrustLevel, LoadAsync::<Impl, OFFSET>)
    }
}
