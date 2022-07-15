#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
#[doc = "*Required features: `\"Storage_Pickers\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct FileExtensionVector(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl FileExtensionVector {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IIterator<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &::windows::core::HSTRING, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::windows::core::Interface::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::windows::core::Interface::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::windows::core::Interface::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::windows::core::HSTRING]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::windows::core::HSTRING]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::windows::core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for FileExtensionVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for FileExtensionVector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.FileExtensionVector;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};string))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for FileExtensionVector {
    type Vtable = super::super::Foundation::Collections::IVector_Vtbl<::windows::core::HSTRING>;
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVector<::windows::core::HSTRING> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for FileExtensionVector {
    const NAME: &'static str = "Windows.Storage.Pickers.FileExtensionVector";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FileExtensionVector {
    type Item = ::windows::core::HSTRING;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FileExtensionVector {
    type Item = ::windows::core::HSTRING;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<FileExtensionVector> for ::windows::core::IUnknown {
    fn from(value: FileExtensionVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FileExtensionVector> for ::windows::core::IUnknown {
    fn from(value: &FileExtensionVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FileExtensionVector> for &::windows::core::IUnknown {
    fn from(value: &FileExtensionVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<FileExtensionVector> for ::windows::core::IInspectable {
    fn from(value: FileExtensionVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FileExtensionVector> for ::windows::core::IInspectable {
    fn from(value: &FileExtensionVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FileExtensionVector> for &::windows::core::IInspectable {
    fn from(value: &FileExtensionVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FileExtensionVector> for super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: FileExtensionVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FileExtensionVector> for super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileExtensionVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&FileExtensionVector> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileExtensionVector) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FileExtensionVector> for super::super::Foundation::Collections::IVector<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: FileExtensionVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FileExtensionVector> for super::super::Foundation::Collections::IVector<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileExtensionVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&FileExtensionVector> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileExtensionVector) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for FileExtensionVector {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for FileExtensionVector {}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
pub struct FileOpenPicker(::windows::core::IUnknown);
impl FileOpenPicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileOpenPicker, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewMode(&self) -> ::windows::core::Result<PickerViewMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PickerViewMode>(result__)
        }
    }
    pub fn SetViewMode(&self, value: PickerViewMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetViewMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SettingsIdentifier)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSettingsIdentifier)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuggestedStartLocation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PickerLocationId>(result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuggestedStartLocation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CommitButtonText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommitButtonText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileTypeFilter)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickSingleFileAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PickMultipleFilesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickMultipleFilesAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IFileOpenPicker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PickSingleFileAndContinue(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileOpenPicker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PickSingleFileAndContinue)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PickMultipleFilesAndContinue(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileOpenPicker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PickMultipleFilesAndContinue)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IFileOpenPicker3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ResumePickSingleFileAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        Self::IFileOpenPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ResumePickSingleFileAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<'a, P0>(user: P0) -> ::windows::core::Result<FileOpenPicker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::User>>,
    {
        Self::IFileOpenPickerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateForUser)(::windows::core::Interface::as_raw(this), user.into().abi(), result__.as_mut_ptr()).from_abi::<FileOpenPicker>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleFileAsync2(&self, pickeroperationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<IFileOpenPickerWithOperationId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickSingleFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pickeroperationid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IFileOpenPickerStatics<R, F: FnOnce(&IFileOpenPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileOpenPicker, IFileOpenPickerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFileOpenPickerStatics2<R, F: FnOnce(&IFileOpenPickerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileOpenPicker, IFileOpenPickerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FileOpenPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for FileOpenPicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.FileOpenPicker;{2ca8278a-12c5-4c5f-8977-94547793c241})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileOpenPicker {
    type Vtable = IFileOpenPicker_Vtbl;
    const IID: ::windows::core::GUID = <IFileOpenPicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileOpenPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FileOpenPicker";
}
impl ::core::convert::From<FileOpenPicker> for ::windows::core::IUnknown {
    fn from(value: FileOpenPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPicker> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileOpenPicker> for &::windows::core::IUnknown {
    fn from(value: &FileOpenPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileOpenPicker> for ::windows::core::IInspectable {
    fn from(value: FileOpenPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPicker> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileOpenPicker> for &::windows::core::IInspectable {
    fn from(value: &FileOpenPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for FileOpenPicker {}
unsafe impl ::core::marker::Sync for FileOpenPicker {}
#[doc = "*Required features: `\"Storage_Pickers\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct FilePickerFileTypesOrderedMap(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl FilePickerFileTypesOrderedMap {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, P0, E0>(&self, key: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Insert)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for FilePickerFileTypesOrderedMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for FilePickerFileTypesOrderedMap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.FilePickerFileTypesOrderedMap;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};string)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for FilePickerFileTypesOrderedMap {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for FilePickerFileTypesOrderedMap {
    const NAME: &'static str = "Windows.Storage.Pickers.FilePickerFileTypesOrderedMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FilePickerFileTypesOrderedMap {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FilePickerFileTypesOrderedMap {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<FilePickerFileTypesOrderedMap> for ::windows::core::IUnknown {
    fn from(value: FilePickerFileTypesOrderedMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerFileTypesOrderedMap> for ::windows::core::IUnknown {
    fn from(value: &FilePickerFileTypesOrderedMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerFileTypesOrderedMap> for &::windows::core::IUnknown {
    fn from(value: &FilePickerFileTypesOrderedMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<FilePickerFileTypesOrderedMap> for ::windows::core::IInspectable {
    fn from(value: FilePickerFileTypesOrderedMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerFileTypesOrderedMap> for ::windows::core::IInspectable {
    fn from(value: &FilePickerFileTypesOrderedMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerFileTypesOrderedMap> for &::windows::core::IInspectable {
    fn from(value: &FilePickerFileTypesOrderedMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FilePickerFileTypesOrderedMap> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: FilePickerFileTypesOrderedMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FilePickerFileTypesOrderedMap> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerFileTypesOrderedMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&FilePickerFileTypesOrderedMap> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerFileTypesOrderedMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FilePickerFileTypesOrderedMap> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: FilePickerFileTypesOrderedMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FilePickerFileTypesOrderedMap> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerFileTypesOrderedMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&FilePickerFileTypesOrderedMap> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerFileTypesOrderedMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for FilePickerFileTypesOrderedMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for FilePickerFileTypesOrderedMap {}
#[doc = "*Required features: `\"Storage_Pickers\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct FilePickerSelectedFilesArray(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl FilePickerSelectedFilesArray {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::StorageFile>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IIterator<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<super::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::StorageFile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.into().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::StorageFile>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for FilePickerSelectedFilesArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for FilePickerSelectedFilesArray {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.FilePickerSelectedFilesArray;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Storage.StorageFile;{fa3f6186-4214-428c-a64c-14c9ac7315ea})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for FilePickerSelectedFilesArray {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::StorageFile>;
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVectorView<super::StorageFile> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for FilePickerSelectedFilesArray {
    const NAME: &'static str = "Windows.Storage.Pickers.FilePickerSelectedFilesArray";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FilePickerSelectedFilesArray {
    type Item = super::StorageFile;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FilePickerSelectedFilesArray {
    type Item = super::StorageFile;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<FilePickerSelectedFilesArray> for ::windows::core::IUnknown {
    fn from(value: FilePickerSelectedFilesArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerSelectedFilesArray> for ::windows::core::IUnknown {
    fn from(value: &FilePickerSelectedFilesArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerSelectedFilesArray> for &::windows::core::IUnknown {
    fn from(value: &FilePickerSelectedFilesArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<FilePickerSelectedFilesArray> for ::windows::core::IInspectable {
    fn from(value: FilePickerSelectedFilesArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerSelectedFilesArray> for ::windows::core::IInspectable {
    fn from(value: &FilePickerSelectedFilesArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&FilePickerSelectedFilesArray> for &::windows::core::IInspectable {
    fn from(value: &FilePickerSelectedFilesArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FilePickerSelectedFilesArray> for super::super::Foundation::Collections::IIterable<super::StorageFile> {
    type Error = ::windows::core::Error;
    fn try_from(value: FilePickerSelectedFilesArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FilePickerSelectedFilesArray> for super::super::Foundation::Collections::IIterable<super::StorageFile> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerSelectedFilesArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&FilePickerSelectedFilesArray> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::StorageFile>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerSelectedFilesArray) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FilePickerSelectedFilesArray> for super::super::Foundation::Collections::IVectorView<super::StorageFile> {
    type Error = ::windows::core::Error;
    fn try_from(value: FilePickerSelectedFilesArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FilePickerSelectedFilesArray> for super::super::Foundation::Collections::IVectorView<super::StorageFile> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerSelectedFilesArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&FilePickerSelectedFilesArray> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IVectorView<super::StorageFile>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FilePickerSelectedFilesArray) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for FilePickerSelectedFilesArray {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for FilePickerSelectedFilesArray {}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
pub struct FileSavePicker(::windows::core::IUnknown);
impl FileSavePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileSavePicker, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SettingsIdentifier)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSettingsIdentifier)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuggestedStartLocation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PickerLocationId>(result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuggestedStartLocation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CommitButtonText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommitButtonText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeChoices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileTypeChoices)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        }
    }
    pub fn DefaultFileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DefaultFileExtension)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDefaultFileExtension(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultFileExtension)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedSaveFile(&self) -> ::windows::core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuggestedSaveFile)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFile>(result__)
        }
    }
    pub fn SetSuggestedSaveFile<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::StorageFile>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuggestedSaveFile)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn SuggestedFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuggestedFileName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSuggestedFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuggestedFileName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSaveFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickSaveFileAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IFileSavePicker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PickSaveFileAndContinue(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileSavePicker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PickSaveFileAndContinue)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePicker3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileSavePicker3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetEnterpriseId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IFileSavePicker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<'a, P0>(user: P0) -> ::windows::core::Result<FileSavePicker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::User>>,
    {
        Self::IFileSavePickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateForUser)(::windows::core::Interface::as_raw(this), user.into().abi(), result__.as_mut_ptr()).from_abi::<FileSavePicker>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileSavePickerStatics<R, F: FnOnce(&IFileSavePickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileSavePicker, IFileSavePickerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FileSavePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for FileSavePicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.FileSavePicker;{3286ffcb-617f-4cc5-af6a-b3fdf29ad145})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileSavePicker {
    type Vtable = IFileSavePicker_Vtbl;
    const IID: ::windows::core::GUID = <IFileSavePicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileSavePicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FileSavePicker";
}
impl ::core::convert::From<FileSavePicker> for ::windows::core::IUnknown {
    fn from(value: FileSavePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePicker> for ::windows::core::IUnknown {
    fn from(value: &FileSavePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileSavePicker> for &::windows::core::IUnknown {
    fn from(value: &FileSavePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileSavePicker> for ::windows::core::IInspectable {
    fn from(value: FileSavePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePicker> for ::windows::core::IInspectable {
    fn from(value: &FileSavePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileSavePicker> for &::windows::core::IInspectable {
    fn from(value: &FileSavePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for FileSavePicker {}
unsafe impl ::core::marker::Sync for FileSavePicker {}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
pub struct FolderPicker(::windows::core::IUnknown);
impl FolderPicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FolderPicker, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewMode(&self) -> ::windows::core::Result<PickerViewMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViewMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PickerViewMode>(result__)
        }
    }
    pub fn SetViewMode(&self, value: PickerViewMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetViewMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SettingsIdentifier)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSettingsIdentifier)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuggestedStartLocation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PickerLocationId>(result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuggestedStartLocation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CommitButtonText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommitButtonText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileTypeFilter)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleFolderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PickSingleFolderAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IFolderPicker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PickFolderAndContinue(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFolderPicker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PickFolderAndContinue)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IFolderPicker3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<'a, P0>(user: P0) -> ::windows::core::Result<FolderPicker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::User>>,
    {
        Self::IFolderPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateForUser)(::windows::core::Interface::as_raw(this), user.into().abi(), result__.as_mut_ptr()).from_abi::<FolderPicker>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFolderPickerStatics<R, F: FnOnce(&IFolderPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FolderPicker, IFolderPickerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FolderPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for FolderPicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.FolderPicker;{084f7799-f3fb-400a-99b1-7b4a772fd60d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FolderPicker {
    type Vtable = IFolderPicker_Vtbl;
    const IID: ::windows::core::GUID = <IFolderPicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FolderPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FolderPicker";
}
impl ::core::convert::From<FolderPicker> for ::windows::core::IUnknown {
    fn from(value: FolderPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderPicker> for ::windows::core::IUnknown {
    fn from(value: &FolderPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FolderPicker> for &::windows::core::IUnknown {
    fn from(value: &FolderPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FolderPicker> for ::windows::core::IInspectable {
    fn from(value: FolderPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderPicker> for ::windows::core::IInspectable {
    fn from(value: &FolderPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FolderPicker> for &::windows::core::IInspectable {
    fn from(value: &FolderPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for FolderPicker {}
unsafe impl ::core::marker::Sync for FolderPicker {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPicker {
    type Vtable = IFileOpenPicker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ca8278a_12c5_4c5f_8977_94547793c241);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickMultipleFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickMultipleFilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPicker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPicker2 {
    type Vtable = IFileOpenPicker2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ceb6cd2_b446_46f7_b265_90f8e55ad650);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickSingleFileAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickSingleFileAndContinue: usize,
    #[cfg(feature = "deprecated")]
    pub PickMultipleFilesAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickMultipleFilesAndContinue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPicker3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPicker3 {
    type Vtable = IFileOpenPicker3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9a5c5b3_c5dc_5b98_bd80_a8d0ca0584d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPickerStatics {
    type Vtable = IFileOpenPickerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6821573b_2f02_4833_96d4_abbfad72b67b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResumePickSingleFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResumePickSingleFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPickerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPickerStatics2 {
    type Vtable = IFileOpenPickerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8917415_eddd_5c98_b6f3_366fdfcad392);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPickerWithOperationId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPickerWithOperationId {
    type Vtable = IFileOpenPickerWithOperationId_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57b569_2522_4ca5_aa73_a15509f1fcbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerWithOperationId_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pickeroperationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePicker {
    type Vtable = IFileSavePicker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3286ffcb_617f_4cc5_af6a_b3fdf29ad145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeChoices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeChoices: usize,
    pub DefaultFileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDefaultFileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SuggestedSaveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSuggestedSaveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuggestedFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSuggestedFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSaveFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSaveFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePicker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePicker2 {
    type Vtable = IFileSavePicker2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ec313a2_d24b_449a_8197_e89104fd42cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickSaveFileAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickSaveFileAndContinue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePicker3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePicker3 {
    type Vtable = IFileSavePicker3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x698aec69_ba3c_4e51_bd90_4abcbbf4cfaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePicker4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePicker4 {
    type Vtable = IFileSavePicker4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d83a5a_ddfa_5de0_8b70_c842c21988ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePickerStatics {
    type Vtable = IFileSavePickerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28e3cf9e_961c_5e2c_aed7_e64737f4ce37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderPicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFolderPicker {
    type Vtable = IFolderPicker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x084f7799_f3fb_400a_99b1_7b4a772fd60d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderPicker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFolderPicker2 {
    type Vtable = IFolderPicker2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8eb3ba97_dc85_4616_be94_9660881f2f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickFolderAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickFolderAndContinue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderPicker3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFolderPicker3 {
    type Vtable = IFolderPicker3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x673b1e29_d326_53c0_bd24_a25c714cee36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFolderPickerStatics {
    type Vtable = IFolderPickerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9be34740_7ca1_5942_a3c8_46f2551ecff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for PickerLocationId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PickerLocationId {
    type Abi = Self;
}
impl ::core::fmt::Debug for PickerLocationId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerLocationId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PickerLocationId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.PickerLocationId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for PickerViewMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PickerViewMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PickerViewMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerViewMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PickerViewMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.PickerViewMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
