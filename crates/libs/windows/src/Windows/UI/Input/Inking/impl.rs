#[doc = "*Required features: `\"UI_Input_Inking\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IInkPointFactory_Impl: Sized {
    fn CreateInkPoint(&self, position: &super::super::super::Foundation::Point, pressure: f32) -> ::windows_core::Result<InkPoint>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IInkPointFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPointFactory";
}
#[cfg(feature = "Foundation")]
impl IInkPointFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPointFactory_Impl, const OFFSET: isize>() -> IInkPointFactory_Vtbl {
        unsafe extern "system" fn CreateInkPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPointFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInkPoint(::core::mem::transmute(&position), pressure) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IInkPointFactory, OFFSET>(), CreateInkPoint: CreateInkPoint::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInkPointFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`, `\"implement\"`*"]
pub trait IInkPresenterRulerFactory_Impl: Sized {
    fn Create(&self, inkpresenter: ::core::option::Option<&InkPresenter>) -> ::windows_core::Result<InkPresenterRuler>;
}
impl ::windows_core::RuntimeName for IInkPresenterRulerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRulerFactory";
}
impl IInkPresenterRulerFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterRulerFactory_Impl, const OFFSET: isize>() -> IInkPresenterRulerFactory_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterRulerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::windows_core::from_raw_borrowed(&inkpresenter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IInkPresenterRulerFactory, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInkPresenterRulerFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`, `\"Foundation_Numerics\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub trait IInkPresenterStencil_Impl: Sized {
    fn Kind(&self) -> ::windows_core::Result<InkPresenterStencilKind>;
    fn IsVisible(&self) -> ::windows_core::Result<bool>;
    fn SetIsVisible(&self, value: bool) -> ::windows_core::Result<()>;
    fn BackgroundColor(&self) -> ::windows_core::Result<super::super::Color>;
    fn SetBackgroundColor(&self, value: &super::super::Color) -> ::windows_core::Result<()>;
    fn ForegroundColor(&self) -> ::windows_core::Result<super::super::Color>;
    fn SetForegroundColor(&self, value: &super::super::Color) -> ::windows_core::Result<()>;
    fn Transform(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::RuntimeName for IInkPresenterStencil {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterStencil";
}
#[cfg(feature = "Foundation_Numerics")]
impl IInkPresenterStencil_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>() -> IInkPresenterStencil_Vtbl {
        unsafe extern "system" fn Kind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkPresenterStencilKind) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsVisible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsVisible(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackgroundColor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ForegroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForegroundColor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Transform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Transform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransform(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IInkPresenterStencil, OFFSET>(),
            Kind: Kind::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
            SetIsVisible: SetIsVisible::<Identity, Impl, OFFSET>,
            BackgroundColor: BackgroundColor::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, Impl, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, Impl, OFFSET>,
            Transform: Transform::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInkPresenterStencil as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IInkRecognizerContainer_Impl: Sized {
    fn SetDefaultRecognizer(&self, recognizer: ::core::option::Option<&InkRecognizer>) -> ::windows_core::Result<()>;
    fn RecognizeAsync(&self, strokecollection: ::core::option::Option<&InkStrokeContainer>, recognitiontarget: InkRecognitionTarget) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
    fn GetRecognizers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IInkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognizerContainer";
}
#[cfg(feature = "Foundation_Collections")]
impl IInkRecognizerContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>() -> IInkRecognizerContainer_Vtbl {
        unsafe extern "system" fn SetDefaultRecognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultRecognizer(::windows_core::from_raw_borrowed(&recognizer)).into()
        }
        unsafe extern "system" fn RecognizeAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokecollection: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecognizeAsync(::windows_core::from_raw_borrowed(&strokecollection), recognitiontarget) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecognizers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecognizers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IInkRecognizerContainer, OFFSET>(),
            SetDefaultRecognizer: SetDefaultRecognizer::<Identity, Impl, OFFSET>,
            RecognizeAsync: RecognizeAsync::<Identity, Impl, OFFSET>,
            GetRecognizers: GetRecognizers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInkRecognizerContainer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IInkStrokeContainer_Impl: Sized {
    fn BoundingRect(&self) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn AddStroke(&self, stroke: ::core::option::Option<&InkStroke>) -> ::windows_core::Result<()>;
    fn DeleteSelected(&self) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn MoveSelected(&self, translation: &super::super::super::Foundation::Point) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithPolyLine(&self, polyline: ::core::option::Option<&super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithLine(&self, from: &super::super::super::Foundation::Point, to: &super::super::super::Foundation::Point) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn CopySelectedToClipboard(&self) -> ::windows_core::Result<()>;
    fn PasteFromClipboard(&self, position: &super::super::super::Foundation::Point) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn CanPasteFromClipboard(&self) -> ::windows_core::Result<bool>;
    fn LoadAsync(&self, inputstream: ::core::option::Option<&super::super::super::Storage::Streams::IInputStream>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>;
    fn SaveAsync(&self, outputstream: ::core::option::Option<&super::super::super::Storage::Streams::IOutputStream>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn UpdateRecognitionResults(&self, recognitionresults: ::core::option::Option<&super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>) -> ::windows_core::Result<()>;
    fn GetStrokes(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn GetRecognitionResults(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IInkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IInkStrokeContainer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>() -> IInkStrokeContainer_Vtbl {
        unsafe extern "system" fn BoundingRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStroke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStroke(::windows_core::from_raw_borrowed(&stroke)).into()
        }
        unsafe extern "system" fn DeleteSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeleteSelected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveSelected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveSelected(::core::mem::transmute(&translation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectWithPolyLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, polyline: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectWithPolyLine(::windows_core::from_raw_borrowed(&polyline)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectWithLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectWithLine(::core::mem::transmute(&from), ::core::mem::transmute(&to)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopySelectedToClipboard<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopySelectedToClipboard().into()
        }
        unsafe extern "system" fn PasteFromClipboard<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PasteFromClipboard(::core::mem::transmute(&position)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPasteFromClipboard<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanPasteFromClipboard() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadAsync(::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveAsync(::windows_core::from_raw_borrowed(&outputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateRecognitionResults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresults: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateRecognitionResults(::windows_core::from_raw_borrowed(&recognitionresults)).into()
        }
        unsafe extern "system" fn GetStrokes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecognitionResults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecognitionResults() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IInkStrokeContainer, OFFSET>(),
            BoundingRect: BoundingRect::<Identity, Impl, OFFSET>,
            AddStroke: AddStroke::<Identity, Impl, OFFSET>,
            DeleteSelected: DeleteSelected::<Identity, Impl, OFFSET>,
            MoveSelected: MoveSelected::<Identity, Impl, OFFSET>,
            SelectWithPolyLine: SelectWithPolyLine::<Identity, Impl, OFFSET>,
            SelectWithLine: SelectWithLine::<Identity, Impl, OFFSET>,
            CopySelectedToClipboard: CopySelectedToClipboard::<Identity, Impl, OFFSET>,
            PasteFromClipboard: PasteFromClipboard::<Identity, Impl, OFFSET>,
            CanPasteFromClipboard: CanPasteFromClipboard::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            SaveAsync: SaveAsync::<Identity, Impl, OFFSET>,
            UpdateRecognitionResults: UpdateRecognitionResults::<Identity, Impl, OFFSET>,
            GetStrokes: GetStrokes::<Identity, Impl, OFFSET>,
            GetRecognitionResults: GetRecognitionResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInkStrokeContainer as ::windows_core::ComInterface>::IID
    }
}
