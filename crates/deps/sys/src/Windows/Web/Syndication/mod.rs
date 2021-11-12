#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISyndicationAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationAttributeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationCategoryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationClientFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationErrorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationFeed(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationFeedFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationGeneratorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationLinkFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationNodeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationPerson(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationPersonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyndicationTextFactory(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RetrievalProgress(i32);
#[repr(transparent)]
pub struct SyndicationAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationErrorStatus(pub i32);
impl SyndicationErrorStatus {
    pub const Unknown: SyndicationErrorStatus = SyndicationErrorStatus(0i32);
    pub const MissingRequiredElement: SyndicationErrorStatus = SyndicationErrorStatus(1i32);
    pub const MissingRequiredAttribute: SyndicationErrorStatus = SyndicationErrorStatus(2i32);
    pub const InvalidXml: SyndicationErrorStatus = SyndicationErrorStatus(3i32);
    pub const UnexpectedContent: SyndicationErrorStatus = SyndicationErrorStatus(4i32);
    pub const UnsupportedFormat: SyndicationErrorStatus = SyndicationErrorStatus(5i32);
}
#[repr(transparent)]
pub struct SyndicationFeed(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationFormat(pub i32);
impl SyndicationFormat {
    pub const Atom10: SyndicationFormat = SyndicationFormat(0i32);
    pub const Rss20: SyndicationFormat = SyndicationFormat(1i32);
    pub const Rss10: SyndicationFormat = SyndicationFormat(2i32);
    pub const Rss092: SyndicationFormat = SyndicationFormat(3i32);
    pub const Rss091: SyndicationFormat = SyndicationFormat(4i32);
    pub const Atom03: SyndicationFormat = SyndicationFormat(5i32);
}
#[repr(transparent)]
pub struct SyndicationGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationPerson(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationTextType(pub i32);
impl SyndicationTextType {
    pub const Text: SyndicationTextType = SyndicationTextType(0i32);
    pub const Html: SyndicationTextType = SyndicationTextType(1i32);
    pub const Xhtml: SyndicationTextType = SyndicationTextType(2i32);
}
#[repr(C)]
pub struct TransferProgress(i32);
