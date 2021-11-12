#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FileExtensionVector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileExtensionVector {}
impl ::core::clone::Clone for FileExtensionVector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileOpenPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileOpenPicker {}
impl ::core::clone::Clone for FileOpenPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FilePickerFileTypesOrderedMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FilePickerFileTypesOrderedMap {}
impl ::core::clone::Clone for FilePickerFileTypesOrderedMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FilePickerSelectedFilesArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FilePickerSelectedFilesArray {}
impl ::core::clone::Clone for FilePickerSelectedFilesArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileSavePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileSavePicker {}
impl ::core::clone::Clone for FileSavePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FolderPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FolderPicker {}
impl ::core::clone::Clone for FolderPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPicker {}
impl ::core::clone::Clone for IFileOpenPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPicker2 {}
impl ::core::clone::Clone for IFileOpenPicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPicker3 {}
impl ::core::clone::Clone for IFileOpenPicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerStatics {}
impl ::core::clone::Clone for IFileOpenPickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerStatics2 {}
impl ::core::clone::Clone for IFileOpenPickerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerWithOperationId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerWithOperationId {}
impl ::core::clone::Clone for IFileOpenPickerWithOperationId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePicker {}
impl ::core::clone::Clone for IFileSavePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePicker2 {}
impl ::core::clone::Clone for IFileSavePicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePicker3 {}
impl ::core::clone::Clone for IFileSavePicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePicker4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePicker4 {}
impl ::core::clone::Clone for IFileSavePicker4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePickerStatics {}
impl ::core::clone::Clone for IFileSavePickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFolderPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFolderPicker {}
impl ::core::clone::Clone for IFolderPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFolderPicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFolderPicker2 {}
impl ::core::clone::Clone for IFolderPicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFolderPicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFolderPicker3 {}
impl ::core::clone::Clone for IFolderPicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFolderPickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFolderPickerStatics {}
impl ::core::clone::Clone for IFolderPickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerLocationId(pub i32);
impl PickerLocationId {
    pub const DocumentsLibrary: Self = Self(0i32);
    pub const ComputerFolder: Self = Self(1i32);
    pub const Desktop: Self = Self(2i32);
    pub const Downloads: Self = Self(3i32);
    pub const HomeGroup: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const PicturesLibrary: Self = Self(6i32);
    pub const VideosLibrary: Self = Self(7i32);
    pub const Objects3D: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for PickerLocationId {}
impl ::core::clone::Clone for PickerLocationId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerViewMode(pub i32);
impl PickerViewMode {
    pub const List: Self = Self(0i32);
    pub const Thumbnail: Self = Self(1i32);
}
impl ::core::marker::Copy for PickerViewMode {}
impl ::core::clone::Clone for PickerViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
