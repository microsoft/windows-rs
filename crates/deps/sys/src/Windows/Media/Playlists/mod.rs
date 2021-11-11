#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPlaylist();
    fn IPlaylistStatics();
    fn Playlist();
    fn PlaylistFormat();
    fn PlaylistsContract();
}
