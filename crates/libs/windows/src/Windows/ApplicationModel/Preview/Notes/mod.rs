#[doc(hidden)]
#[repr(transparent)]
pub struct INotePlacementChangedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for INotePlacementChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INotePlacementChangedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x491d57b7_f780_4e7f_a939_9a4caf965214);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotePlacementChangedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INoteVisibilityChangedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for INoteVisibilityChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INoteVisibilityChangedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e34649e_3815_4ff6_83b3_a14d17120e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoteVisibilityChangedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
}
impl ::core::clone::Clone for INotesWindowManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INotesWindowManagerPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc2ac23e_4850_4f13_9cc7_ff487efdfcde);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShowNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows_core::HRESULT,
    pub ShowNoteRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacement: usize,
    pub HideNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetNotePlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetNotePlacement: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetNoteSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetNoteSize: usize,
    pub SetFocusToNextView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetNotesThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetNotesThumbnailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SystemLockStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemLockStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NotePlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotePlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NoteVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NoteVisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNoteVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNoteVisibilityChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreview2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotesWindowManagerPreview2 {
    type Vtable = INotesWindowManagerPreview2_Vtbl;
}
impl ::core::clone::Clone for INotesWindowManagerPreview2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INotesWindowManagerPreview2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedfe864a_1f54_4b09_9823_ff477f6fa3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowNoteRelativeToWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacementWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacementWithOptions: usize,
    pub SetFocusToPreviousView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailImageForTaskSwitcherAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailImageForTaskSwitcherAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewShowNoteOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
}
impl ::core::clone::Clone for INotesWindowManagerPreviewShowNoteOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INotesWindowManagerPreviewShowNoteOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x886b09d6_a6ae_4007_a56d_1ca70c84c0d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewShowNoteOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowWithFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShowWithFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotesWindowManagerPreviewStatics {
    type Vtable = INotesWindowManagerPreviewStatics_Vtbl;
}
impl ::core::clone::Clone for INotesWindowManagerPreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INotesWindowManagerPreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6668cc88_0a8e_4127_a38e_995445868a78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotePlacementChangedPreviewEventArgs(::windows_core::IUnknown);
impl NotePlacementChangedPreviewEventArgs {
    pub fn ViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for NotePlacementChangedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs;{491d57b7-f780-4e7f-a939-9a4caf965214})");
}
impl ::core::clone::Clone for NotePlacementChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for NotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotePlacementChangedPreviewEventArgs {
    const IID: ::windows_core::GUID = <INotePlacementChangedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(NotePlacementChangedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NotePlacementChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NotePlacementChangedPreviewEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NoteVisibilityChangedPreviewEventArgs(::windows_core::IUnknown);
impl NoteVisibilityChangedPreviewEventArgs {
    pub fn ViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for NoteVisibilityChangedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs;{0e34649e-3815-4ff6-83b3-a14d17120e24})");
}
impl ::core::clone::Clone for NoteVisibilityChangedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for NoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NoteVisibilityChangedPreviewEventArgs {
    const IID: ::windows_core::GUID = <INoteVisibilityChangedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(NoteVisibilityChangedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NoteVisibilityChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NoteVisibilityChangedPreviewEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreview(::windows_core::IUnknown);
impl NotesWindowManagerPreview {
    pub fn IsScreenLocked(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsScreenLocked)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShowNote(&self, noteviewid: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowNote)(::windows_core::Interface::as_raw(this), noteviewid).ok() }
    }
    pub fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowNoteRelativeTo)(::windows_core::Interface::as_raw(this), noteviewid, anchornoteviewid).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacement<P0>(&self, noteviewid: i32, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowNoteWithPlacement)(::windows_core::Interface::as_raw(this), noteviewid, data.try_into_param()?.abi()).ok() }
    }
    pub fn HideNote(&self, noteviewid: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).HideNote)(::windows_core::Interface::as_raw(this), noteviewid).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetNotePlacement(&self, noteviewid: i32) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNotePlacement)(::windows_core::Interface::as_raw(this), noteviewid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetNoteSize(&self, noteviewid: i32, size: super::super::super::Foundation::Size) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetNoteSize)(::windows_core::Interface::as_raw(this), noteviewid, size, &mut result__).from_abi(result__)
        }
    }
    pub fn SetFocusToNextView(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusToNextView)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetNotesThumbnailAsync<P0>(&self, thumbnail: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetNotesThumbnailAsync)(::windows_core::Interface::as_raw(this), thumbnail.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemLockStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemLockStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemLockStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSystemLockStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotePlacementChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotePlacementChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotePlacementChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNotePlacementChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NoteVisibilityChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NoteVisibilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNoteVisibilityChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNoteVisibilityChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ShowNoteRelativeToWithOptions<P0>(&self, noteviewid: i32, anchornoteviewid: i32, options: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<NotesWindowManagerPreviewShowNoteOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ShowNoteRelativeToWithOptions)(::windows_core::Interface::as_raw(this), noteviewid, anchornoteviewid, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacementWithOptions<P0, P1>(&self, noteviewid: i32, data: P0, options: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::IntoParam<NotesWindowManagerPreviewShowNoteOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ShowNoteWithPlacementWithOptions)(::windows_core::Interface::as_raw(this), noteviewid, data.try_into_param()?.abi(), options.into_param().abi()).ok() }
    }
    pub fn SetFocusToPreviousView(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusToPreviousView)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailImageForTaskSwitcherAsync<P0>(&self, bitmap: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = &::windows_core::ComInterface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetThumbnailImageForTaskSwitcherAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetForCurrentApp() -> ::windows_core::Result<NotesWindowManagerPreview> {
        Self::INotesWindowManagerPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentApp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INotesWindowManagerPreviewStatics<R, F: FnOnce(&INotesWindowManagerPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NotesWindowManagerPreview, INotesWindowManagerPreviewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for NotesWindowManagerPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview;{dc2ac23e-4850-4f13-9cc7-ff487efdfcde})");
}
impl ::core::clone::Clone for NotesWindowManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for NotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotesWindowManagerPreview {
    const IID: ::windows_core::GUID = <INotesWindowManagerPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
}
::windows_core::imp::interface_hierarchy!(NotesWindowManagerPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NotesWindowManagerPreview {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreview {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreviewShowNoteOptions(::windows_core::IUnknown);
impl NotesWindowManagerPreviewShowNoteOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NotesWindowManagerPreviewShowNoteOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ShowWithFocus(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowWithFocus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowWithFocus(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowWithFocus)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for NotesWindowManagerPreviewShowNoteOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions;{886b09d6-a6ae-4007-a56d-1ca70c84c0d2})");
}
impl ::core::clone::Clone for NotesWindowManagerPreviewShowNoteOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for NotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotesWindowManagerPreviewShowNoteOptions {
    const IID: ::windows_core::GUID = <INotesWindowManagerPreviewShowNoteOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
}
::windows_core::imp::interface_hierarchy!(NotesWindowManagerPreviewShowNoteOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NotesWindowManagerPreviewShowNoteOptions {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreviewShowNoteOptions {}
