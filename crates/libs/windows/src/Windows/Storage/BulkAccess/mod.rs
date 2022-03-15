#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FileInformation(::windows::core::IUnknown);
impl FileInformation {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenSequentialReadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenReadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenAsync(&self, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenAsync)(::core::mem::transmute_copy(this), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultNameAndOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverload<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverload)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyAndReplaceAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultNameAndOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverload<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverload)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveAndReplaceAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenWithOptionsAsync(&self, accessmode: super::FileAccessMode, options: super::StorageOpenOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenWithOptionsAsync)(::core::mem::transmute_copy(this), accessmode, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: super::StorageOpenOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<super::FileAttributes> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BasicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThumbnailUpdated)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThumbnailUpdated)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesUpdated)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesUpdated)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Provider(&self) -> ::windows::core::Result<super::StorageProvider> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for FileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInformation {}
impl ::core::fmt::Debug for FileInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows::core::GUID = <IStorageItemInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformation";
}
impl ::core::convert::From<FileInformation> for ::windows::core::IUnknown {
    fn from(value: FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformation> for ::windows::core::IUnknown {
    fn from(value: &FileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileInformation> for ::windows::core::IInspectable {
    fn from(value: FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformation> for ::windows::core::IInspectable {
    fn from(value: &FileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IInputStreamReference> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IInputStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IInputStreamReference> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IInputStreamReference> {
        ::core::convert::TryInto::<super::Streams::IInputStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStreamReference> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IRandomAccessStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStreamReference> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<super::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFile {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFile {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFile> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFile> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFile> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFile> {
        ::core::convert::TryInto::<super::IStorageFile>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFile2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFile2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFile2> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFile2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFile2> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFile2> {
        ::core::convert::TryInto::<super::IStorageFile2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFilePropertiesWithAvailability> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFilePropertiesWithAvailability> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFilePropertiesWithAvailability> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFilePropertiesWithAvailability> {
        ::core::convert::TryInto::<super::IStorageFilePropertiesWithAvailability>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem> {
        ::core::convert::TryInto::<super::IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem2> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem2> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem2> {
        ::core::convert::TryInto::<super::IStorageItem2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemInformation> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemInformation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemInformation> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemInformation> {
        ::core::convert::TryInto::<IStorageItemInformation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemProperties> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemProperties> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemProperties> {
        ::core::convert::TryInto::<super::IStorageItemProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for &FileInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::core::convert::TryInto::<super::IStorageItemPropertiesWithProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FileInformationFactory(::windows::core::IUnknown);
impl FileInformationFactory {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn GetVirtualizedItemsVector(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVirtualizedItemsVector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn GetVirtualizedFilesVector(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVirtualizedFilesVector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn GetVirtualizedFoldersVector(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVirtualizedFoldersVector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithMode<'a, Param0: ::windows::core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithMode)(::core::mem::transmute_copy(this), queryresult.into_param().abi(), mode, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSize<'a, Param0: ::windows::core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows::core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithModeAndSize)(::core::mem::transmute_copy(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptions<'a, Param0: ::windows::core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithModeAndSizeAndOptions)(::core::mem::transmute_copy(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, thumbnailoptions, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptionsAndFlags<'a, Param0: ::windows::core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows::core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithModeAndSizeAndOptionsAndFlags)(::core::mem::transmute_copy(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, thumbnailoptions, delayload, &mut result__).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileInformationFactoryFactory<R, F: FnOnce(&IFileInformationFactoryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileInformationFactory, IFileInformationFactoryFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FileInformationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInformationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInformationFactory {}
impl ::core::fmt::Debug for FileInformationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInformationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileInformationFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformationFactory;{401d88be-960f-4d6d-a7d0-1a3861e76c83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
    const IID: ::windows::core::GUID = <IFileInformationFactory as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformationFactory";
}
impl ::core::convert::From<FileInformationFactory> for ::windows::core::IUnknown {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformationFactory> for ::windows::core::IUnknown {
    fn from(value: &FileInformationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileInformationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileInformationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileInformationFactory> for ::windows::core::IInspectable {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformationFactory> for ::windows::core::IInspectable {
    fn from(value: &FileInformationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileInformationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileInformationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FileInformationFactory {}
unsafe impl ::core::marker::Sync for FileInformationFactory {}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FolderInformation(::windows::core::IUnknown);
impl FolderInformation {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: super::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: super::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetItemAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    pub fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedStateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQuery(&self, query: super::Search::CommonFileQuery) -> ::windows::core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQuery(&self, query: super::Search::CommonFolderQuery) -> ::windows::core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQuery(&self) -> ::windows::core::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQuery)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsync(&self, query: super::Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsync(&self, query: super::Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows::core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreQueryOptionsSupported)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFolderQuerySupported(&self, query: super::Search::CommonFolderQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFolderQuerySupported)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFileQuerySupported(&self, query: super::Search::CommonFileQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFileQuerySupported)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<super::FileAttributes> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BasicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThumbnailUpdated)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThumbnailUpdated)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesUpdated)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesUpdated)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Provider(&self) -> ::windows::core::Result<super::StorageProvider> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for FolderInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderInformation {}
impl ::core::fmt::Debug for FolderInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FolderInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FolderInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FolderInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows::core::GUID = <IStorageItemInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FolderInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FolderInformation";
}
impl ::core::convert::From<FolderInformation> for ::windows::core::IUnknown {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderInformation> for ::windows::core::IUnknown {
    fn from(value: &FolderInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FolderInformation> for ::windows::core::IInspectable {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderInformation> for ::windows::core::IInspectable {
    fn from(value: &FolderInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageFolder {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageFolder {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFolder> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFolder> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFolder> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFolder> {
        ::core::convert::TryInto::<super::IStorageFolder>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFolder2> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFolder2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageFolder2> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageFolder2> {
        ::core::convert::TryInto::<super::IStorageFolder2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<&FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows::core::IntoParam<'a, super::Search::IStorageFolderQueryOperations> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Search::IStorageFolderQueryOperations> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows::core::IntoParam<'a, super::Search::IStorageFolderQueryOperations> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Search::IStorageFolderQueryOperations> {
        ::core::convert::TryInto::<super::Search::IStorageFolderQueryOperations>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem> {
        ::core::convert::TryInto::<super::IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem2> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItem2> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItem2> {
        ::core::convert::TryInto::<super::IStorageItem2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemInformation> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemInformation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemInformation> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemInformation> {
        ::core::convert::TryInto::<IStorageItemInformation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemProperties> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemProperties> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemProperties> {
        ::core::convert::TryInto::<super::IStorageItemProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for &FolderInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::core::convert::TryInto::<super::IStorageItemPropertiesWithProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileInformationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401d88be_960f_4d6d_a7d0_1a3861e76c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
    pub GetVirtualizedItemsVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVirtualizedFilesVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVirtualizedFoldersVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileInformationFactoryFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileInformationFactoryFactory {
    type Vtable = IFileInformationFactoryFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84ea0e7d_e4a2_4f00_8afa_af5e0f826bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactoryFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithMode: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSize: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptions: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptionsAndFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptionsAndFlags: usize,
}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct IStorageItemInformation(::windows::core::IUnknown);
impl IStorageItemInformation {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BasicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThumbnailUpdated)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThumbnailUpdated)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesUpdated)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesUpdated)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IStorageItemInformation> for ::windows::core::IUnknown {
    fn from(value: IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemInformation> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemInformation> for ::windows::core::IInspectable {
    fn from(value: IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemInformation> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItemInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemInformation {}
impl ::core::fmt::Debug for IStorageItemInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{87a5cb8b-8972-4f40-8de0-d86fb179d8fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a5cb8b_8972_4f40_8de0_d86fb179d8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    ImageProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub DocumentProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    DocumentProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub BasicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    BasicProperties: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub ThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertiesUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertiesUpdated: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
