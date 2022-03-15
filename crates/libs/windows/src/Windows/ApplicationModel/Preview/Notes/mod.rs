#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct INotePlacementChangedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x491d57b7_f780_4e7f_a939_9a4caf965214);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotePlacementChangedPreviewEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INoteVisibilityChangedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e34649e_3815_4ff6_83b3_a14d17120e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoteVisibilityChangedPreviewEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2ac23e_4850_4f13_9cc7_ff487efdfcde);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT,
    pub ShowNoteRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacement: usize,
    pub HideNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetNotePlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetNotePlacement: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetNoteSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetNoteSize: usize,
    pub SetFocusToNextView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetNotesThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetNotesThumbnailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SystemLockStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemLockStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NotePlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotePlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NoteVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for INotesWindowManagerPreview2 {
    type Vtable = INotesWindowManagerPreview2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedfe864a_1f54_4b09_9823_ff477f6fa3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShowNoteRelativeToWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacementWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacementWithOptions: usize,
    pub SetFocusToPreviousView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailImageForTaskSwitcherAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailImageForTaskSwitcherAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewShowNoteOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x886b09d6_a6ae_4007_a56d_1ca70c84c0d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewShowNoteOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShowWithFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowWithFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreviewStatics {
    type Vtable = INotesWindowManagerPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6668cc88_0a8e_4127_a38e_995445868a78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotePlacementChangedPreviewEventArgs(::windows::core::IUnknown);
impl NotePlacementChangedPreviewEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
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
unsafe impl ::windows::core::Interface for NotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <INotePlacementChangedPreviewEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
}
impl ::core::convert::From<NotePlacementChangedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: NotePlacementChangedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotePlacementChangedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NotePlacementChangedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotePlacementChangedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: NotePlacementChangedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotePlacementChangedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NotePlacementChangedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotePlacementChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NotePlacementChangedPreviewEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NoteVisibilityChangedPreviewEventArgs(::windows::core::IUnknown);
impl NoteVisibilityChangedPreviewEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::Interface for NoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <INoteVisibilityChangedPreviewEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
}
impl ::core::convert::From<NoteVisibilityChangedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: NoteVisibilityChangedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NoteVisibilityChangedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NoteVisibilityChangedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NoteVisibilityChangedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: NoteVisibilityChangedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NoteVisibilityChangedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NoteVisibilityChangedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NoteVisibilityChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NoteVisibilityChangedPreviewEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreview(::windows::core::IUnknown);
impl NotesWindowManagerPreview {
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn IsScreenLocked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsScreenLocked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn ShowNote(&self, noteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowNote)(::core::mem::transmute_copy(this), noteviewid).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowNoteRelativeTo)(::core::mem::transmute_copy(this), noteviewid, anchornoteviewid).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacement<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, noteviewid: i32, data: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowNoteWithPlacement)(::core::mem::transmute_copy(this), noteviewid, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn HideNote(&self, noteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).HideNote)(::core::mem::transmute_copy(this), noteviewid).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetNotePlacement(&self, noteviewid: i32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNotePlacement)(::core::mem::transmute_copy(this), noteviewid, &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetNoteSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, noteviewid: i32, size: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetNoteSize)(::core::mem::transmute_copy(this), noteviewid, size.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn SetFocusToNextView(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFocusToNextView)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetNotesThumbnailAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, thumbnail: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetNotesThumbnailAsync)(::core::mem::transmute_copy(this), thumbnail.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemLockStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemLockStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemLockStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSystemLockStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotePlacementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotePlacementChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotePlacementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNotePlacementChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NoteVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NoteVisibilityChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNoteVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNoteVisibilityChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn ShowNoteRelativeToWithOptions<'a, Param2: ::windows::core::IntoParam<'a, NotesWindowManagerPreviewShowNoteOptions>>(&self, noteviewid: i32, anchornoteviewid: i32, options: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ShowNoteRelativeToWithOptions)(::core::mem::transmute_copy(this), noteviewid, anchornoteviewid, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacementWithOptions<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, NotesWindowManagerPreviewShowNoteOptions>>(&self, noteviewid: i32, data: Param1, options: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ShowNoteWithPlacementWithOptions)(::core::mem::transmute_copy(this), noteviewid, data.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn SetFocusToPreviousView(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFocusToPreviousView)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailImageForTaskSwitcherAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetThumbnailImageForTaskSwitcherAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn GetForCurrentApp() -> ::windows::core::Result<NotesWindowManagerPreview> {
        Self::INotesWindowManagerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentApp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotesWindowManagerPreview>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INotesWindowManagerPreviewStatics<R, F: FnOnce(&INotesWindowManagerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotesWindowManagerPreview, INotesWindowManagerPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for NotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
    const IID: ::windows::core::GUID = <INotesWindowManagerPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
}
impl ::core::convert::From<NotesWindowManagerPreview> for ::windows::core::IUnknown {
    fn from(value: NotesWindowManagerPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotesWindowManagerPreview> for ::windows::core::IUnknown {
    fn from(value: &NotesWindowManagerPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotesWindowManagerPreview> for ::windows::core::IInspectable {
    fn from(value: NotesWindowManagerPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotesWindowManagerPreview> for ::windows::core::IInspectable {
    fn from(value: &NotesWindowManagerPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotesWindowManagerPreview {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreview {}
#[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreviewShowNoteOptions(::windows::core::IUnknown);
impl NotesWindowManagerPreviewShowNoteOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotesWindowManagerPreviewShowNoteOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn ShowWithFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowWithFocus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_Notes\"`*"]
    pub fn SetShowWithFocus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShowWithFocus)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for NotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
    const IID: ::windows::core::GUID = <INotesWindowManagerPreviewShowNoteOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
}
impl ::core::convert::From<NotesWindowManagerPreviewShowNoteOptions> for ::windows::core::IUnknown {
    fn from(value: NotesWindowManagerPreviewShowNoteOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotesWindowManagerPreviewShowNoteOptions> for ::windows::core::IUnknown {
    fn from(value: &NotesWindowManagerPreviewShowNoteOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotesWindowManagerPreviewShowNoteOptions> for ::windows::core::IInspectable {
    fn from(value: NotesWindowManagerPreviewShowNoteOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotesWindowManagerPreviewShowNoteOptions> for ::windows::core::IInspectable {
    fn from(value: &NotesWindowManagerPreviewShowNoteOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotesWindowManagerPreviewShowNoteOptions {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreviewShowNoteOptions {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
