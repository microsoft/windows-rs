#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for AddFileResult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AddFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddFileResult {}
impl ::core::fmt::Debug for AddFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddFileResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.AddFileResult;i4)");
}
impl ::windows::core::DefaultType for AddFileResult {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct FileOpenPickerUI(::windows::core::IUnknown);
impl FileOpenPickerUI {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::IStorageFile>>(&self, id: Param0, file: Param1) -> ::windows::core::Result<AddFileResult> {
        let this = self;
        unsafe {
            let mut result__: AddFileResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<AddFileResult>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn RemoveFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn ContainsFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn CanAddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn SelectionMode(&self) -> ::windows::core::Result<FileSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: FileSelectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileSelectionMode>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FileRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFileRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
}
unsafe impl ::windows::core::Interface for FileOpenPickerUI {
    type Vtable = IFileOpenPickerUIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda45a10_f9d4_40c4_8af5_c5b6b5a61d1d);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileOpenPickerUI {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileOpenPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileRemovedEventArgs {
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13043da7_7fca_4c2b_9eca_6890f9f00185);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileRemovedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct FileSavePickerUI(::windows::core::IUnknown);
impl FileSavePickerUI {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn TrySetFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<SetFileNameResult> {
        let this = self;
        unsafe {
            let mut result__: SetFileNameResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<SetFileNameResult>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FileNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetFileRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetFileRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
}
unsafe impl ::windows::core::Interface for FileSavePickerUI {
    type Vtable = IFileSavePickerUIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9656c1e7_3e56_43cc_8a39_33c73d9d542b);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileSavePickerUI {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileSavePickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for FileSelectionMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FileSelectionMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSelectionMode {}
impl ::core::fmt::Debug for FileSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.FileSelectionMode;i4)");
}
impl ::windows::core::DefaultType for FileSelectionMode {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileOpenPickerUI {
    type Vtable = IFileOpenPickerUIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda45a10_f9d4_40c4_8af5_c5b6b5a61d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerUIVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr, result__: *mut AddFileResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileSelectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13043da7_7fca_4c2b_9eca_6890f9f00185);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileRemovedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileSavePickerUI {
    type Vtable = IFileSavePickerUIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9656c1e7_3e56_43cc_8a39_33c73d9d542b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerUIVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut SetFileNameResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPickerClosingDeferral {
    type Vtable = IPickerClosingDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7af7f71e_1a67_4a31_ae80_e907708a619b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e59f224_b332_4f12_8b9f_a8c2f06b32cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPickerClosingOperation {
    type Vtable = IPickerClosingOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9fb84_beee_4e39_a773_fc5f0eae328d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetFileRequest {
    type Vtable = ITargetFileRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42bd3355_7f88_478b_8e81_690b20340678);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aee9d91_bf15_4da9_95f6_f6b7d558225b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb163dbc1_1b51_4c89_a591_0fd40b3c57c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct PickerClosingDeferral(::windows::core::IUnknown);
impl PickerClosingDeferral {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
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
}
unsafe impl ::windows::core::Interface for PickerClosingDeferral {
    type Vtable = IPickerClosingDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7af7f71e_1a67_4a31_ae80_e907708a619b);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PickerClosingDeferral {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PickerClosingDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct PickerClosingEventArgs(::windows::core::IUnknown);
impl PickerClosingEventArgs {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn ClosingOperation(&self) -> ::windows::core::Result<PickerClosingOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PickerClosingOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
}
unsafe impl ::windows::core::Interface for PickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e59f224_b332_4f12_8b9f_a8c2f06b32cd);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PickerClosingEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PickerClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct PickerClosingOperation(::windows::core::IUnknown);
impl PickerClosingOperation {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<PickerClosingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PickerClosingDeferral>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
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
}
unsafe impl ::windows::core::Interface for PickerClosingOperation {
    type Vtable = IPickerClosingOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9fb84_beee_4e39_a773_fc5f0eae328d);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PickerClosingOperation {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PickerClosingOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for SetFileNameResult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SetFileNameResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SetFileNameResult {}
impl ::core::fmt::Debug for SetFileNameResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetFileNameResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetFileNameResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.SetFileNameResult;i4)");
}
impl ::windows::core::DefaultType for SetFileNameResult {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct TargetFileRequest(::windows::core::IUnknown);
impl TargetFileRequest {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn TargetFile(&self) -> ::windows::core::Result<super::super::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn SetTargetFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<TargetFileRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetFileRequestDeferral>(result__)
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
}
unsafe impl ::windows::core::Interface for TargetFileRequest {
    type Vtable = ITargetFileRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42bd3355_7f88_478b_8e81_690b20340678);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetFileRequest {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetFileRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct TargetFileRequestDeferral(::windows::core::IUnknown);
impl TargetFileRequestDeferral {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
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
}
unsafe impl ::windows::core::Interface for TargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aee9d91_bf15_4da9_95f6_f6b7d558225b);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetFileRequestDeferral {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetFileRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Storage_Pickers_Provider'*"]
#[repr(transparent)]
pub struct TargetFileRequestedEventArgs(::windows::core::IUnknown);
impl TargetFileRequestedEventArgs {
    #[doc = "*Required features: 'Storage_Pickers_Provider'*"]
    pub fn Request(&self) -> ::windows::core::Result<TargetFileRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetFileRequest>(result__)
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
}
unsafe impl ::windows::core::Interface for TargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb163dbc1_1b51_4c89_a591_0fd40b3c57c9);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetFileRequestedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetFileRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
