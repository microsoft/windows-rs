#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Web_AtomPub")]
pub mod AtomPub;
#[cfg(feature = "Web_Http")]
pub mod Http;
#[cfg(feature = "Web_Syndication")]
pub mod Syndication;
#[cfg(feature = "Web_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUriToStreamResolver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebErrorStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebErrorStatus(i32);
