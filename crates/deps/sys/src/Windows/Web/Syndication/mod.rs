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
pub struct RetrievalProgress {
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for RetrievalProgress {}
impl ::core::clone::Clone for RetrievalProgress {
    fn clone(&self) -> Self {
        *self
    }
}
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
    pub const Unknown: Self = Self(0i32);
    pub const MissingRequiredElement: Self = Self(1i32);
    pub const MissingRequiredAttribute: Self = Self(2i32);
    pub const InvalidXml: Self = Self(3i32);
    pub const UnexpectedContent: Self = Self(4i32);
    pub const UnsupportedFormat: Self = Self(5i32);
}
#[repr(transparent)]
pub struct SyndicationFeed(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SyndicationFormat(pub i32);
impl SyndicationFormat {
    pub const Atom10: Self = Self(0i32);
    pub const Rss20: Self = Self(1i32);
    pub const Rss10: Self = Self(2i32);
    pub const Rss092: Self = Self(3i32);
    pub const Rss091: Self = Self(4i32);
    pub const Atom03: Self = Self(5i32);
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
    pub const Text: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
    pub const Xhtml: Self = Self(2i32);
}
#[repr(C)]
pub struct TransferProgress {
    pub BytesSent: u32,
    pub TotalBytesToSend: u32,
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for TransferProgress {}
impl ::core::clone::Clone for TransferProgress {
    fn clone(&self) -> Self {
        *self
    }
}
