#[cfg(feature = "Foundation")]
pub trait IMediaCue_Impl: Sized {
    fn SetStartTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMediaCue {
    const NAME: &'static str = "Windows.Media.Core.IMediaCue";
}
#[cfg(feature = "Foundation")]
impl IMediaCue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>() -> IMediaCue_Vtbl {
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartTime(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDuration(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Duration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Duration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCue, OFFSET>(),
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCue as ::windows::core::Interface>::IID
    }
}
pub trait IMediaSource_Impl: Sized {}
impl ::windows::core::RuntimeName for IMediaSource {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource";
}
impl IMediaSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaSource_Impl, const OFFSET: isize>() -> IMediaSource_Vtbl {
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSource, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSource as ::windows::core::Interface>::IID
    }
}
pub trait IMediaStreamDescriptor_Impl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor";
}
impl IMediaStreamDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: isize>() -> IMediaStreamDescriptor_Vtbl {
        unsafe extern "system" fn IsSelected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguage(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamDescriptor, OFFSET>(),
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamDescriptor as ::windows::core::Interface>::IID
    }
}
pub trait IMediaStreamDescriptor2_Impl: Sized + IMediaStreamDescriptor_Impl {
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor2";
}
impl IMediaStreamDescriptor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor2_Impl, const OFFSET: isize>() -> IMediaStreamDescriptor2_Vtbl {
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaStreamDescriptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamDescriptor2, OFFSET>(),
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamDescriptor2 as ::windows::core::Interface>::IID
    }
}
pub trait IMediaTrack_Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaTrack {
    const NAME: &'static str = "Windows.Media.Core.IMediaTrack";
}
impl IMediaTrack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: isize>() -> IMediaTrack_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrackKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTrack, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            TrackKind: TrackKind::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISingleSelectMediaTrackList_Impl: Sized {
    fn SelectedIndexChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedIndexChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISingleSelectMediaTrackList {
    const NAME: &'static str = "Windows.Media.Core.ISingleSelectMediaTrackList";
}
#[cfg(feature = "Foundation")]
impl ISingleSelectMediaTrackList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>() -> ISingleSelectMediaTrackList_Vtbl {
        unsafe extern "system" fn SelectedIndexChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectedIndexChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectedIndexChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSelectedIndexChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn SetSelectedIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ISingleSelectMediaTrackList, OFFSET>(),
            SelectedIndexChanged: SelectedIndexChanged::<Identity, Impl, OFFSET>,
            RemoveSelectedIndexChanged: RemoveSelectedIndexChanged::<Identity, Impl, OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Identity, Impl, OFFSET>,
            SelectedIndex: SelectedIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISingleSelectMediaTrackList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ITimedMetadataTrackProvider_Impl: Sized {
    fn TimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackProvider {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl ITimedMetadataTrackProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimedMetadataTrackProvider_Impl, const OFFSET: isize>() -> ITimedMetadataTrackProvider_Vtbl {
        unsafe extern "system" fn TimedMetadataTracks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimedMetadataTrackProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrackProvider, OFFSET>(),
            TimedMetadataTracks: TimedMetadataTracks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrackProvider as ::windows::core::Interface>::IID
    }
}
