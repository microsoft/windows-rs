#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddFileResult(pub i32);
impl AddFileResult {
    pub const Added: AddFileResult = AddFileResult(0i32);
    pub const AlreadyAdded: AddFileResult = AddFileResult(1i32);
    pub const NotAllowed: AddFileResult = AddFileResult(2i32);
    pub const Unavailable: AddFileResult = AddFileResult(3i32);
}
impl ::std::convert::From<i32> for AddFileResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AddFileResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AddFileResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.AddFileResult;i4)");
}
impl ::windows::runtime::DefaultType for AddFileResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct FileOpenPickerUI(::windows::runtime::IInspectable);
impl FileOpenPickerUI {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn AddFile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::IStorageFile>>(&self, id: Param0, file: Param1) -> ::windows::runtime::Result<AddFileResult> {
        let this = self;
        unsafe {
            let mut result__: AddFileResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), id.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<AddFileResult>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn RemoveFile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, id: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn ContainsFile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, id: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn CanAddFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::IStorageFile>>(&self, file: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation_Collections`*"]
    pub fn AllowedFileTypes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn SelectionMode(&self) -> ::windows::runtime::Result<FileSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: FileSelectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FileSelectionMode>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn SettingsIdentifier(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn FileRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn RemoveFileRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn Closing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn RemoveClosing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileOpenPickerUI {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.FileOpenPickerUI;{dda45a10-f9d4-40c4-8af5-c5b6b5a61d1d})");
}
unsafe impl ::windows::runtime::Interface for FileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3718535696, 63956, 16580, [138, 245, 197, 182, 181, 166, 29, 29]);
}
impl ::windows::runtime::RuntimeName for FileOpenPickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileOpenPickerUI";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct FileRemovedEventArgs(::windows::runtime::IInspectable);
impl FileRemovedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.FileRemovedEventArgs;{13043da7-7fca-4c2b-9eca-6890f9f00185})");
}
unsafe impl ::windows::runtime::Interface for FileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(319045031, 32714, 19499, [158, 202, 104, 144, 249, 240, 1, 133]);
}
impl ::windows::runtime::RuntimeName for FileRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileRemovedEventArgs";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct FileSavePickerUI(::windows::runtime::IInspectable);
impl FileSavePickerUI {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation_Collections`*"]
    pub fn AllowedFileTypes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn SettingsIdentifier(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn FileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn TrySetFileName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<SetFileNameResult> {
        let this = self;
        unsafe {
            let mut result__: SetFileNameResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<SetFileNameResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn FileNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn RemoveFileNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn TargetFileRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn RemoveTargetFileRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileSavePickerUI {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.FileSavePickerUI;{9656c1e7-3e56-43cc-8a39-33c73d9d542b})");
}
unsafe impl ::windows::runtime::Interface for FileSavePickerUI {
    type Vtable = IFileSavePickerUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2522268135, 15958, 17356, [138, 57, 51, 199, 61, 157, 84, 43]);
}
impl ::windows::runtime::RuntimeName for FileSavePickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileSavePickerUI";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileSelectionMode(pub i32);
impl FileSelectionMode {
    pub const Single: FileSelectionMode = FileSelectionMode(0i32);
    pub const Multiple: FileSelectionMode = FileSelectionMode(1i32);
}
impl ::std::convert::From<i32> for FileSelectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FileSelectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FileSelectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.FileSelectionMode;i4)");
}
impl ::windows::runtime::DefaultType for FileSelectionMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileOpenPickerUI(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3718535696, 63956, 16580, [138, 245, 197, 182, 181, 166, 29, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, file: ::windows::runtime::RawPtr, result__: *mut AddFileResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FileSelectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileRemovedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(319045031, 32714, 19499, [158, 202, 104, 144, 249, 240, 1, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileSavePickerUI(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileSavePickerUI {
    type Vtable = IFileSavePickerUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2522268135, 15958, 17356, [138, 57, 51, 199, 61, 157, 84, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut SetFileNameResult) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerClosingDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2063071006, 6759, 18993, [174, 128, 233, 7, 112, 138, 97, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerClosingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2119823908, 45874, 20242, [139, 159, 168, 194, 240, 107, 50, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPickerClosingOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPickerClosingOperation {
    type Vtable = IPickerClosingOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290402692, 48878, 20025, [167, 115, 252, 95, 14, 174, 50, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetFileRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITargetFileRequest {
    type Vtable = ITargetFileRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1119695701, 32648, 18315, [142, 129, 105, 11, 32, 52, 6, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetFileRequestDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1257151889, 48917, 19881, [149, 246, 246, 183, 213, 88, 34, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetFileRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2976111553, 6993, 19593, [165, 145, 15, 212, 11, 60, 87, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PickerClosingDeferral(::windows::runtime::IInspectable);
impl PickerClosingDeferral {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PickerClosingDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.PickerClosingDeferral;{7af7f71e-1a67-4a31-ae80-e907708a619b})");
}
unsafe impl ::windows::runtime::Interface for PickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2063071006, 6759, 18993, [174, 128, 233, 7, 112, 138, 97, 155]);
}
impl ::windows::runtime::RuntimeName for PickerClosingDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingDeferral";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PickerClosingEventArgs(::windows::runtime::IInspectable);
impl PickerClosingEventArgs {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn ClosingOperation(&self) -> ::windows::runtime::Result<PickerClosingOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PickerClosingOperation>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PickerClosingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.PickerClosingEventArgs;{7e59f224-b332-4f12-8b9f-a8c2f06b32cd})");
}
unsafe impl ::windows::runtime::Interface for PickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2119823908, 45874, 20242, [139, 159, 168, 194, 240, 107, 50, 205]);
}
impl ::windows::runtime::RuntimeName for PickerClosingEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingEventArgs";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PickerClosingOperation(::windows::runtime::IInspectable);
impl PickerClosingOperation {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<PickerClosingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PickerClosingDeferral>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Pickers_Provider`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PickerClosingOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.PickerClosingOperation;{4ce9fb84-beee-4e39-a773-fc5f0eae328d})");
}
unsafe impl ::windows::runtime::Interface for PickerClosingOperation {
    type Vtable = IPickerClosingOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290402692, 48878, 20025, [167, 115, 252, 95, 14, 174, 50, 141]);
}
impl ::windows::runtime::RuntimeName for PickerClosingOperation {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingOperation";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SetFileNameResult(pub i32);
impl SetFileNameResult {
    pub const Succeeded: SetFileNameResult = SetFileNameResult(0i32);
    pub const NotAllowed: SetFileNameResult = SetFileNameResult(1i32);
    pub const Unavailable: SetFileNameResult = SetFileNameResult(2i32);
}
impl ::std::convert::From<i32> for SetFileNameResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SetFileNameResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SetFileNameResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.SetFileNameResult;i4)");
}
impl ::windows::runtime::DefaultType for SetFileNameResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct TargetFileRequest(::windows::runtime::IInspectable);
impl TargetFileRequest {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn TargetFile(&self) -> ::windows::runtime::Result<super::super::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn SetTargetFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::IStorageFile>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<TargetFileRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TargetFileRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TargetFileRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.TargetFileRequest;{42bd3355-7f88-478b-8e81-690b20340678})");
}
unsafe impl ::windows::runtime::Interface for TargetFileRequest {
    type Vtable = ITargetFileRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1119695701, 32648, 18315, [142, 129, 105, 11, 32, 52, 6, 120]);
}
impl ::windows::runtime::RuntimeName for TargetFileRequest {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequest";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct TargetFileRequestDeferral(::windows::runtime::IInspectable);
impl TargetFileRequestDeferral {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TargetFileRequestDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.TargetFileRequestDeferral;{4aee9d91-bf15-4da9-95f6-f6b7d558225b})");
}
unsafe impl ::windows::runtime::Interface for TargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1257151889, 48917, 19881, [149, 246, 246, 183, 213, 88, 34, 91]);
}
impl ::windows::runtime::RuntimeName for TargetFileRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestDeferral";
}
#[doc = "*Required features: `Storage_Pickers_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct TargetFileRequestedEventArgs(::windows::runtime::IInspectable);
impl TargetFileRequestedEventArgs {
    #[doc = "*Required features: `Storage_Pickers_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<TargetFileRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TargetFileRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TargetFileRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs;{b163dbc1-1b51-4c89-a591-0fd40b3c57c9})");
}
unsafe impl ::windows::runtime::Interface for TargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2976111553, 6993, 19593, [165, 145, 15, 212, 11, 60, 87, 201]);
}
impl ::windows::runtime::RuntimeName for TargetFileRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs";
}
