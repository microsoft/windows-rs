#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlaylist(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaylistStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Playlist(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaylistFormat(pub i32);
impl PlaylistFormat {
    pub const WindowsMedia: Self = Self(0i32);
    pub const Zune: Self = Self(1i32);
    pub const M3u: Self = Self(2i32);
}
