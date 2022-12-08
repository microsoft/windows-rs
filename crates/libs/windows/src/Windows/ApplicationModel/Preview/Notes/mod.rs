#[doc(hidden)]
#[repr(transparent)]
pub struct INotePlacementChangedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for INotePlacementChangedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x491d57b7_f780_4e7f_a939_9a4caf965214);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotePlacementChangedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INoteVisibilityChangedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for INoteVisibilityChangedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e34649e_3815_4ff6_83b3_a14d17120e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoteVisibilityChangedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
}
unsafe impl ::windows::core::Interface for INotesWindowManagerPreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2ac23e_4850_4f13_9cc7_ff487efdfcde);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT,
    pub ShowNoteRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacement: usize,
    pub HideNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetNotePlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetNotePlacement: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetNoteSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetNoteSize: usize,
    pub SetFocusToNextView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetNotesThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetNotesThumbnailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SystemLockStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemLockStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NotePlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotePlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NoteVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NoteVisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNoteVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNoteVisibilityChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreview2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotesWindowManagerPreview2 {
    type Vtable = INotesWindowManagerPreview2_Vtbl;
}
unsafe impl ::windows::core::Interface for INotesWindowManagerPreview2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedfe864a_1f54_4b09_9823_ff477f6fa3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowNoteRelativeToWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32, options: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacementWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacementWithOptions: usize,
    pub SetFocusToPreviousView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailImageForTaskSwitcherAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailImageForTaskSwitcherAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewShowNoteOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for INotesWindowManagerPreviewShowNoteOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x886b09d6_a6ae_4007_a56d_1ca70c84c0d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewShowNoteOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowWithFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowWithFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotesWindowManagerPreviewStatics {
    type Vtable = INotesWindowManagerPreviewStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for INotesWindowManagerPreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6668cc88_0a8e_4127_a38e_995445868a78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotePlacementChangedPreviewEventArgs(::windows::core::IUnknown);
impl NotePlacementChangedPreviewEventArgs {
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ViewId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for NotePlacementChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotePlacementChangedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotePlacementChangedPreviewEventArgs {}
impl ::core::fmt::Debug for NotePlacementChangedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotePlacementChangedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotePlacementChangedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs;{491d57b7-f780-4e7f-a939-9a4caf965214})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for NotePlacementChangedPreviewEventArgs {
    const IID: ::windows::core::GUID = <INotePlacementChangedPreviewEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
}
::windows::core::interface_hierarchy!(NotePlacementChangedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NotePlacementChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NotePlacementChangedPreviewEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NoteVisibilityChangedPreviewEventArgs(::windows::core::IUnknown);
impl NoteVisibilityChangedPreviewEventArgs {
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ViewId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for NoteVisibilityChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NoteVisibilityChangedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NoteVisibilityChangedPreviewEventArgs {}
impl ::core::fmt::Debug for NoteVisibilityChangedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NoteVisibilityChangedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NoteVisibilityChangedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs;{0e34649e-3815-4ff6-83b3-a14d17120e24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for NoteVisibilityChangedPreviewEventArgs {
    const IID: ::windows::core::GUID = <INoteVisibilityChangedPreviewEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
}
::windows::core::interface_hierarchy!(NoteVisibilityChangedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NoteVisibilityChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NoteVisibilityChangedPreviewEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreview(::windows::core::IUnknown);
impl NotesWindowManagerPreview {
    pub fn IsScreenLocked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScreenLocked)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShowNote(&self, noteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowNote)(::windows::core::Vtable::as_raw(this), noteviewid).ok() }
    }
    pub fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowNoteRelativeTo)(::windows::core::Vtable::as_raw(this), noteviewid, anchornoteviewid).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacement<P0, E0>(&self, noteviewid: i32, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowNoteWithPlacement)(::windows::core::Vtable::as_raw(this), noteviewid, data.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn HideNote(&self, noteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).HideNote)(::windows::core::Vtable::as_raw(this), noteviewid).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetNotePlacement(&self, noteviewid: i32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNotePlacement)(::windows::core::Vtable::as_raw(this), noteviewid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetNoteSize(&self, noteviewid: i32, size: super::super::super::Foundation::Size) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetNoteSize)(::windows::core::Vtable::as_raw(this), noteviewid, size, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFocusToNextView(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFocusToNextView)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetNotesThumbnailAsync<P0, E0>(&self, thumbnail: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetNotesThumbnailAsync)(::windows::core::Vtable::as_raw(this), thumbnail.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemLockStateChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemLockStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemLockStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSystemLockStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotePlacementChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotePlacementChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotePlacementChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNotePlacementChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NoteVisibilityChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NoteVisibilityChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNoteVisibilityChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNoteVisibilityChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn ShowNoteRelativeToWithOptions(&self, noteviewid: i32, anchornoteviewid: i32, options: &NotesWindowManagerPreviewShowNoteOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ShowNoteRelativeToWithOptions)(::windows::core::Vtable::as_raw(this), noteviewid, anchornoteviewid, ::core::mem::transmute_copy(options)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacementWithOptions<P0, E0>(&self, noteviewid: i32, data: P0, options: &NotesWindowManagerPreviewShowNoteOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ShowNoteWithPlacementWithOptions)(::windows::core::Vtable::as_raw(this), noteviewid, data.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(options)).ok() }
    }
    pub fn SetFocusToPreviousView(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetFocusToPreviousView)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailImageForTaskSwitcherAsync(&self, bitmap: &super::super::super::Graphics::Imaging::SoftwareBitmap) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetThumbnailImageForTaskSwitcherAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetForCurrentApp() -> ::windows::core::Result<NotesWindowManagerPreview> {
        Self::INotesWindowManagerPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentApp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INotesWindowManagerPreviewStatics<R, F: FnOnce(&INotesWindowManagerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NotesWindowManagerPreview, INotesWindowManagerPreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for NotesWindowManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotesWindowManagerPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotesWindowManagerPreview {}
impl ::core::fmt::Debug for NotesWindowManagerPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotesWindowManagerPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotesWindowManagerPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview;{dc2ac23e-4850-4f13-9cc7-ff487efdfcde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
}
unsafe impl ::windows::core::Interface for NotesWindowManagerPreview {
    const IID: ::windows::core::GUID = <INotesWindowManagerPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
}
::windows::core::interface_hierarchy!(NotesWindowManagerPreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NotesWindowManagerPreview {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreview {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreviewShowNoteOptions(::windows::core::IUnknown);
impl NotesWindowManagerPreviewShowNoteOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NotesWindowManagerPreviewShowNoteOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ShowWithFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowWithFocus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetShowWithFocus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShowWithFocus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for NotesWindowManagerPreviewShowNoteOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotesWindowManagerPreviewShowNoteOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotesWindowManagerPreviewShowNoteOptions {}
impl ::core::fmt::Debug for NotesWindowManagerPreviewShowNoteOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotesWindowManagerPreviewShowNoteOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotesWindowManagerPreviewShowNoteOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions;{886b09d6-a6ae-4007-a56d-1ca70c84c0d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for NotesWindowManagerPreviewShowNoteOptions {
    const IID: ::windows::core::GUID = <INotesWindowManagerPreviewShowNoteOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
}
::windows::core::interface_hierarchy!(NotesWindowManagerPreviewShowNoteOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NotesWindowManagerPreviewShowNoteOptions {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreviewShowNoteOptions {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
