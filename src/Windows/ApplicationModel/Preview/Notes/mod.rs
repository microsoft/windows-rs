#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INotePlacementChangedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1226659767, 63360, 20095, [169, 57, 154, 76, 175, 150, 82, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotePlacementChangedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INoteVisibilityChangedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238314654, 14357, 20470, [131, 179, 161, 77, 23, 18, 14, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoteVisibilityChangedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3693789758, 18512, 20243, [156, 199, 255, 72, 126, 253, 252, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32, anchornoteviewid: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thumbnail: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotesWindowManagerPreview2 {
    type Vtable = INotesWindowManagerPreview2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3992880714, 8020, 19209, [152, 35, 255, 71, 127, 111, 163, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreview2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32, anchornoteviewid: i32, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, noteviewid: i32, data: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewShowNoteOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2288716246, 42670, 16391, [165, 109, 28, 167, 12, 132, 192, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewShowNoteOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotesWindowManagerPreviewStatics {
    type Vtable = INotesWindowManagerPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1718144136, 2702, 16679, [163, 142, 153, 84, 69, 134, 138, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotesWindowManagerPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NotePlacementChangedPreviewEventArgs(::windows::runtime::IInspectable);
impl NotePlacementChangedPreviewEventArgs {
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn ViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotePlacementChangedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs;{491d57b7-f780-4e7f-a939-9a4caf965214})");
}
unsafe impl ::windows::runtime::Interface for NotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1226659767, 63360, 20095, [169, 57, 154, 76, 175, 150, 82, 20]);
}
impl ::windows::runtime::RuntimeName for NotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
}
impl ::std::convert::From<NotePlacementChangedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NotePlacementChangedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotePlacementChangedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NotePlacementChangedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NotePlacementChangedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NotePlacementChangedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotePlacementChangedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NotePlacementChangedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NotePlacementChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotePlacementChangedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for NotePlacementChangedPreviewEventArgs {}
#[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NoteVisibilityChangedPreviewEventArgs(::windows::runtime::IInspectable);
impl NoteVisibilityChangedPreviewEventArgs {
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn ViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NoteVisibilityChangedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs;{0e34649e-3815-4ff6-83b3-a14d17120e24})");
}
unsafe impl ::windows::runtime::Interface for NoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238314654, 14357, 20470, [131, 179, 161, 77, 23, 18, 14, 36]);
}
impl ::windows::runtime::RuntimeName for NoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
}
impl ::std::convert::From<NoteVisibilityChangedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NoteVisibilityChangedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NoteVisibilityChangedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NoteVisibilityChangedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NoteVisibilityChangedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NoteVisibilityChangedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NoteVisibilityChangedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NoteVisibilityChangedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NoteVisibilityChangedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NoteVisibilityChangedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for NoteVisibilityChangedPreviewEventArgs {}
#[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NotesWindowManagerPreview(::windows::runtime::IInspectable);
impl NotesWindowManagerPreview {
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn IsScreenLocked(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn ShowNote(&self, noteviewid: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), noteviewid).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), noteviewid, anchornoteviewid).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Storage_Streams`*"]
    pub fn ShowNoteWithPlacement<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, noteviewid: i32, data: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), noteviewid, data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn HideNote(&self, noteviewid: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), noteviewid).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Storage_Streams`*"]
    pub fn GetNotePlacement(&self, noteviewid: i32) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), noteviewid, &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn TrySetNoteSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Size>>(&self, noteviewid: i32, size: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), noteviewid, size.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn SetFocusToNextView(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`, `Storage_Streams`*"]
    pub fn SetNotesThumbnailAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, thumbnail: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), thumbnail.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn SystemLockStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn RemoveSystemLockStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn NotePlacementChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn RemoveNotePlacementChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn NoteVisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`*"]
    pub fn RemoveNoteVisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn GetForCurrentApp() -> ::windows::runtime::Result<NotesWindowManagerPreview> {
        Self::INotesWindowManagerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NotesWindowManagerPreview>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn ShowNoteRelativeToWithOptions<'a, Param2: ::windows::runtime::IntoParam<'a, NotesWindowManagerPreviewShowNoteOptions>>(&self, noteviewid: i32, anchornoteviewid: i32, options: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), noteviewid, anchornoteviewid, options.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Storage_Streams`*"]
    pub fn ShowNoteWithPlacementWithOptions<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, NotesWindowManagerPreviewShowNoteOptions>>(&self, noteviewid: i32, data: Param1, options: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), noteviewid, data.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn SetFocusToPreviousView(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`, `Foundation`, `Graphics_Imaging`*"]
    pub fn SetThumbnailImageForTaskSwitcherAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn INotesWindowManagerPreviewStatics<R, F: FnOnce(&INotesWindowManagerPreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NotesWindowManagerPreview, INotesWindowManagerPreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotesWindowManagerPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview;{dc2ac23e-4850-4f13-9cc7-ff487efdfcde})");
}
unsafe impl ::windows::runtime::Interface for NotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3693789758, 18512, 20243, [156, 199, 255, 72, 126, 253, 252, 222]);
}
impl ::windows::runtime::RuntimeName for NotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
}
impl ::std::convert::From<NotesWindowManagerPreview> for ::windows::runtime::IUnknown {
    fn from(value: NotesWindowManagerPreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotesWindowManagerPreview> for ::windows::runtime::IUnknown {
    fn from(value: &NotesWindowManagerPreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NotesWindowManagerPreview> for ::windows::runtime::IInspectable {
    fn from(value: NotesWindowManagerPreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotesWindowManagerPreview> for ::windows::runtime::IInspectable {
    fn from(value: &NotesWindowManagerPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NotesWindowManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotesWindowManagerPreview {}
unsafe impl ::std::marker::Sync for NotesWindowManagerPreview {}
#[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NotesWindowManagerPreviewShowNoteOptions(::windows::runtime::IInspectable);
impl NotesWindowManagerPreviewShowNoteOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NotesWindowManagerPreviewShowNoteOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn ShowWithFocus(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_Notes`*"]
    pub fn SetShowWithFocus(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotesWindowManagerPreviewShowNoteOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions;{886b09d6-a6ae-4007-a56d-1ca70c84c0d2})");
}
unsafe impl ::windows::runtime::Interface for NotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2288716246, 42670, 16391, [165, 109, 28, 167, 12, 132, 192, 210]);
}
impl ::windows::runtime::RuntimeName for NotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
}
impl ::std::convert::From<NotesWindowManagerPreviewShowNoteOptions> for ::windows::runtime::IUnknown {
    fn from(value: NotesWindowManagerPreviewShowNoteOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotesWindowManagerPreviewShowNoteOptions> for ::windows::runtime::IUnknown {
    fn from(value: &NotesWindowManagerPreviewShowNoteOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NotesWindowManagerPreviewShowNoteOptions> for ::windows::runtime::IInspectable {
    fn from(value: NotesWindowManagerPreviewShowNoteOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotesWindowManagerPreviewShowNoteOptions> for ::windows::runtime::IInspectable {
    fn from(value: &NotesWindowManagerPreviewShowNoteOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NotesWindowManagerPreviewShowNoteOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotesWindowManagerPreviewShowNoteOptions {}
unsafe impl ::std::marker::Sync for NotesWindowManagerPreviewShowNoteOptions {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct PreviewNotesContract(pub u8);
