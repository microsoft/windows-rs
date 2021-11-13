#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlaylist(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaylist {}
impl ::core::clone::Clone for IPlaylist {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaylistStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaylistStatics {}
impl ::core::clone::Clone for IPlaylistStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Playlist(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Playlist {}
impl ::core::clone::Clone for Playlist {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
