#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for FileExtensionVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for FileExtensionVector {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for FileExtensionVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileExtensionVector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FileOpenPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPicker {}
impl ::core::fmt::Debug for FileOpenPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPicker").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for FilePickerFileTypesOrderedMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for FilePickerFileTypesOrderedMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for FilePickerFileTypesOrderedMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilePickerFileTypesOrderedMap").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for FilePickerSelectedFilesArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for FilePickerSelectedFilesArray {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for FilePickerSelectedFilesArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilePickerSelectedFilesArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FileSavePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePicker {}
impl ::core::fmt::Debug for FileSavePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePicker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FolderPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderPicker {}
impl ::core::fmt::Debug for FolderPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderPicker").field(&self.0).finish()
    }
}
impl ::core::default::Default for PickerLocationId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PickerLocationId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerLocationId").field(&self.0).finish()
    }
}
impl ::core::default::Default for PickerViewMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PickerViewMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerViewMode").field(&self.0).finish()
    }
}
