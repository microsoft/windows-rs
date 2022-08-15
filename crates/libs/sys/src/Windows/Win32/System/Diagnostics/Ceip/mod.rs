#[cfg_attr(windows, link(name = "windows"))]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL;
}
