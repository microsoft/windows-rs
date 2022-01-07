#[cfg(feature = "implement_exclusive")]
pub trait INotePlacementChangedPreviewEventArgsImpl: Sized {
    fn ViewId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotePlacementChangedPreviewEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INotePlacementChangedPreviewEventArgsVtbl {
    pub const fn new<Impl: INotePlacementChangedPreviewEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotePlacementChangedPreviewEventArgsVtbl {
        unsafe extern "system" fn ViewId<Impl: INotePlacementChangedPreviewEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotePlacementChangedPreviewEventArgs>, base.5, ViewId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INoteVisibilityChangedPreviewEventArgsImpl: Sized {
    fn ViewId(&self) -> ::windows::core::Result<i32>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INoteVisibilityChangedPreviewEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INoteVisibilityChangedPreviewEventArgsVtbl {
    pub const fn new<Impl: INoteVisibilityChangedPreviewEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INoteVisibilityChangedPreviewEventArgsVtbl {
        unsafe extern "system" fn ViewId<Impl: INoteVisibilityChangedPreviewEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: INoteVisibilityChangedPreviewEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INoteVisibilityChangedPreviewEventArgs>, base.5, ViewId::<Impl, OFFSET>, IsVisible::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewImpl: Sized {
    fn IsScreenLocked(&self) -> ::windows::core::Result<bool>;
    fn ShowNote(&self, noteviewid: i32) -> ::windows::core::Result<()>;
    fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::Result<()>;
    fn ShowNoteWithPlacement(&self, noteviewid: i32, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn HideNote(&self, noteviewid: i32) -> ::windows::core::Result<()>;
    fn GetNotePlacement(&self, noteviewid: i32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn TrySetNoteSize(&self, noteviewid: i32, size: &super::super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn SetFocusToNextView(&self) -> ::windows::core::Result<()>;
    fn SetNotesThumbnailAsync(&self, thumbnail: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn SystemLockStateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemLockStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotePlacementChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotePlacementChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NoteVisibilityChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNoteVisibilityChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview";
}
#[cfg(feature = "implement_exclusive")]
impl INotesWindowManagerPreviewVtbl {
    pub const fn new<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotesWindowManagerPreviewVtbl {
        unsafe extern "system" fn IsScreenLocked<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScreenLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowNote<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowNote(noteviewid).into()
        }
        unsafe extern "system" fn ShowNoteRelativeTo<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowNoteRelativeTo(noteviewid, anchornoteviewid).into()
        }
        unsafe extern "system" fn ShowNoteWithPlacement<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowNoteWithPlacement(noteviewid, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HideNote<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).HideNote(noteviewid).into()
        }
        unsafe extern "system" fn GetNotePlacement<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNotePlacement(noteviewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetNoteSize<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetNoteSize(noteviewid, &*(&size as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusToNextView<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusToNextView().into()
        }
        unsafe extern "system" fn SetNotesThumbnailAsync<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotesThumbnailAsync(&*(&thumbnail as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemLockStateChanged<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemLockStateChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemLockStateChanged<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSystemLockStateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotePlacementChanged<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotePlacementChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNotePlacementChanged<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNotePlacementChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NoteVisibilityChanged<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NoteVisibilityChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNoteVisibilityChanged<Impl: INotesWindowManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNoteVisibilityChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<INotesWindowManagerPreview>,
            base.5,
            IsScreenLocked::<Impl, OFFSET>,
            ShowNote::<Impl, OFFSET>,
            ShowNoteRelativeTo::<Impl, OFFSET>,
            ShowNoteWithPlacement::<Impl, OFFSET>,
            HideNote::<Impl, OFFSET>,
            GetNotePlacement::<Impl, OFFSET>,
            TrySetNoteSize::<Impl, OFFSET>,
            SetFocusToNextView::<Impl, OFFSET>,
            SetNotesThumbnailAsync::<Impl, OFFSET>,
            SystemLockStateChanged::<Impl, OFFSET>,
            RemoveSystemLockStateChanged::<Impl, OFFSET>,
            NotePlacementChanged::<Impl, OFFSET>,
            RemoveNotePlacementChanged::<Impl, OFFSET>,
            NoteVisibilityChanged::<Impl, OFFSET>,
            RemoveNoteVisibilityChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreview2Impl: Sized {
    fn ShowNoteRelativeToWithOptions(&self, noteviewid: i32, anchornoteviewid: i32, options: &::core::option::Option<NotesWindowManagerPreviewShowNoteOptions>) -> ::windows::core::Result<()>;
    fn ShowNoteWithPlacementWithOptions(&self, noteviewid: i32, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, options: &::core::option::Option<NotesWindowManagerPreviewShowNoteOptions>) -> ::windows::core::Result<()>;
    fn SetFocusToPreviousView(&self) -> ::windows::core::Result<()>;
    fn SetThumbnailImageForTaskSwitcherAsync(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotesWindowManagerPreview2 {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreview2";
}
#[cfg(feature = "implement_exclusive")]
impl INotesWindowManagerPreview2Vtbl {
    pub const fn new<Impl: INotesWindowManagerPreview2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotesWindowManagerPreview2Vtbl {
        unsafe extern "system" fn ShowNoteRelativeToWithOptions<Impl: INotesWindowManagerPreview2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32, anchornoteviewid: i32, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowNoteRelativeToWithOptions(noteviewid, anchornoteviewid, &*(&options as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::Abi>::Abi as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowNoteWithPlacementWithOptions<Impl: INotesWindowManagerPreview2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noteviewid: i32, data: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowNoteWithPlacementWithOptions(noteviewid, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::Abi>::Abi as *const <NotesWindowManagerPreviewShowNoteOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFocusToPreviousView<Impl: INotesWindowManagerPreview2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusToPreviousView().into()
        }
        unsafe extern "system" fn SetThumbnailImageForTaskSwitcherAsync<Impl: INotesWindowManagerPreview2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetThumbnailImageForTaskSwitcherAsync(&*(&bitmap as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotesWindowManagerPreview2>, base.5, ShowNoteRelativeToWithOptions::<Impl, OFFSET>, ShowNoteWithPlacementWithOptions::<Impl, OFFSET>, SetFocusToPreviousView::<Impl, OFFSET>, SetThumbnailImageForTaskSwitcherAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewShowNoteOptionsImpl: Sized {
    fn ShowWithFocus(&self) -> ::windows::core::Result<bool>;
    fn SetShowWithFocus(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewShowNoteOptions";
}
#[cfg(feature = "implement_exclusive")]
impl INotesWindowManagerPreviewShowNoteOptionsVtbl {
    pub const fn new<Impl: INotesWindowManagerPreviewShowNoteOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotesWindowManagerPreviewShowNoteOptionsVtbl {
        unsafe extern "system" fn ShowWithFocus<Impl: INotesWindowManagerPreviewShowNoteOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowWithFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowWithFocus<Impl: INotesWindowManagerPreviewShowNoteOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShowWithFocus(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotesWindowManagerPreviewShowNoteOptions>, base.5, ShowWithFocus::<Impl, OFFSET>, SetShowWithFocus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotesWindowManagerPreviewStaticsImpl: Sized {
    fn GetForCurrentApp(&self) -> ::windows::core::Result<NotesWindowManagerPreview>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotesWindowManagerPreviewStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.INotesWindowManagerPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INotesWindowManagerPreviewStaticsVtbl {
    pub const fn new<Impl: INotesWindowManagerPreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotesWindowManagerPreviewStaticsVtbl {
        unsafe extern "system" fn GetForCurrentApp<Impl: INotesWindowManagerPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotesWindowManagerPreviewStatics>, base.5, GetForCurrentApp::<Impl, OFFSET>)
    }
}
