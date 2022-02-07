#[cfg(feature = "deprecated")]
pub trait IMediaEnginePlaybackSource_Impl: Sized {
    fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetPlaybackSource(&self, source: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IMediaEnginePlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaEnginePlaybackSource";
}
#[cfg(feature = "deprecated")]
impl IMediaEnginePlaybackSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>() -> IMediaEnginePlaybackSource_Vtbl {
        unsafe extern "system" fn CurrentItem<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackSource<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlaybackSource(::core::mem::transmute(&source)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEnginePlaybackSource, OFFSET>(),
            CurrentItem: CurrentItem::<Identity, Impl, OFFSET>,
            SetPlaybackSource: SetPlaybackSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEnginePlaybackSource as ::windows::core::Interface>::IID
    }
}
pub trait IMediaPlaybackSource_Impl: Sized {}
impl ::windows::core::RuntimeName for IMediaPlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSource";
}
impl IMediaPlaybackSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSource_Impl, const OFFSET: isize>() -> IMediaPlaybackSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSource, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSource as ::windows::core::Interface>::IID
    }
}
