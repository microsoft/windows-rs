#[cfg(feature = "implement_exclusive")]
pub trait INotePlacementChangedPreviewEventArgs_Impl: Sized {
    fn ViewId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INotePlacementChangedPreviewEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotePlacementChangedPreviewEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotePlacementChangedPreviewEventArgs_Vtbl {
        unsafe extern "system" fn ViewId<Impl: INotePlacementChangedPreviewEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotePlacementChangedPreviewEventArgs, BASE_OFFSET>(),
            ViewId: ViewId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotePlacementChangedPreviewEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INoteVisibilityChangedPreviewEventArgs_Impl: Sized {
    fn ViewId(&mut self) -> ::windows::core::Result<i32>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INoteVisibilityChangedPreviewEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INoteVisibilityChangedPreviewEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INoteVisibilityChangedPreviewEventArgs_Vtbl {
        unsafe extern "system" fn ViewId<Impl: INoteVisibilityChangedPreviewEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: INoteVisibilityChangedPreviewEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INoteVisibilityChangedPreviewEventArgs, BASE_OFFSET>(),
            ViewId: ViewId::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INoteVisibilityChangedPreviewEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait INotesWindowManagerPreview_Impl: Sized {
    fn IsScreenLocked(&mut self) -> ::windows::core::Result<bool>;
    fn ShowNote(&mut self, noteviewid: i32) -> ::windows::core::Result<()>;
    fn ShowNoteRelativeTo(&mut self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::Result<()>;
    fn ShowNoteWithPlacement(&mut self, noteviewid: i32, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn HideNote(&mut self, noteviewid: i32) -> ::windows::core::Result<()>;
    fn GetNotePlacement(&mut self, noteviewid: i32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn TrySetNoteSize(&mut self, noteviewid: i32, size: &super::super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn SetFocusToNextView(&mut self) -> ::windows::core::Result<()>;
    fn SetNotesThumbnailAsync(&mut self, thumbnail: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn SystemLockStateChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemLockStateChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotePlacementChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotePlacementChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NoteVisibilityChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNoteVisibilityChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl INotesWindowManagerPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotesWindowManagerPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotesWindowManagerPreview_Vtbl {
        unsafe extern "system" fn IsScreenLocked<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScreenLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowNote<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowNote(noteviewid).into()
        }
        unsafe extern "system" fn ShowNoteRelativeTo<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowNoteRelativeTo(noteviewid, anchornoteviewid).into()
        }
        unsafe extern "system" fn ShowNoteWithPlacement<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowNoteWithPlacement(noteviewid, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HideNote<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HideNote(noteviewid).into()
        }
        unsafe extern "system" fn GetNotePlacement<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotePlacement(noteviewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetNoteSize<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetNoteSize(noteviewid, &*(&size as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusToNextView<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusToNextView().into()
        }
        unsafe extern "system" fn SetNotesThumbnailAsync<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotesThumbnailAsync(&*(&thumbnail as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemLockStateChanged<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemLockStateChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemLockStateChanged<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemLockStateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotePlacementChanged<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotePlacementChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNotePlacementChanged<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNotePlacementChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NoteVisibilityChanged<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoteVisibilityChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNoteVisibilityChanged<Impl: INotesWindowManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNoteVisibilityChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotesWindowManagerPreview, BASE_OFFSET>(),
            IsScreenLocked: IsScreenLocked::<Impl, IMPL_OFFSET>,
            ShowNote: ShowNote::<Impl, IMPL_OFFSET>,
            ShowNoteRelativeTo: ShowNoteRelativeTo::<Impl, IMPL_OFFSET>,
            ShowNoteWithPlacement: ShowNoteWithPlacement::<Impl, IMPL_OFFSET>,
            HideNote: HideNote::<Impl, IMPL_OFFSET>,
            GetNotePlacement: GetNotePlacement::<Impl, IMPL_OFFSET>,
            TrySetNoteSize: TrySetNoteSize::<Impl, IMPL_OFFSET>,
            SetFocusToNextView: SetFocusToNextView::<Impl, IMPL_OFFSET>,
            SetNotesThumbnailAsync: SetNotesThumbnailAsync::<Impl, IMPL_OFFSET>,
            SystemLockStateChanged: SystemLockStateChanged::<Impl, IMPL_OFFSET>,
            RemoveSystemLockStateChanged: RemoveSystemLockStateChanged::<Impl, IMPL_OFFSET>,
            NotePlacementChanged: NotePlacementChanged::<Impl, IMPL_OFFSET>,
            RemoveNotePlacementChanged: RemoveNotePlacementChanged::<Impl, IMPL_OFFSET>,
            NoteVisibilityChanged: NoteVisibilityChanged::<Impl, IMPL_OFFSET>,
            RemoveNoteVisibilityChanged: RemoveNoteVisibilityChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotesWindowManagerPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait INotesWindowManagerPreview2_Impl: Sized {
    fn ShowNoteRelativeToWithOptions(&mut self, noteviewid: i32, anchornoteviewid: i32, options: &::core::option::Option<NotesWindowManagerPreviewShowNoteOptions>) -> ::windows::core::Result<()>;
    fn ShowNoteWithPlacementWithOptions(&mut self, noteviewid: i32, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, options: &::core::option::Option<NotesWindowManagerPreviewShowNoteOptions>) -> ::windows::core::Result<()>;
    fn SetFocusToPreviousView(&mut self) -> ::windows::core::Result<()>;
    fn SetThumbnailImageForTaskSwitcherAsync(&mut self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotesWindowManagerPreview2 {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl INotesWindowManagerPreview2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotesWindowManagerPreview2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotesWindowManagerPreview2_Vtbl {
        unsafe extern "system" fn ShowNoteRelativeToWithOptions<Impl: INotesWindowManagerPreview2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowNoteRelativeToWithOptions(noteviewid, anchornoteviewid, &*(&options as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::Abi>::Abi as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowNoteWithPlacementWithOptions<Impl: INotesWindowManagerPreview2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowNoteWithPlacementWithOptions(noteviewid, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::Abi>::Abi as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFocusToPreviousView<Impl: INotesWindowManagerPreview2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusToPreviousView().into()
        }
        unsafe extern "system" fn SetThumbnailImageForTaskSwitcherAsync<Impl: INotesWindowManagerPreview2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetThumbnailImageForTaskSwitcherAsync(&*(&bitmap as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotesWindowManagerPreview2, BASE_OFFSET>(),
            ShowNoteRelativeToWithOptions: ShowNoteRelativeToWithOptions::<Impl, IMPL_OFFSET>,
            ShowNoteWithPlacementWithOptions: ShowNoteWithPlacementWithOptions::<Impl, IMPL_OFFSET>,
            SetFocusToPreviousView: SetFocusToPreviousView::<Impl, IMPL_OFFSET>,
            SetThumbnailImageForTaskSwitcherAsync: SetThumbnailImageForTaskSwitcherAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotesWindowManagerPreview2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewShowNoteOptions_Impl: Sized {
    fn ShowWithFocus(&mut self) -> ::windows::core::Result<bool>;
    fn SetShowWithFocus(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions";
}
#[cfg(feature = "implement_exclusive")]
impl INotesWindowManagerPreviewShowNoteOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotesWindowManagerPreviewShowNoteOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotesWindowManagerPreviewShowNoteOptions_Vtbl {
        unsafe extern "system" fn ShowWithFocus<Impl: INotesWindowManagerPreviewShowNoteOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowWithFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowWithFocus<Impl: INotesWindowManagerPreviewShowNoteOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowWithFocus(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotesWindowManagerPreviewShowNoteOptions, BASE_OFFSET>(),
            ShowWithFocus: ShowWithFocus::<Impl, IMPL_OFFSET>,
            SetShowWithFocus: SetShowWithFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotesWindowManagerPreviewShowNoteOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewStatics_Impl: Sized {
    fn GetForCurrentApp(&mut self) -> ::windows::core::Result<NotesWindowManagerPreview>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotesWindowManagerPreviewStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INotesWindowManagerPreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotesWindowManagerPreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotesWindowManagerPreviewStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentApp<Impl: INotesWindowManagerPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotesWindowManagerPreviewStatics, BASE_OFFSET>(),
            GetForCurrentApp: GetForCurrentApp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotesWindowManagerPreviewStatics as ::windows::core::Interface>::IID
    }
}
