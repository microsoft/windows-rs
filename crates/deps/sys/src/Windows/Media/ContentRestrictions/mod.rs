#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContentAccessRestrictionLevel(pub i32);
impl ContentAccessRestrictionLevel {
    pub const Allow: ContentAccessRestrictionLevel = ContentAccessRestrictionLevel(0i32);
    pub const Warn: ContentAccessRestrictionLevel = ContentAccessRestrictionLevel(1i32);
    pub const Block: ContentAccessRestrictionLevel = ContentAccessRestrictionLevel(2i32);
    pub const Hide: ContentAccessRestrictionLevel = ContentAccessRestrictionLevel(3i32);
}
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentRestrictions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatedContentCategory(pub i32);
impl RatedContentCategory {
    pub const General: RatedContentCategory = RatedContentCategory(0i32);
    pub const Application: RatedContentCategory = RatedContentCategory(1i32);
    pub const Game: RatedContentCategory = RatedContentCategory(2i32);
    pub const Movie: RatedContentCategory = RatedContentCategory(3i32);
    pub const Television: RatedContentCategory = RatedContentCategory(4i32);
    pub const Music: RatedContentCategory = RatedContentCategory(5i32);
}
#[repr(transparent)]
pub struct RatedContentDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatedContentRestrictions(pub *mut ::core::ffi::c_void);
