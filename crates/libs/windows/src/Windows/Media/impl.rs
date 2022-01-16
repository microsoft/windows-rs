#[cfg(feature = "Foundation_Collections")]
pub trait IMediaExtension_Impl: Sized {
    fn SetProperties(&mut self, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaExtension {
    const NAME: &'static str = "Windows.Media.IMediaExtension";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtension_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtension_Vtbl {
        unsafe extern "system" fn SetProperties<Impl: IMediaExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperties(&*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaExtension, BASE_OFFSET>(), SetProperties: SetProperties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IMediaFrame_Impl: Sized + super::Foundation::IClosable_Impl {
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetRelativeTime(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn RelativeTime(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetSystemRelativeTime(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SystemRelativeTime(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetIsDiscontinuous(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscontinuous(&mut self) -> ::windows::core::Result<bool>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IMediaFrame {
    const NAME: &'static str = "Windows.Media.IMediaFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IMediaFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrame_Vtbl {
        unsafe extern "system" fn Type<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemRelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDiscontinuous<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDiscontinuous(value).into()
        }
        unsafe extern "system" fn IsDiscontinuous<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDiscontinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrame, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            SetRelativeTime: SetRelativeTime::<Impl, IMPL_OFFSET>,
            RelativeTime: RelativeTime::<Impl, IMPL_OFFSET>,
            SetSystemRelativeTime: SetSystemRelativeTime::<Impl, IMPL_OFFSET>,
            SystemRelativeTime: SystemRelativeTime::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetIsDiscontinuous: SetIsDiscontinuous::<Impl, IMPL_OFFSET>,
            IsDiscontinuous: IsDiscontinuous::<Impl, IMPL_OFFSET>,
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IMediaMarker_Impl: Sized {
    fn Time(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMediaMarker {
    const NAME: &'static str = "Windows.Media.IMediaMarker";
}
#[cfg(feature = "Foundation")]
impl IMediaMarker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarker_Vtbl {
        unsafe extern "system" fn Time<Impl: IMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Time() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaMarkerType<Impl: IMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaMarkerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaMarker, BASE_OFFSET>(),
            Time: Time::<Impl, IMPL_OFFSET>,
            MediaMarkerType: MediaMarkerType::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaMarkers_Impl: Sized {
    fn Markers(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaMarkers {
    const NAME: &'static str = "Windows.Media.IMediaMarkers";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaMarkers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarkers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarkers_Vtbl {
        unsafe extern "system" fn Markers<Impl: IMediaMarkers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Markers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaMarkers, BASE_OFFSET>(), Markers: Markers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarkers as ::windows::core::Interface>::IID
    }
}
