#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPlaylist(pub *mut ::core::ffi::c_void);
pub struct IPlaylistStatics(pub *mut ::core::ffi::c_void);
pub struct Playlist(i32);
pub struct PlaylistFormat(i32);
pub struct PlaylistsContract(i32);
