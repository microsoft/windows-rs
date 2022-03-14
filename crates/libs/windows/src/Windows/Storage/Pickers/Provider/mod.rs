#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AddFileResult(pub i32);
impl AddFileResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const NotAllowed: Self = Self(2i32);
    pub const Unavailable: Self = Self(3i32);
}
impl ::core::marker::Copy for AddFileResult {}
impl ::core::clone::Clone for AddFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddFileResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AddFileResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.AddFileResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct FileOpenPickerUI(::windows::core::IUnknown);
impl FileOpenPickerUI {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::IStorageFile>>(&self, id: Param0, file: Param1) -> ::windows::core::Result<AddFileResult> {
        let this = self;
        unsafe {
            let mut result__: AddFileResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddFile)(::core::mem::transmute_copy(this), id.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<AddFileResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn RemoveFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFile)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn ContainsFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFile)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn CanAddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanAddFile)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowedFileTypes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn SelectionMode(&self) -> ::windows::core::Result<FileSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: FileSelectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileSelectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SettingsIdentifier)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FileRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileRemoved)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFileRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closing)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosing)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FileOpenPickerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileOpenPickerUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPickerUI {}
impl ::core::fmt::Debug for FileOpenPickerUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileOpenPickerUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.FileOpenPickerUI;{dda45a10-f9d4-40c4-8af5-c5b6b5a61d1d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_Vtbl;
    const IID: ::windows::core::GUID = <IFileOpenPickerUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileOpenPickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileOpenPickerUI";
}
impl ::core::convert::From<FileOpenPickerUI> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerUI> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileOpenPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileOpenPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileOpenPickerUI> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerUI> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileOpenPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileOpenPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileRemovedEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileRemovedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for FileRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.FileRemovedEventArgs;{13043da7-7fca-4c2b-9eca-6890f9f00185})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileRemovedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct FileSavePickerUI(::windows::core::IUnknown);
impl FileSavePickerUI {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowedFileTypes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SettingsIdentifier)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn TrySetFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<SetFileNameResult> {
        let this = self;
        unsafe {
            let mut result__: SetFileNameResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetFileName)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<SetFileNameResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileNameChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileNameChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetFileRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetFileRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetFileRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTargetFileRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FileSavePickerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileSavePickerUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePickerUI {}
impl ::core::fmt::Debug for FileSavePickerUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileSavePickerUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.FileSavePickerUI;{9656c1e7-3e56-43cc-8a39-33c73d9d542b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileSavePickerUI {
    type Vtable = IFileSavePickerUI_Vtbl;
    const IID: ::windows::core::GUID = <IFileSavePickerUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileSavePickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileSavePickerUI";
}
impl ::core::convert::From<FileSavePickerUI> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerUI> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileSavePickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileSavePickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileSavePickerUI> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerUI> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileSavePickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileSavePickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileSelectionMode(pub i32);
impl FileSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for FileSelectionMode {}
impl ::core::clone::Clone for FileSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.FileSelectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda45a10_f9d4_40c4_8af5_c5b6b5a61d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerUI_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr, result__: *mut AddFileResult) -> ::windows::core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContainsFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanAddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileSelectionMode) -> ::windows::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FileRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FileRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFileRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFileRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13043da7_7fca_4c2b_9eca_6890f9f00185);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileRemovedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePickerUI {
    type Vtable = IFileSavePickerUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9656c1e7_3e56_43cc_8a39_33c73d9d542b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerUI_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TrySetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut SetFileNameResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TargetFileRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetFileRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetFileRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetFileRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7af7f71e_1a67_4a31_ae80_e907708a619b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e59f224_b332_4f12_8b9f_a8c2f06b32cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ClosingOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPickerClosingOperation {
    type Vtable = IPickerClosingOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9fb84_beee_4e39_a773_fc5f0eae328d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetFileRequest {
    type Vtable = ITargetFileRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42bd3355_7f88_478b_8e81_690b20340678);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TargetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTargetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aee9d91_bf15_4da9_95f6_f6b7d558225b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb163dbc1_1b51_4c89_a591_0fd40b3c57c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct PickerClosingDeferral(::windows::core::IUnknown);
impl PickerClosingDeferral {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PickerClosingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerClosingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerClosingDeferral {}
impl ::core::fmt::Debug for PickerClosingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerClosingDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PickerClosingDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.PickerClosingDeferral;{7af7f71e-1a67-4a31-ae80-e907708a619b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IPickerClosingDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerClosingDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingDeferral";
}
impl ::core::convert::From<PickerClosingDeferral> for ::windows::core::IUnknown {
    fn from(value: PickerClosingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerClosingDeferral> for ::windows::core::IUnknown {
    fn from(value: &PickerClosingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PickerClosingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PickerClosingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerClosingDeferral> for ::windows::core::IInspectable {
    fn from(value: PickerClosingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerClosingDeferral> for ::windows::core::IInspectable {
    fn from(value: &PickerClosingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PickerClosingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PickerClosingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct PickerClosingEventArgs(::windows::core::IUnknown);
impl PickerClosingEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn ClosingOperation(&self) -> ::windows::core::Result<PickerClosingOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClosingOperation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PickerClosingOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PickerClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerClosingEventArgs {}
impl ::core::fmt::Debug for PickerClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerClosingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PickerClosingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.PickerClosingEventArgs;{7e59f224-b332-4f12-8b9f-a8c2f06b32cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPickerClosingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerClosingEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingEventArgs";
}
impl ::core::convert::From<PickerClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PickerClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PickerClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PickerClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PickerClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PickerClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PickerClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PickerClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PickerClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct PickerClosingOperation(::windows::core::IUnknown);
impl PickerClosingOperation {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<PickerClosingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PickerClosingDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for PickerClosingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerClosingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerClosingOperation {}
impl ::core::fmt::Debug for PickerClosingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerClosingOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PickerClosingOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.PickerClosingOperation;{4ce9fb84-beee-4e39-a773-fc5f0eae328d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PickerClosingOperation {
    type Vtable = IPickerClosingOperation_Vtbl;
    const IID: ::windows::core::GUID = <IPickerClosingOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerClosingOperation {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingOperation";
}
impl ::core::convert::From<PickerClosingOperation> for ::windows::core::IUnknown {
    fn from(value: PickerClosingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerClosingOperation> for ::windows::core::IUnknown {
    fn from(value: &PickerClosingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PickerClosingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PickerClosingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerClosingOperation> for ::windows::core::IInspectable {
    fn from(value: PickerClosingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerClosingOperation> for ::windows::core::IInspectable {
    fn from(value: &PickerClosingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PickerClosingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PickerClosingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SetFileNameResult(pub i32);
impl SetFileNameResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NotAllowed: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for SetFileNameResult {}
impl ::core::clone::Clone for SetFileNameResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetFileNameResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SetFileNameResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for SetFileNameResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetFileNameResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetFileNameResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.SetFileNameResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct TargetFileRequest(::windows::core::IUnknown);
impl TargetFileRequest {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn TargetFile(&self) -> ::windows::core::Result<super::super::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetFile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn SetTargetFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetFile)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<TargetFileRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetFileRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetFileRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetFileRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetFileRequest {}
impl ::core::fmt::Debug for TargetFileRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetFileRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetFileRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.TargetFileRequest;{42bd3355-7f88-478b-8e81-690b20340678})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TargetFileRequest {
    type Vtable = ITargetFileRequest_Vtbl;
    const IID: ::windows::core::GUID = <ITargetFileRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TargetFileRequest {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequest";
}
impl ::core::convert::From<TargetFileRequest> for ::windows::core::IUnknown {
    fn from(value: TargetFileRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetFileRequest> for ::windows::core::IUnknown {
    fn from(value: &TargetFileRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetFileRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetFileRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetFileRequest> for ::windows::core::IInspectable {
    fn from(value: TargetFileRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetFileRequest> for ::windows::core::IInspectable {
    fn from(value: &TargetFileRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetFileRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetFileRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct TargetFileRequestDeferral(::windows::core::IUnknown);
impl TargetFileRequestDeferral {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for TargetFileRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetFileRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetFileRequestDeferral {}
impl ::core::fmt::Debug for TargetFileRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetFileRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetFileRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.TargetFileRequestDeferral;{4aee9d91-bf15-4da9-95f6-f6b7d558225b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = <ITargetFileRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TargetFileRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestDeferral";
}
impl ::core::convert::From<TargetFileRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: TargetFileRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetFileRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &TargetFileRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetFileRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetFileRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetFileRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: TargetFileRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetFileRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &TargetFileRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetFileRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetFileRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct TargetFileRequestedEventArgs(::windows::core::IUnknown);
impl TargetFileRequestedEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<TargetFileRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetFileRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetFileRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetFileRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetFileRequestedEventArgs {}
impl ::core::fmt::Debug for TargetFileRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetFileRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetFileRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs;{b163dbc1-1b51-4c89-a591-0fd40b3c57c9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITargetFileRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TargetFileRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs";
}
impl ::core::convert::From<TargetFileRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetFileRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetFileRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetFileRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetFileRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetFileRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetFileRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetFileRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetFileRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetFileRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetFileRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetFileRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
