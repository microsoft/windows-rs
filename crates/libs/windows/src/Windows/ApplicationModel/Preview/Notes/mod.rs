#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct INotePlacementChangedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x491d57b7_f780_4e7f_a939_9a4caf965214);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotePlacementChangedPreviewEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INoteVisibilityChangedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e34649e_3815_4ff6_83b3_a14d17120e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoteVisibilityChangedPreviewEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreviewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2ac23e_4850_4f13_9cc7_ff487efdfcde);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
pub struct INotesWindowManagerPreview2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreview2 {
    type Vtable = INotesWindowManagerPreview2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedfe864a_1f54_4b09_9823_ff477f6fa3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewShowNoteOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x886b09d6_a6ae_4007_a56d_1ca70c84c0d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewShowNoteOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INotesWindowManagerPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotesWindowManagerPreviewStatics {
    type Vtable = INotesWindowManagerPreviewStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6668cc88_0a8e_4127_a38e_995445868a78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
#[repr(transparent)]
pub struct NotePlacementChangedPreviewEventArgs(::windows::core::IUnknown);
impl NotePlacementChangedPreviewEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
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
}
unsafe impl ::windows::core::Interface for NotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x491d57b7_f780_4e7f_a939_9a4caf965214);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NotePlacementChangedPreviewEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotePlacementChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NotePlacementChangedPreviewEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
#[repr(transparent)]
pub struct NoteVisibilityChangedPreviewEventArgs(::windows::core::IUnknown);
impl NoteVisibilityChangedPreviewEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
}
unsafe impl ::windows::core::Interface for NoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e34649e_3815_4ff6_83b3_a14d17120e24);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NoteVisibilityChangedPreviewEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NoteVisibilityChangedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for NoteVisibilityChangedPreviewEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreview(::windows::core::IUnknown);
impl NotesWindowManagerPreview {
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn IsScreenLocked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn ShowNote(&self, noteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), noteviewid).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), noteviewid, anchornoteviewid).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacement<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, noteviewid: i32, data: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), noteviewid, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn HideNote(&self, noteviewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), noteviewid).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetNotePlacement(&self, noteviewid: i32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), noteviewid, &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetNoteSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, noteviewid: i32, size: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), noteviewid, size.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn SetFocusToNextView(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetNotesThumbnailAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, thumbnail: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), thumbnail.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemLockStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemLockStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NotePlacementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotePlacementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NoteVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNoteVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn ShowNoteRelativeToWithOptions<'a, Param2: ::windows::core::IntoParam<'a, NotesWindowManagerPreviewShowNoteOptions>>(&self, noteviewid: i32, anchornoteviewid: i32, options: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), noteviewid, anchornoteviewid, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacementWithOptions<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, NotesWindowManagerPreviewShowNoteOptions>>(&self, noteviewid: i32, data: Param1, options: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), noteviewid, data.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn SetFocusToPreviousView(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes', 'Foundation', 'Graphics_Imaging'*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailImageForTaskSwitcherAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn GetForCurrentApp() -> ::windows::core::Result<NotesWindowManagerPreview> {
        Self::INotesWindowManagerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotesWindowManagerPreview>(result__)
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
}
unsafe impl ::windows::core::Interface for NotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreviewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2ac23e_4850_4f13_9cc7_ff487efdfcde);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NotesWindowManagerPreview {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotesWindowManagerPreview {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreview {}
#[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
#[repr(transparent)]
pub struct NotesWindowManagerPreviewShowNoteOptions(::windows::core::IUnknown);
impl NotesWindowManagerPreviewShowNoteOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotesWindowManagerPreviewShowNoteOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn ShowWithFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Preview_Notes'*"]
    pub fn SetShowWithFocus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
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
}
unsafe impl ::windows::core::Interface for NotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x886b09d6_a6ae_4007_a56d_1ca70c84c0d2);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NotesWindowManagerPreviewShowNoteOptions {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotesWindowManagerPreviewShowNoteOptions {}
unsafe impl ::core::marker::Sync for NotesWindowManagerPreviewShowNoteOptions {}
