#[doc(hidden)]
#[repr(transparent)]
pub struct IFileOpenPickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileOpenPickerUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda45a10_f9d4_40c4_8af5_c5b6b5a61d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut AddFileResult) -> ::windows::core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContainsFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanAddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileSelectionMode) -> ::windows::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FileRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FileRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFileRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFileRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IFileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13043da7_7fca_4c2b_9eca_6890f9f00185);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileSavePickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileSavePickerUI {
    type Vtable = IFileSavePickerUI_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileSavePickerUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9656c1e7_3e56_43cc_8a39_33c73d9d542b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TrySetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut SetFileNameResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TargetFileRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IPickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IPickerClosingDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7af7f71e_1a67_4a31_ae80_e907708a619b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPickerClosingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e59f224_b332_4f12_8b9f_a8c2f06b32cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ClosingOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerClosingOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPickerClosingOperation {
    type Vtable = IPickerClosingOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for IPickerClosingOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9fb84_beee_4e39_a773_fc5f0eae328d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerClosingOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITargetFileRequest {
    type Vtable = ITargetFileRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ITargetFileRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42bd3355_7f88_478b_8e81_690b20340678);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TargetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTargetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for ITargetFileRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aee9d91_bf15_4da9_95f6_f6b7d558225b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetFileRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ITargetFileRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb163dbc1_1b51_4c89_a591_0fd40b3c57c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFileRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct FileOpenPickerUI(::windows::core::IUnknown);
impl FileOpenPickerUI {
    pub fn AddFile<P0, E0>(&self, id: &::windows::core::HSTRING, file: P0) -> ::windows::core::Result<AddFileResult>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddFile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveFile(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id)).ok() }
    }
    pub fn ContainsFile(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsFile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanAddFile<P0, E0>(&self, file: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanAddFile)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedFileTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectionMode(&self) -> ::windows::core::Result<FileSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectionMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SettingsIdentifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FileRemoved(&self, handler: &super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileRemoved)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFileRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFileRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closing(&self, handler: &super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closing)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosing(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosing)(::windows::core::Vtable::as_raw(this), token).ok() }
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
unsafe impl ::windows::core::Vtable for FileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_Vtbl;
}
unsafe impl ::windows::core::Interface for FileOpenPickerUI {
    const IID: ::windows::core::GUID = <IFileOpenPickerUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileOpenPickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileOpenPickerUI";
}
::windows::core::interface_hierarchy!(FileOpenPickerUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileRemovedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for FileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileRemovedEventArgs {
    const IID: ::windows::core::GUID = <IFileRemovedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(FileRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct FileSavePickerUI(::windows::core::IUnknown);
impl FileSavePickerUI {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedFileTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SettingsIdentifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrySetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<SetFileNameResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetFileName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileNameChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileNameChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileNameChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFileNameChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetFileRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetFileRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetFileRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTargetFileRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
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
unsafe impl ::windows::core::Vtable for FileSavePickerUI {
    type Vtable = IFileSavePickerUI_Vtbl;
}
unsafe impl ::windows::core::Interface for FileSavePickerUI {
    const IID: ::windows::core::GUID = <IFileSavePickerUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileSavePickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileSavePickerUI";
}
::windows::core::interface_hierarchy!(FileSavePickerUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct PickerClosingDeferral(::windows::core::IUnknown);
impl PickerClosingDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
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
unsafe impl ::windows::core::Vtable for PickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for PickerClosingDeferral {
    const IID: ::windows::core::GUID = <IPickerClosingDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerClosingDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingDeferral";
}
::windows::core::interface_hierarchy!(PickerClosingDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct PickerClosingEventArgs(::windows::core::IUnknown);
impl PickerClosingEventArgs {
    pub fn ClosingOperation(&self) -> ::windows::core::Result<PickerClosingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClosingOperation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PickerClosingEventArgs {
    const IID: ::windows::core::GUID = <IPickerClosingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerClosingEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingEventArgs";
}
::windows::core::interface_hierarchy!(PickerClosingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct PickerClosingOperation(::windows::core::IUnknown);
impl PickerClosingOperation {
    pub fn GetDeferral(&self) -> ::windows::core::Result<PickerClosingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PickerClosingOperation {
    type Vtable = IPickerClosingOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for PickerClosingOperation {
    const IID: ::windows::core::GUID = <IPickerClosingOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PickerClosingOperation {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingOperation";
}
::windows::core::interface_hierarchy!(PickerClosingOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct TargetFileRequest(::windows::core::IUnknown);
impl TargetFileRequest {
    pub fn TargetFile(&self) -> ::windows::core::Result<super::super::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetFile)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTargetFile<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTargetFile)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<TargetFileRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for TargetFileRequest {
    type Vtable = ITargetFileRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for TargetFileRequest {
    const IID: ::windows::core::GUID = <ITargetFileRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TargetFileRequest {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequest";
}
::windows::core::interface_hierarchy!(TargetFileRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct TargetFileRequestDeferral(::windows::core::IUnknown);
impl TargetFileRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
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
unsafe impl ::windows::core::Vtable for TargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for TargetFileRequestDeferral {
    const IID: ::windows::core::GUID = <ITargetFileRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TargetFileRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestDeferral";
}
::windows::core::interface_hierarchy!(TargetFileRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct TargetFileRequestedEventArgs(::windows::core::IUnknown);
impl TargetFileRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<TargetFileRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for TargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for TargetFileRequestedEventArgs {
    const IID: ::windows::core::GUID = <ITargetFileRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TargetFileRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs";
}
::windows::core::interface_hierarchy!(TargetFileRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
