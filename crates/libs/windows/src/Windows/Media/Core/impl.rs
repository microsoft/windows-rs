pub trait IMediaCue_Impl: Sized + windows_core::IUnknownImpl {
    fn SetStartTime(&self, value: &super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
    fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IMediaCue {
    const NAME: &'static str = "Windows.Media.Core.IMediaCue";
}
impl IMediaCue_Vtbl {
    pub const fn new<Identity: IMediaCue_Impl, const OFFSET: isize>() -> IMediaCue_Vtbl {
        unsafe extern "system" fn SetStartTime<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaCue_Impl::SetStartTime(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn StartTime<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaCue_Impl::StartTime(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaCue_Impl::SetDuration(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Duration<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaCue_Impl::Duration(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaCue_Impl::SetId(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Id<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaCue_Impl::Id(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaCue, OFFSET>(),
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaCue as windows_core::Interface>::IID
    }
}
pub trait IMediaSource_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IMediaSource {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource";
}
impl IMediaSource_Vtbl {
    pub const fn new<Identity: IMediaSource_Impl, const OFFSET: isize>() -> IMediaSource_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaSource as windows_core::Interface>::IID
    }
}
pub trait IMediaStreamDescriptor_Impl: Sized + windows_core::IUnknownImpl {
    fn IsSelected(&self) -> windows_core::Result<bool>;
    fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IMediaStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor";
}
impl IMediaStreamDescriptor_Vtbl {
    pub const fn new<Identity: IMediaStreamDescriptor_Impl, const OFFSET: isize>() -> IMediaStreamDescriptor_Vtbl {
        unsafe extern "system" fn IsSelected<Identity: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaStreamDescriptor_Impl::IsSelected(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaStreamDescriptor_Impl::SetName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Name<Identity: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaStreamDescriptor_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaStreamDescriptor_Impl::SetLanguage(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Language<Identity: IMediaStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaStreamDescriptor_Impl::Language(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaStreamDescriptor, OFFSET>(),
            IsSelected: IsSelected::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaStreamDescriptor as windows_core::Interface>::IID
    }
}
pub trait IMediaStreamDescriptor2_Impl: Sized + windows_core::IUnknownImpl + IMediaStreamDescriptor_Impl {
    fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IMediaStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor2";
}
impl IMediaStreamDescriptor2_Vtbl {
    pub const fn new<Identity: IMediaStreamDescriptor2_Impl, const OFFSET: isize>() -> IMediaStreamDescriptor2_Vtbl {
        unsafe extern "system" fn SetLabel<Identity: IMediaStreamDescriptor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaStreamDescriptor2_Impl::SetLabel(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Label<Identity: IMediaStreamDescriptor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaStreamDescriptor2_Impl::Label(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaStreamDescriptor2, OFFSET>(),
            SetLabel: SetLabel::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaStreamDescriptor2 as windows_core::Interface>::IID
    }
}
pub trait IMediaTrack_Impl: Sized + windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn TrackKind(&self) -> windows_core::Result<MediaTrackKind>;
    fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IMediaTrack {
    const NAME: &'static str = "Windows.Media.Core.IMediaTrack";
}
impl IMediaTrack_Vtbl {
    pub const fn new<Identity: IMediaTrack_Impl, const OFFSET: isize>() -> IMediaTrack_Vtbl {
        unsafe extern "system" fn Id<Identity: IMediaTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaTrack_Impl::Id(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: IMediaTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaTrack_Impl::Language(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackKind<Identity: IMediaTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MediaTrackKind) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaTrack_Impl::TrackKind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMediaTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaTrack_Impl::SetLabel(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Label<Identity: IMediaTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaTrack_Impl::Label(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaTrack, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            TrackKind: TrackKind::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaTrack as windows_core::Interface>::IID
    }
}
pub trait ISingleSelectMediaTrackList_Impl: Sized + windows_core::IUnknownImpl {
    fn SelectedIndexChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedIndexChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()>;
    fn SelectedIndex(&self) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for ISingleSelectMediaTrackList {
    const NAME: &'static str = "Windows.Media.Core.ISingleSelectMediaTrackList";
}
impl ISingleSelectMediaTrackList_Vtbl {
    pub const fn new<Identity: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>() -> ISingleSelectMediaTrackList_Vtbl {
        unsafe extern "system" fn SelectedIndexChanged<Identity: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISingleSelectMediaTrackList_Impl::SelectedIndexChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectedIndexChanged<Identity: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISingleSelectMediaTrackList_Impl::RemoveSelectedIndexChanged(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn SetSelectedIndex<Identity: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISingleSelectMediaTrackList_Impl::SetSelectedIndex(this, value).into()
        }
        unsafe extern "system" fn SelectedIndex<Identity: ISingleSelectMediaTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISingleSelectMediaTrackList_Impl::SelectedIndex(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISingleSelectMediaTrackList, OFFSET>(),
            SelectedIndexChanged: SelectedIndexChanged::<Identity, OFFSET>,
            RemoveSelectedIndexChanged: RemoveSelectedIndexChanged::<Identity, OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Identity, OFFSET>,
            SelectedIndex: SelectedIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISingleSelectMediaTrackList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ITimedMetadataTrackProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn TimedMetadataTracks(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ITimedMetadataTrackProvider {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl ITimedMetadataTrackProvider_Vtbl {
    pub const fn new<Identity: ITimedMetadataTrackProvider_Impl, const OFFSET: isize>() -> ITimedMetadataTrackProvider_Vtbl {
        unsafe extern "system" fn TimedMetadataTracks<Identity: ITimedMetadataTrackProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITimedMetadataTrackProvider_Impl::TimedMetadataTracks(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITimedMetadataTrackProvider, OFFSET>(),
            TimedMetadataTracks: TimedMetadataTracks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimedMetadataTrackProvider as windows_core::Interface>::IID
    }
}
