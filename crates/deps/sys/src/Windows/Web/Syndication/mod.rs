#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISyndicationAttribute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationAttribute {}
impl ::core::clone::Clone for ISyndicationAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationAttributeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationAttributeFactory {}
impl ::core::clone::Clone for ISyndicationAttributeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationCategory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationCategory {}
impl ::core::clone::Clone for ISyndicationCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationCategoryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationCategoryFactory {}
impl ::core::clone::Clone for ISyndicationCategoryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationClient {}
impl ::core::clone::Clone for ISyndicationClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationClientFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationClientFactory {}
impl ::core::clone::Clone for ISyndicationClientFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationContent {}
impl ::core::clone::Clone for ISyndicationContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationContentFactory {}
impl ::core::clone::Clone for ISyndicationContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationErrorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationErrorStatics {}
impl ::core::clone::Clone for ISyndicationErrorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationFeed(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationFeed {}
impl ::core::clone::Clone for ISyndicationFeed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationFeedFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationFeedFactory {}
impl ::core::clone::Clone for ISyndicationFeedFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationGenerator {}
impl ::core::clone::Clone for ISyndicationGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationGeneratorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationGeneratorFactory {}
impl ::core::clone::Clone for ISyndicationGeneratorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationItem {}
impl ::core::clone::Clone for ISyndicationItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationItemFactory {}
impl ::core::clone::Clone for ISyndicationItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationLink {}
impl ::core::clone::Clone for ISyndicationLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationLinkFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationLinkFactory {}
impl ::core::clone::Clone for ISyndicationLinkFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationNode {}
impl ::core::clone::Clone for ISyndicationNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationNodeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationNodeFactory {}
impl ::core::clone::Clone for ISyndicationNodeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationPerson(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationPerson {}
impl ::core::clone::Clone for ISyndicationPerson {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationPersonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationPersonFactory {}
impl ::core::clone::Clone for ISyndicationPersonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationText {}
impl ::core::clone::Clone for ISyndicationText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISyndicationTextFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISyndicationTextFactory {}
impl ::core::clone::Clone for ISyndicationTextFactory {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SyndicationAttribute {}
impl ::core::clone::Clone for SyndicationAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationCategory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationCategory {}
impl ::core::clone::Clone for SyndicationCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationClient {}
impl ::core::clone::Clone for SyndicationClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationContent {}
impl ::core::clone::Clone for SyndicationContent {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SyndicationErrorStatus {}
impl ::core::clone::Clone for SyndicationErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationFeed(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationFeed {}
impl ::core::clone::Clone for SyndicationFeed {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SyndicationFormat {}
impl ::core::clone::Clone for SyndicationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationGenerator {}
impl ::core::clone::Clone for SyndicationGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationItem {}
impl ::core::clone::Clone for SyndicationItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationLink {}
impl ::core::clone::Clone for SyndicationLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationNode {}
impl ::core::clone::Clone for SyndicationNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationPerson(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationPerson {}
impl ::core::clone::Clone for SyndicationPerson {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SyndicationText {}
impl ::core::clone::Clone for SyndicationText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SyndicationTextType(pub i32);
impl SyndicationTextType {
    pub const Text: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
    pub const Xhtml: Self = Self(2i32);
}
impl ::core::marker::Copy for SyndicationTextType {}
impl ::core::clone::Clone for SyndicationTextType {
    fn clone(&self) -> Self {
        *self
    }
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
