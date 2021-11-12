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
    pub const WindowsMedia: PlaylistFormat = PlaylistFormat(0i32);
    pub const Zune: PlaylistFormat = PlaylistFormat(1i32);
    pub const M3u: PlaylistFormat = PlaylistFormat(2i32);
}
#[repr(C)]
pub struct PlaylistsContract(i32);
