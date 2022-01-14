#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreview_Impl: Sized {
    fn GetCurrentPostureAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>;
    fn PostureChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePostureChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITwoPanelHingedDevicePosturePreview {
    const NAME: &'static str = "Windows.System.Preview.ITwoPanelHingedDevicePosturePreview";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ITwoPanelHingedDevicePosturePreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITwoPanelHingedDevicePosturePreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITwoPanelHingedDevicePosturePreview_Vtbl {
        unsafe extern "system" fn GetCurrentPostureAsync<Impl: ITwoPanelHingedDevicePosturePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPostureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostureChanged<Impl: ITwoPanelHingedDevicePosturePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostureChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePostureChanged<Impl: ITwoPanelHingedDevicePosturePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePostureChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITwoPanelHingedDevicePosturePreview, BASE_OFFSET>(),
            GetCurrentPostureAsync: GetCurrentPostureAsync::<Impl, IMPL_OFFSET>,
            PostureChanged: PostureChanged::<Impl, IMPL_OFFSET>,
            RemovePostureChanged: RemovePostureChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITwoPanelHingedDevicePosturePreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sensors", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn HingeState(&mut self) -> ::windows::core::Result<HingeState>;
    fn Panel1Orientation(&mut self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation>;
    fn Panel1Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Panel2Orientation(&mut self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation>;
    fn Panel2Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Sensors", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITwoPanelHingedDevicePosturePreviewReading {
    const NAME: &'static str = "Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading";
}
#[cfg(all(feature = "Devices_Sensors", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ITwoPanelHingedDevicePosturePreviewReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITwoPanelHingedDevicePosturePreviewReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HingeState<Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HingeState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HingeState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel1Orientation<Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Panel1Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel1Id<Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Panel1Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel2Orientation<Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Panel2Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel2Id<Impl: ITwoPanelHingedDevicePosturePreviewReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Panel2Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITwoPanelHingedDevicePosturePreviewReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            HingeState: HingeState::<Impl, IMPL_OFFSET>,
            Panel1Orientation: Panel1Orientation::<Impl, IMPL_OFFSET>,
            Panel1Id: Panel1Id::<Impl, IMPL_OFFSET>,
            Panel2Orientation: Panel2Orientation::<Impl, IMPL_OFFSET>,
            Panel2Id: Panel2Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITwoPanelHingedDevicePosturePreviewReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<TwoPanelHingedDevicePosturePreviewReading>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const NAME: &'static str = "Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewStatics_Impl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITwoPanelHingedDevicePosturePreviewStatics {
    const NAME: &'static str = "Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ITwoPanelHingedDevicePosturePreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITwoPanelHingedDevicePosturePreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITwoPanelHingedDevicePosturePreviewStatics_Vtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: ITwoPanelHingedDevicePosturePreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITwoPanelHingedDevicePosturePreviewStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITwoPanelHingedDevicePosturePreviewStatics as ::windows::core::Interface>::IID
    }
}
