#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICoreIncrementalInkStrokeImpl: Sized {
    fn AppendInkPoints(&mut self, inkpoints: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CreateInkStroke(&mut self) -> ::windows::core::Result<super::InkStroke>;
    fn DrawingAttributes(&mut self) -> ::windows::core::Result<super::InkDrawingAttributes>;
    fn PointTransform(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Matrix3x2>;
    fn BoundingRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreIncrementalInkStroke";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICoreIncrementalInkStrokeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreIncrementalInkStrokeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreIncrementalInkStrokeVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreIncrementalInkStroke, BASE_OFFSET>(),
            AppendInkPoints: AppendInkPoints::<Impl, IMPL_OFFSET>,
            CreateInkStroke: CreateInkStroke::<Impl, IMPL_OFFSET>,
            DrawingAttributes: DrawingAttributes::<Impl, IMPL_OFFSET>,
            PointTransform: PointTransform::<Impl, IMPL_OFFSET>,
            BoundingRect: BoundingRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreIncrementalInkStroke as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICoreIncrementalInkStrokeFactoryImpl: Sized {
    fn Create(&mut self, drawingattributes: &::core::option::Option<super::InkDrawingAttributes>, pointtransform: &super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<CoreIncrementalInkStroke>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreIncrementalInkStrokeFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreIncrementalInkStrokeFactory";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICoreIncrementalInkStrokeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreIncrementalInkStrokeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreIncrementalInkStrokeFactoryVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreIncrementalInkStrokeFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreIncrementalInkStrokeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICoreInkIndependentInputSourceImpl: Sized {
    fn PointerEntering(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntering(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerHovering(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerHovering(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExiting(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExiting(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressing(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressing(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoving(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoving(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleasing(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleasing(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerLost(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerLost(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&mut self) -> ::windows::core::Result<super::InkPresenter>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ICoreInkIndependentInputSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkIndependentInputSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInkIndependentInputSourceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInkIndependentInputSource, BASE_OFFSET>(),
            PointerEntering: PointerEntering::<Impl, IMPL_OFFSET>,
            RemovePointerEntering: RemovePointerEntering::<Impl, IMPL_OFFSET>,
            PointerHovering: PointerHovering::<Impl, IMPL_OFFSET>,
            RemovePointerHovering: RemovePointerHovering::<Impl, IMPL_OFFSET>,
            PointerExiting: PointerExiting::<Impl, IMPL_OFFSET>,
            RemovePointerExiting: RemovePointerExiting::<Impl, IMPL_OFFSET>,
            PointerPressing: PointerPressing::<Impl, IMPL_OFFSET>,
            RemovePointerPressing: RemovePointerPressing::<Impl, IMPL_OFFSET>,
            PointerMoving: PointerMoving::<Impl, IMPL_OFFSET>,
            RemovePointerMoving: RemovePointerMoving::<Impl, IMPL_OFFSET>,
            PointerReleasing: PointerReleasing::<Impl, IMPL_OFFSET>,
            RemovePointerReleasing: RemovePointerReleasing::<Impl, IMPL_OFFSET>,
            PointerLost: PointerLost::<Impl, IMPL_OFFSET>,
            RemovePointerLost: RemovePointerLost::<Impl, IMPL_OFFSET>,
            InkPresenter: InkPresenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInkIndependentInputSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICoreInkIndependentInputSource2Impl: Sized {
    fn PointerCursor(&mut self) -> ::windows::core::Result<super::super::super::Core::CoreCursor>;
    fn SetPointerCursor(&mut self, value: &::core::option::Option<super::super::super::Core::CoreCursor>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInkIndependentInputSource2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSource2";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ICoreInkIndependentInputSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkIndependentInputSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInkIndependentInputSource2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInkIndependentInputSource2, BASE_OFFSET>(),
            PointerCursor: PointerCursor::<Impl, IMPL_OFFSET>,
            SetPointerCursor: SetPointerCursor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInkIndependentInputSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSourceStaticsImpl: Sized {
    fn Create(&mut self, inkpresenter: &::core::option::Option<super::InkPresenter>) -> ::windows::core::Result<CoreInkIndependentInputSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreInkIndependentInputSourceStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkIndependentInputSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreInkIndependentInputSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkIndependentInputSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInkIndependentInputSourceStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInkIndependentInputSourceStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInkIndependentInputSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait ICoreInkPresenterHostImpl: Sized {
    fn InkPresenter(&mut self) -> ::windows::core::Result<super::InkPresenter>;
    fn RootVisual(&mut self) -> ::windows::core::Result<super::super::super::Composition::ContainerVisual>;
    fn SetRootVisual(&mut self, value: &::core::option::Option<super::super::super::Composition::ContainerVisual>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreInkPresenterHost";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ICoreInkPresenterHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInkPresenterHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInkPresenterHostVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInkPresenterHost, BASE_OFFSET>(),
            InkPresenter: InkPresenter::<Impl, IMPL_OFFSET>,
            RootVisual: RootVisual::<Impl, IMPL_OFFSET>,
            SetRootVisual: SetRootVisual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInkPresenterHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreWetStrokeUpdateEventArgsImpl: Sized {
    fn NewInkPoints(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>>;
    fn PointerId(&mut self) -> ::windows::core::Result<u32>;
    fn Disposition(&mut self) -> ::windows::core::Result<CoreWetStrokeDisposition>;
    fn SetDisposition(&mut self, value: CoreWetStrokeDisposition) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreWetStrokeUpdateEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWetStrokeUpdateEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWetStrokeUpdateEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWetStrokeUpdateEventArgs, BASE_OFFSET>(),
            NewInkPoints: NewInkPoints::<Impl, IMPL_OFFSET>,
            PointerId: PointerId::<Impl, IMPL_OFFSET>,
            Disposition: Disposition::<Impl, IMPL_OFFSET>,
            SetDisposition: SetDisposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWetStrokeUpdateEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreWetStrokeUpdateSourceImpl: Sized {
    fn WetStrokeStarting(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeStarting(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeContinuing(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeContinuing(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeStopping(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeStopping(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeCompleted(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeCompleted(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeCanceled(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeCanceled(&mut self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&mut self) -> ::windows::core::Result<super::InkPresenter>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreWetStrokeUpdateSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWetStrokeUpdateSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWetStrokeUpdateSourceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWetStrokeUpdateSource, BASE_OFFSET>(),
            WetStrokeStarting: WetStrokeStarting::<Impl, IMPL_OFFSET>,
            RemoveWetStrokeStarting: RemoveWetStrokeStarting::<Impl, IMPL_OFFSET>,
            WetStrokeContinuing: WetStrokeContinuing::<Impl, IMPL_OFFSET>,
            RemoveWetStrokeContinuing: RemoveWetStrokeContinuing::<Impl, IMPL_OFFSET>,
            WetStrokeStopping: WetStrokeStopping::<Impl, IMPL_OFFSET>,
            RemoveWetStrokeStopping: RemoveWetStrokeStopping::<Impl, IMPL_OFFSET>,
            WetStrokeCompleted: WetStrokeCompleted::<Impl, IMPL_OFFSET>,
            RemoveWetStrokeCompleted: RemoveWetStrokeCompleted::<Impl, IMPL_OFFSET>,
            WetStrokeCanceled: WetStrokeCanceled::<Impl, IMPL_OFFSET>,
            RemoveWetStrokeCanceled: RemoveWetStrokeCanceled::<Impl, IMPL_OFFSET>,
            InkPresenter: InkPresenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWetStrokeUpdateSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateSourceStaticsImpl: Sized {
    fn Create(&mut self, inkpresenter: &::core::option::Option<super::InkPresenter>) -> ::windows::core::Result<CoreWetStrokeUpdateSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWetStrokeUpdateSourceStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.ICoreWetStrokeUpdateSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWetStrokeUpdateSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWetStrokeUpdateSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWetStrokeUpdateSourceStaticsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWetStrokeUpdateSourceStatics, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWetStrokeUpdateSourceStatics as ::windows::core::Interface>::IID
    }
}
