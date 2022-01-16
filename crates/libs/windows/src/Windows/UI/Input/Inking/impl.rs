#[cfg(feature = "Foundation")]
pub trait IInkPointFactory_Impl: Sized {
    fn CreateInkPoint(&mut self, position: &super::super::super::Foundation::Point, pressure: f32) -> ::windows::core::Result<InkPoint>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInkPointFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPointFactory";
}
#[cfg(feature = "Foundation")]
impl IInkPointFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPointFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPointFactory_Vtbl {
        unsafe extern "system" fn CreateInkPoint<Impl: IInkPointFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInkPoint(&*(&position as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), pressure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPointFactory, BASE_OFFSET>(), CreateInkPoint: CreateInkPoint::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPointFactory as ::windows::core::Interface>::IID
    }
}
pub trait IInkPresenterRulerFactory_Impl: Sized {
    fn Create(&mut self, inkpresenter: &::core::option::Option<InkPresenter>) -> ::windows::core::Result<InkPresenterRuler>;
}
impl ::windows::core::RuntimeName for IInkPresenterRulerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRulerFactory";
}
impl IInkPresenterRulerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterRulerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterRulerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IInkPresenterRulerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&inkpresenter as *const <InkPresenter as ::windows::core::Abi>::Abi as *const <InkPresenter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenterRulerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterRulerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait IInkPresenterStencil_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<InkPresenterStencilKind>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetBackgroundColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetForegroundColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn Transform(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransform(&mut self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::RuntimeName for IInkPresenterStencil {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterStencil";
}
#[cfg(feature = "Foundation_Numerics")]
impl IInkPresenterStencil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterStencil_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterStencil_Vtbl {
        unsafe extern "system" fn Kind<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkPresenterStencilKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsVisible<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVisible(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Transform<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&value as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenterStencil, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            SetIsVisible: SetIsVisible::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            Transform: Transform::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterStencil as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IInkRecognizerContainer_Impl: Sized {
    fn SetDefaultRecognizer(&mut self, recognizer: &::core::option::Option<InkRecognizer>) -> ::windows::core::Result<()>;
    fn RecognizeAsync(&mut self, strokecollection: &::core::option::Option<InkStrokeContainer>, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
    fn GetRecognizers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IInkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognizerContainer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IInkRecognizerContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerContainer_Vtbl {
        unsafe extern "system" fn SetDefaultRecognizer<Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultRecognizer(&*(&recognizer as *const <InkRecognizer as ::windows::core::Abi>::Abi as *const <InkRecognizer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecognizeAsync<Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokecollection: ::windows::core::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizeAsync(&*(&strokecollection as *const <InkStrokeContainer as ::windows::core::Abi>::Abi as *const <InkStrokeContainer as ::windows::core::DefaultType>::DefaultType), recognitiontarget) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecognizers<Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkRecognizerContainer, BASE_OFFSET>(),
            SetDefaultRecognizer: SetDefaultRecognizer::<Impl, IMPL_OFFSET>,
            RecognizeAsync: RecognizeAsync::<Impl, IMPL_OFFSET>,
            GetRecognizers: GetRecognizers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IInkStrokeContainer_Impl: Sized {
    fn BoundingRect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn AddStroke(&mut self, stroke: &::core::option::Option<InkStroke>) -> ::windows::core::Result<()>;
    fn DeleteSelected(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn MoveSelected(&mut self, translation: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithPolyLine(&mut self, polyline: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithLine(&mut self, from: &super::super::super::Foundation::Point, to: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CopySelectedToClipboard(&mut self) -> ::windows::core::Result<()>;
    fn PasteFromClipboard(&mut self, position: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CanPasteFromClipboard(&mut self) -> ::windows::core::Result<bool>;
    fn LoadAsync(&mut self, inputstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>;
    fn SaveAsync(&mut self, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn UpdateRecognitionResults(&mut self, recognitionresults: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>) -> ::windows::core::Result<()>;
    fn GetStrokes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn GetRecognitionResults(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IInkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IInkStrokeContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeContainer_Vtbl {
        unsafe extern "system" fn BoundingRect<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddStroke<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStroke(&*(&stroke as *const <InkStroke as ::windows::core::Abi>::Abi as *const <InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteSelected<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveSelected<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveSelected(&*(&translation as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectWithPolyLine<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, polyline: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectWithPolyLine(&*(&polyline as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectWithLine<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectWithLine(&*(&from as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&to as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopySelectedToClipboard<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySelectedToClipboard().into()
        }
        unsafe extern "system" fn PasteFromClipboard<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasteFromClipboard(&*(&position as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPasteFromClipboard<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPasteFromClipboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAsync(&*(&inputstream as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync(&*(&outputstream as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateRecognitionResults<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateRecognitionResults(&*(&recognitionresults as *const <super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStrokes<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecognitionResults<Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognitionResults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeContainer, BASE_OFFSET>(),
            BoundingRect: BoundingRect::<Impl, IMPL_OFFSET>,
            AddStroke: AddStroke::<Impl, IMPL_OFFSET>,
            DeleteSelected: DeleteSelected::<Impl, IMPL_OFFSET>,
            MoveSelected: MoveSelected::<Impl, IMPL_OFFSET>,
            SelectWithPolyLine: SelectWithPolyLine::<Impl, IMPL_OFFSET>,
            SelectWithLine: SelectWithLine::<Impl, IMPL_OFFSET>,
            CopySelectedToClipboard: CopySelectedToClipboard::<Impl, IMPL_OFFSET>,
            PasteFromClipboard: PasteFromClipboard::<Impl, IMPL_OFFSET>,
            CanPasteFromClipboard: CanPasteFromClipboard::<Impl, IMPL_OFFSET>,
            LoadAsync: LoadAsync::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            UpdateRecognitionResults: UpdateRecognitionResults::<Impl, IMPL_OFFSET>,
            GetStrokes: GetStrokes::<Impl, IMPL_OFFSET>,
            GetRecognitionResults: GetRecognitionResults::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeContainer as ::windows::core::Interface>::IID
    }
}
