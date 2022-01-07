#[cfg(feature = "implement_exclusive")]
pub trait ICoreIncrementalInkStrokeImpl: Sized {
    fn AppendInkPoints(&self, inkpoints: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CreateInkStroke(&self) -> ::windows::core::Result<super::InkStroke>;
    fn DrawingAttributes(&self) -> ::windows::core::Result<super::InkDrawingAttributes>;
    fn PointTransform(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Matrix3x2>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreIncrementalInkStrokeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreIncrementalInkStrokeImpl, const OFFSET: isize>() -> ICoreIncrementalInkStrokeVtbl {
        unsafe extern "system" fn AppendInkPoints<Impl: ICoreIncrementalInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendInkPoints(&*(&inkpoints as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkPoint> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkPoint> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInkStroke<Impl: ICoreIncrementalInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInkStroke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Impl: ICoreIncrementalInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointTransform<Impl: ICoreIncrementalInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRect<Impl: ICoreIncrementalInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreIncrementalInkStroke>, ::windows::core::GetTrustLevel, AppendInkPoints::<Impl, OFFSET>, CreateInkStroke::<Impl, OFFSET>, DrawingAttributes::<Impl, OFFSET>, PointTransform::<Impl, OFFSET>, BoundingRect::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreIncrementalInkStrokeFactoryImpl: Sized {
    fn Create(&self, drawingattributes: &::core::option::Option<super::InkDrawingAttributes>, pointtransform: &super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<CoreIncrementalInkStroke>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreIncrementalInkStrokeFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreIncrementalInkStrokeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreIncrementalInkStrokeFactoryImpl, const OFFSET: isize>() -> ICoreIncrementalInkStrokeFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICoreIncrementalInkStrokeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr, pointtransform: super::super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&drawingattributes as *const <super::InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <super::InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType), &*(&pointtransform as *const <super::super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreIncrementalInkStrokeFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSourceImpl: Sized {
    fn PointerEntering(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntering(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerHovering(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerHovering(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExiting(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExiting(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressing(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoving(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoving(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleasing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleasing(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerLost(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerLost(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInkIndependentInputSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>() -> ICoreInkIndependentInputSourceVtbl {
        unsafe extern "system" fn PointerEntering<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerEntering(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntering<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerEntering(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerHovering<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerHovering(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerHovering<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerHovering(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExiting<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerExiting(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExiting<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerExiting(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressing<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPressing(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressing<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerPressing(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoving<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerMoving(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoving<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerMoving(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleasing<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerReleasing(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleasing<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerReleasing(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerLost<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerLost(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerLost<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerLost(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InkPresenter<Impl: ICoreInkIndependentInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkPresenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICoreInkIndependentInputSource>,
            ::windows::core::GetTrustLevel,
            PointerEntering::<Impl, OFFSET>,
            RemovePointerEntering::<Impl, OFFSET>,
            PointerHovering::<Impl, OFFSET>,
            RemovePointerHovering::<Impl, OFFSET>,
            PointerExiting::<Impl, OFFSET>,
            RemovePointerExiting::<Impl, OFFSET>,
            PointerPressing::<Impl, OFFSET>,
            RemovePointerPressing::<Impl, OFFSET>,
            PointerMoving::<Impl, OFFSET>,
            RemovePointerMoving::<Impl, OFFSET>,
            PointerReleasing::<Impl, OFFSET>,
            RemovePointerReleasing::<Impl, OFFSET>,
            PointerLost::<Impl, OFFSET>,
            RemovePointerLost::<Impl, OFFSET>,
            InkPresenter::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSource2Impl: Sized {
    fn PointerCursor(&self) -> ::windows::core::Result<super::super::super::Core::CoreCursor>;
    fn SetPointerCursor(&self, value: &::core::option::Option<super::super::super::Core::CoreCursor>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInkIndependentInputSource2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInkIndependentInputSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkIndependentInputSource2Impl, const OFFSET: isize>() -> ICoreInkIndependentInputSource2Vtbl {
        unsafe extern "system" fn PointerCursor<Impl: ICoreInkIndependentInputSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Impl: ICoreInkIndependentInputSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerCursor(&*(&value as *const <super::super::super::Core::CoreCursor as ::windows::core::Abi>::Abi as *const <super::super::super::Core::CoreCursor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreInkIndependentInputSource2>, ::windows::core::GetTrustLevel, PointerCursor::<Impl, OFFSET>, SetPointerCursor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSourceStaticsImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<super::InkPresenter>) -> ::windows::core::Result<CoreInkIndependentInputSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInkIndependentInputSourceStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInkIndependentInputSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkIndependentInputSourceStaticsImpl, const OFFSET: isize>() -> ICoreInkIndependentInputSourceStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ICoreInkIndependentInputSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&inkpresenter as *const <super::InkPresenter as ::windows::core::Abi>::Abi as *const <super::InkPresenter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreInkIndependentInputSourceStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkPresenterHostImpl: Sized {
    fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter>;
    fn RootVisual(&self) -> ::windows::core::Result<super::super::super::Composition::ContainerVisual>;
    fn SetRootVisual(&self, value: &::core::option::Option<super::super::super::Composition::ContainerVisual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkPresenterHost";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInkPresenterHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkPresenterHostImpl, const OFFSET: isize>() -> ICoreInkPresenterHostVtbl {
        unsafe extern "system" fn InkPresenter<Impl: ICoreInkPresenterHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkPresenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootVisual<Impl: ICoreInkPresenterHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootVisual<Impl: ICoreInkPresenterHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootVisual(&*(&value as *const <super::super::super::Composition::ContainerVisual as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::ContainerVisual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreInkPresenterHost>, ::windows::core::GetTrustLevel, InkPresenter::<Impl, OFFSET>, RootVisual::<Impl, OFFSET>, SetRootVisual::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateEventArgsImpl: Sized {
    fn NewInkPoints(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>>;
    fn PointerId(&self) -> ::windows::core::Result<u32>;
    fn Disposition(&self) -> ::windows::core::Result<CoreWetStrokeDisposition>;
    fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWetStrokeUpdateEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWetStrokeUpdateEventArgsImpl, const OFFSET: isize>() -> ICoreWetStrokeUpdateEventArgsVtbl {
        unsafe extern "system" fn NewInkPoints<Impl: ICoreWetStrokeUpdateEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewInkPoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerId<Impl: ICoreWetStrokeUpdateEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disposition<Impl: ICoreWetStrokeUpdateEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreWetStrokeDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisposition<Impl: ICoreWetStrokeUpdateEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreWetStrokeDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisposition(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreWetStrokeUpdateEventArgs>, ::windows::core::GetTrustLevel, NewInkPoints::<Impl, OFFSET>, PointerId::<Impl, OFFSET>, Disposition::<Impl, OFFSET>, SetDisposition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateSourceImpl: Sized {
    fn WetStrokeStarting(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeStarting(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeContinuing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeContinuing(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeStopping(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeStopping(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeCompleted(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeCompleted(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeCanceled(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeCanceled(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWetStrokeUpdateSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>() -> ICoreWetStrokeUpdateSourceVtbl {
        unsafe extern "system" fn WetStrokeStarting<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WetStrokeStarting(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWetStrokeStarting<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWetStrokeStarting(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WetStrokeContinuing<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WetStrokeContinuing(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWetStrokeContinuing<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWetStrokeContinuing(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WetStrokeStopping<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WetStrokeStopping(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWetStrokeStopping<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWetStrokeStopping(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WetStrokeCompleted<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WetStrokeCompleted(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWetStrokeCompleted<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWetStrokeCompleted(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WetStrokeCanceled<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WetStrokeCanceled(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWetStrokeCanceled<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWetStrokeCanceled(&*(&cookie as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InkPresenter<Impl: ICoreWetStrokeUpdateSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkPresenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICoreWetStrokeUpdateSource>,
            ::windows::core::GetTrustLevel,
            WetStrokeStarting::<Impl, OFFSET>,
            RemoveWetStrokeStarting::<Impl, OFFSET>,
            WetStrokeContinuing::<Impl, OFFSET>,
            RemoveWetStrokeContinuing::<Impl, OFFSET>,
            WetStrokeStopping::<Impl, OFFSET>,
            RemoveWetStrokeStopping::<Impl, OFFSET>,
            WetStrokeCompleted::<Impl, OFFSET>,
            RemoveWetStrokeCompleted::<Impl, OFFSET>,
            WetStrokeCanceled::<Impl, OFFSET>,
            RemoveWetStrokeCanceled::<Impl, OFFSET>,
            InkPresenter::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateSourceStaticsImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<super::InkPresenter>) -> ::windows::core::Result<CoreWetStrokeUpdateSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWetStrokeUpdateSourceStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWetStrokeUpdateSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWetStrokeUpdateSourceStaticsImpl, const OFFSET: isize>() -> ICoreWetStrokeUpdateSourceStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ICoreWetStrokeUpdateSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&inkpresenter as *const <super::InkPresenter as ::windows::core::Abi>::Abi as *const <super::InkPresenter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreWetStrokeUpdateSourceStatics>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
