#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Storage_BulkAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FileInformation(::windows::runtime::IInspectable);
impl FileInformation {
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn MusicProperties(&self) -> ::windows::runtime::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn VideoProperties(&self) -> ::windows::runtime::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn ImageProperties(&self) -> ::windows::runtime::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn DocumentProperties(&self) -> ::windows::runtime::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn BasicProperties(&self) -> ::windows::runtime::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn ThumbnailUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::runtime::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn PropertiesUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::runtime::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn FileType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenAsync(&self, accessmode: super::FileAccessMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CopyOverload<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: super::NameCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn MoveOverload<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: super::NameCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenWithOptionsAsync(&self, accessmode: super::FileAccessMode, options: super::StorageOpenOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), accessmode, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: super::StorageOpenOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn IsAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0, option: super::NameCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<super::FileAttributes> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::FileAttributes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn GetParentAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageItem>>(&self, item: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn DisplayType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn FolderRelativeId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Provider(&self) -> ::windows::runtime::Result<super::StorageProvider> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenReadAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
}
unsafe impl ::windows::runtime::Interface for FileInformation {
    type Vtable = IStorageItemInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275789707, 35186, 20288, [141, 224, 216, 111, 177, 121, 216, 250]);
}
impl ::windows::runtime::RuntimeName for FileInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformation";
}
impl ::std::convert::From<FileInformation> for ::windows::runtime::IUnknown {
    fn from(value: FileInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileInformation> for ::windows::runtime::IUnknown {
    fn from(value: &FileInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FileInformation> for ::windows::runtime::IInspectable {
    fn from(value: FileInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileInformation> for ::windows::runtime::IInspectable {
    fn from(value: &FileInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<FileInformation> for IStorageItemInformation {
    fn from(value: FileInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileInformation> for IStorageItemInformation {
    fn from(value: &FileInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemInformation> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemInformation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStorageItemInformation>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemInformation> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemInformation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStorageItemInformation>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageFile {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageFile {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFile> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFile> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFile> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFile> {
        ::std::convert::TryInto::<super::IStorageFile>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageFile2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageFile2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFile2> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFile2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFile2> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFile2> {
        ::std::convert::TryInto::<super::IStorageFile2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFilePropertiesWithAvailability> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFilePropertiesWithAvailability> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFilePropertiesWithAvailability> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFilePropertiesWithAvailability> {
        ::std::convert::TryInto::<super::IStorageFilePropertiesWithAvailability>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem> {
        ::std::convert::TryInto::<super::IStorageItem>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageItem2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageItem2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem2> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem2> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem2> {
        ::std::convert::TryInto::<super::IStorageItem2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageItemProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageItemProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemProperties> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemProperties> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemProperties> {
        ::std::convert::TryInto::<super::IStorageItemProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::std::convert::TryInto::<super::IStorageItemPropertiesWithProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IInputStreamReference> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IInputStreamReference> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IInputStreamReference> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IInputStreamReference> {
        ::std::convert::TryInto::<super::Streams::IInputStreamReference>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IRandomAccessStreamReference> for FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IRandomAccessStreamReference> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IRandomAccessStreamReference> for &FileInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IRandomAccessStreamReference> {
        ::std::convert::TryInto::<super::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Storage_BulkAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FileInformationFactory(::windows::runtime::IInspectable);
impl FileInformationFactory {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn GetVirtualizedItemsVector(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn GetVirtualizedFilesVector(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn GetVirtualizedFoldersVector(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Search`*"]
    pub fn CreateWithMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode) -> ::windows::runtime::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), queryresult.into_param().abi(), mode, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Search`*"]
    pub fn CreateWithModeAndSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows::runtime::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Search`*"]
    pub fn CreateWithModeAndSizeAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows::runtime::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, thumbnailoptions, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Search`*"]
    pub fn CreateWithModeAndSizeAndOptionsAndFlags<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows::runtime::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, thumbnailoptions, delayload, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    pub fn IFileInformationFactoryFactory<R, F: FnOnce(&IFileInformationFactoryFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileInformationFactory, IFileInformationFactoryFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileInformationFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformationFactory;{401d88be-960f-4d6d-a7d0-1a3861e76c83})");
}
unsafe impl ::windows::runtime::Interface for FileInformationFactory {
    type Vtable = IFileInformationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1075677374, 38415, 19821, [167, 208, 26, 56, 97, 231, 108, 131]);
}
impl ::windows::runtime::RuntimeName for FileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformationFactory";
}
impl ::std::convert::From<FileInformationFactory> for ::windows::runtime::IUnknown {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileInformationFactory> for ::windows::runtime::IUnknown {
    fn from(value: &FileInformationFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileInformationFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileInformationFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FileInformationFactory> for ::windows::runtime::IInspectable {
    fn from(value: FileInformationFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileInformationFactory> for ::windows::runtime::IInspectable {
    fn from(value: &FileInformationFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileInformationFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileInformationFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FileInformationFactory {}
unsafe impl ::std::marker::Sync for FileInformationFactory {}
#[doc = "*Required features: `Storage_BulkAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FolderInformation(::windows::runtime::IInspectable);
impl FolderInformation {
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn MusicProperties(&self) -> ::windows::runtime::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn VideoProperties(&self) -> ::windows::runtime::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn ImageProperties(&self) -> ::windows::runtime::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn DocumentProperties(&self) -> ::windows::runtime::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn BasicProperties(&self) -> ::windows::runtime::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn ThumbnailUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::runtime::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn PropertiesUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::runtime::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CreateFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0, options: super::CreationCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn CreateFolderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0, options: super::CreationCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn GetFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn GetFolderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn GetItemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn TryGetItemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageFolder2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, desiredname: Param0, option: super::NameCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<super::FileAttributes> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::FileAttributes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn GetParentAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageItem>>(&self, item: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn DisplayType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn FolderRelativeId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `Storage_BulkAccess`*"]
    pub fn Provider(&self) -> ::windows::runtime::Result<super::StorageProvider> {
        let this = &::windows::runtime::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Storage_Search`*"]
    pub fn GetIndexedStateAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows::runtime::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateFileQuery(&self, query: super::Search::CommonFileQuery) -> ::windows::runtime::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::runtime::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows::runtime::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateFolderQuery(&self, query: super::Search::CommonFolderQuery) -> ::windows::runtime::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::runtime::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateItemQuery(&self) -> ::windows::runtime::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::runtime::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFilesAsync(&self, query: super::Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFileQuery) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), query, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFoldersAsync(&self, query: super::Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFolderQuery) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), query, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows::runtime::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn IsCommonFolderQuerySupported(&self, query: super::Search::CommonFolderQuery) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_Search`*"]
    pub fn IsCommonFileQuerySupported(&self, query: super::Search::CommonFileQuery) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FolderInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FolderInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
}
unsafe impl ::windows::runtime::Interface for FolderInformation {
    type Vtable = IStorageItemInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275789707, 35186, 20288, [141, 224, 216, 111, 177, 121, 216, 250]);
}
impl ::windows::runtime::RuntimeName for FolderInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FolderInformation";
}
impl ::std::convert::From<FolderInformation> for ::windows::runtime::IUnknown {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FolderInformation> for ::windows::runtime::IUnknown {
    fn from(value: &FolderInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FolderInformation> for ::windows::runtime::IInspectable {
    fn from(value: FolderInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FolderInformation> for ::windows::runtime::IInspectable {
    fn from(value: &FolderInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<FolderInformation> for IStorageItemInformation {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FolderInformation> for IStorageItemInformation {
    fn from(value: &FolderInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemInformation> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemInformation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStorageItemInformation>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemInformation> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemInformation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStorageItemInformation>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<FolderInformation> for super::IStorageFolder {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FolderInformation> for super::IStorageFolder {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFolder> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFolder> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFolder> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFolder> {
        ::std::convert::TryInto::<super::IStorageFolder>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFolder2> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFolder2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageFolder2> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageFolder2> {
        ::std::convert::TryInto::<super::IStorageFolder2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FolderInformation> for super::IStorageItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FolderInformation> for super::IStorageItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem> {
        ::std::convert::TryInto::<super::IStorageItem>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FolderInformation> for super::IStorageItem2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FolderInformation> for super::IStorageItem2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem2> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItem2> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItem2> {
        ::std::convert::TryInto::<super::IStorageItem2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemProperties> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemProperties> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemProperties> {
        ::std::convert::TryInto::<super::IStorageItemProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::std::convert::TryInto::<super::IStorageItemPropertiesWithProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::std::convert::TryFrom<FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderInformation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::std::convert::TryFrom<&FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderInformation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Search::IStorageFolderQueryOperations> for FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Search::IStorageFolderQueryOperations> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Search::IStorageFolderQueryOperations> for &FolderInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Search::IStorageFolderQueryOperations> {
        ::std::convert::TryInto::<super::Search::IStorageFolderQueryOperations>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IFileInformationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileInformationFactory {
    type Vtable = IFileInformationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1075677374, 38415, 19821, [167, 208, 26, 56, 97, 231, 108, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IFileInformationFactoryFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileInformationFactoryFactory {
    type Vtable = IFileInformationFactoryFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2229931645, 58530, 20224, [138, 250, 175, 94, 15, 130, 107, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactoryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queryresult: ::windows::runtime::RawPtr, mode: super::FileProperties::ThumbnailMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))] usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queryresult: ::windows::runtime::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))] usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queryresult: ::windows::runtime::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))] usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queryresult: ::windows::runtime::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Storage_BulkAccess`*"]
pub struct IStorageItemInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorageItemInformation {
    type Vtable = IStorageItemInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275789707, 35186, 20288, [141, 224, 216, 111, 177, 121, 216, 250]);
}
impl IStorageItemInformation {
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn MusicProperties(&self) -> ::windows::runtime::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn VideoProperties(&self) -> ::windows::runtime::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn ImageProperties(&self) -> ::windows::runtime::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn DocumentProperties(&self) -> ::windows::runtime::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`*"]
    pub fn BasicProperties(&self) -> ::windows::runtime::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_BulkAccess`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn ThumbnailUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::runtime::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn PropertiesUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::runtime::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_BulkAccess`, `Foundation`*"]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IStorageItemInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{87a5cb8b-8972-4f40-8de0-d86fb179d8fa}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changedhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changedhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
