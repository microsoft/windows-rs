#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkDrawingAttributes_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn PenTip(&mut self) -> ::windows::core::Result<PenTipShape>;
    fn SetPenTip(&mut self, value: PenTipShape) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn SetSize(&mut self, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn IgnorePressure(&mut self) -> ::windows::core::Result<bool>;
    fn SetIgnorePressure(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FitToCurve(&mut self) -> ::windows::core::Result<bool>;
    fn SetFitToCurve(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkDrawingAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes_Vtbl {
        unsafe extern "system" fn Color<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PenTip<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PenTipShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenTip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenTip<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PenTipShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenTip(value).into()
        }
        unsafe extern "system" fn Size<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnorePressure<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnorePressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnorePressure<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnorePressure(value).into()
        }
        unsafe extern "system" fn FitToCurve<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FitToCurve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFitToCurve<Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFitToCurve(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributes, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            PenTip: PenTip::<Impl, IMPL_OFFSET>,
            SetPenTip: SetPenTip::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            IgnorePressure: IgnorePressure::<Impl, IMPL_OFFSET>,
            SetIgnorePressure: SetIgnorePressure::<Impl, IMPL_OFFSET>,
            FitToCurve: FitToCurve::<Impl, IMPL_OFFSET>,
            SetFitToCurve: SetFitToCurve::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInkDrawingAttributes2_Impl: Sized {
    fn PenTipTransform(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetPenTipTransform(&mut self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn DrawAsHighlighter(&mut self) -> ::windows::core::Result<bool>;
    fn SetDrawAsHighlighter(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkDrawingAttributes2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkDrawingAttributes2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes2_Vtbl {
        unsafe extern "system" fn PenTipTransform<Impl: IInkDrawingAttributes2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenTipTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenTipTransform<Impl: IInkDrawingAttributes2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenTipTransform(&*(&value as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawAsHighlighter<Impl: IInkDrawingAttributes2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawAsHighlighter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDrawAsHighlighter<Impl: IInkDrawingAttributes2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDrawAsHighlighter(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributes2, BASE_OFFSET>(),
            PenTipTransform: PenTipTransform::<Impl, IMPL_OFFSET>,
            SetPenTipTransform: SetPenTipTransform::<Impl, IMPL_OFFSET>,
            DrawAsHighlighter: DrawAsHighlighter::<Impl, IMPL_OFFSET>,
            SetDrawAsHighlighter: SetDrawAsHighlighter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes3_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<InkDrawingAttributesKind>;
    fn PencilProperties(&mut self) -> ::windows::core::Result<InkDrawingAttributesPencilProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes3";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributes3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes3_Vtbl {
        unsafe extern "system" fn Kind<Impl: IInkDrawingAttributes3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkDrawingAttributesKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PencilProperties<Impl: IInkDrawingAttributes3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PencilProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributes3, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            PencilProperties: PencilProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes4_Impl: Sized {
    fn IgnoreTilt(&mut self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTilt(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes4 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes4";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributes4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes4_Vtbl {
        unsafe extern "system" fn IgnoreTilt<Impl: IInkDrawingAttributes4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnoreTilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreTilt<Impl: IInkDrawingAttributes4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreTilt(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributes4, BASE_OFFSET>(),
            IgnoreTilt: IgnoreTilt::<Impl, IMPL_OFFSET>,
            SetIgnoreTilt: SetIgnoreTilt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes5_Impl: Sized {
    fn ModelerAttributes(&mut self) -> ::windows::core::Result<InkModelerAttributes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes5 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes5";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributes5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes5_Vtbl {
        unsafe extern "system" fn ModelerAttributes<Impl: IInkDrawingAttributes5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelerAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributes5, BASE_OFFSET>(),
            ModelerAttributes: ModelerAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributesPencilProperties_Impl: Sized {
    fn Opacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributesPencilProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesPencilProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributesPencilProperties_Vtbl {
        unsafe extern "system" fn Opacity<Impl: IInkDrawingAttributesPencilProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IInkDrawingAttributesPencilProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributesPencilProperties, BASE_OFFSET>(),
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributesPencilProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributesStatics_Impl: Sized {
    fn CreateForPencil(&mut self) -> ::windows::core::Result<InkDrawingAttributes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributesStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributesStatics_Vtbl {
        unsafe extern "system" fn CreateForPencil<Impl: IInkDrawingAttributesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForPencil() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkDrawingAttributesStatics, BASE_OFFSET>(),
            CreateForPencil: CreateForPencil::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkInputConfiguration_Impl: Sized {
    fn IsPrimaryBarrelButtonInputEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryBarrelButtonInputEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsEraserInputEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEraserInputEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkInputConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IInkInputConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkInputConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkInputConfiguration_Vtbl {
        unsafe extern "system" fn IsPrimaryBarrelButtonInputEnabled<Impl: IInkInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrimaryBarrelButtonInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPrimaryBarrelButtonInputEnabled<Impl: IInkInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPrimaryBarrelButtonInputEnabled(value).into()
        }
        unsafe extern "system" fn IsEraserInputEnabled<Impl: IInkInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEraserInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEraserInputEnabled<Impl: IInkInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEraserInputEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkInputConfiguration, BASE_OFFSET>(),
            IsPrimaryBarrelButtonInputEnabled: IsPrimaryBarrelButtonInputEnabled::<Impl, IMPL_OFFSET>,
            SetIsPrimaryBarrelButtonInputEnabled: SetIsPrimaryBarrelButtonInputEnabled::<Impl, IMPL_OFFSET>,
            IsEraserInputEnabled: IsEraserInputEnabled::<Impl, IMPL_OFFSET>,
            SetIsEraserInputEnabled: SetIsEraserInputEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkInputConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkInputConfiguration2_Impl: Sized {
    fn IsPenHapticFeedbackEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPenHapticFeedbackEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkInputConfiguration2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkInputConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkInputConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkInputConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkInputConfiguration2_Vtbl {
        unsafe extern "system" fn IsPenHapticFeedbackEnabled<Impl: IInkInputConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPenHapticFeedbackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPenHapticFeedbackEnabled<Impl: IInkInputConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPenHapticFeedbackEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkInputConfiguration2, BASE_OFFSET>(),
            IsPenHapticFeedbackEnabled: IsPenHapticFeedbackEnabled::<Impl, IMPL_OFFSET>,
            SetIsPenHapticFeedbackEnabled: SetIsPenHapticFeedbackEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkInputConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkInputProcessingConfiguration_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<InkInputProcessingMode>;
    fn SetMode(&mut self, value: InkInputProcessingMode) -> ::windows::core::Result<()>;
    fn RightDragAction(&mut self) -> ::windows::core::Result<InkInputRightDragAction>;
    fn SetRightDragAction(&mut self, value: InkInputRightDragAction) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkInputProcessingConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IInkInputProcessingConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkInputProcessingConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkInputProcessingConfiguration_Vtbl {
        unsafe extern "system" fn Mode<Impl: IInkInputProcessingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkInputProcessingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IInkInputProcessingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkInputProcessingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn RightDragAction<Impl: IInkInputProcessingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkInputRightDragAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightDragAction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightDragAction<Impl: IInkInputProcessingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkInputRightDragAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightDragAction(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkInputProcessingConfiguration, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            RightDragAction: RightDragAction::<Impl, IMPL_OFFSET>,
            SetRightDragAction: SetRightDragAction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkInputProcessingConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IInkManager_Impl: Sized + IInkRecognizerContainer_Impl + IInkStrokeContainer_Impl {
    fn Mode(&mut self) -> ::windows::core::Result<InkManipulationMode>;
    fn SetMode(&mut self, value: InkManipulationMode) -> ::windows::core::Result<()>;
    fn ProcessPointerDown(&mut self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessPointerUpdate(&mut self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ProcessPointerUp(&mut self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetDefaultDrawingAttributes(&mut self, drawingattributes: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn RecognizeAsync2(&mut self, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IInkManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkManager_Vtbl {
        unsafe extern "system" fn Mode<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkManipulationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkManipulationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn ProcessPointerDown<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessPointerDown(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessPointerUpdate<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessPointerUpdate(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessPointerUp<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessPointerUp(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDrawingAttributes<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDrawingAttributes(&*(&drawingattributes as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecognizeAsync2<Impl: IInkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizeAsync2(recognitiontarget) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkManager, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            ProcessPointerDown: ProcessPointerDown::<Impl, IMPL_OFFSET>,
            ProcessPointerUpdate: ProcessPointerUpdate::<Impl, IMPL_OFFSET>,
            ProcessPointerUp: ProcessPointerUp::<Impl, IMPL_OFFSET>,
            SetDefaultDrawingAttributes: SetDefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            RecognizeAsync2: RecognizeAsync2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkModelerAttributes_Impl: Sized {
    fn PredictionTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetPredictionTime(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScalingFactor(&mut self) -> ::windows::core::Result<f32>;
    fn SetScalingFactor(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkModelerAttributes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkModelerAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkModelerAttributes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkModelerAttributes_Vtbl {
        unsafe extern "system" fn PredictionTime<Impl: IInkModelerAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PredictionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPredictionTime<Impl: IInkModelerAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredictionTime(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScalingFactor<Impl: IInkModelerAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScalingFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScalingFactor<Impl: IInkModelerAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScalingFactor(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkModelerAttributes, BASE_OFFSET>(),
            PredictionTime: PredictionTime::<Impl, IMPL_OFFSET>,
            SetPredictionTime: SetPredictionTime::<Impl, IMPL_OFFSET>,
            ScalingFactor: ScalingFactor::<Impl, IMPL_OFFSET>,
            SetScalingFactor: SetScalingFactor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkModelerAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkModelerAttributes2_Impl: Sized {
    fn UseVelocityBasedPressure(&mut self) -> ::windows::core::Result<bool>;
    fn SetUseVelocityBasedPressure(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkModelerAttributes2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkModelerAttributes2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkModelerAttributes2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkModelerAttributes2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkModelerAttributes2_Vtbl {
        unsafe extern "system" fn UseVelocityBasedPressure<Impl: IInkModelerAttributes2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseVelocityBasedPressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseVelocityBasedPressure<Impl: IInkModelerAttributes2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseVelocityBasedPressure(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkModelerAttributes2, BASE_OFFSET>(),
            UseVelocityBasedPressure: UseVelocityBasedPressure::<Impl, IMPL_OFFSET>,
            SetUseVelocityBasedPressure: SetUseVelocityBasedPressure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkModelerAttributes2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkPoint_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Pressure(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPoint";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkPoint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPoint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPoint_Vtbl {
        unsafe extern "system" fn Position<Impl: IInkPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pressure<Impl: IInkPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPoint, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPoint as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPoint2_Impl: Sized {
    fn TiltX(&mut self) -> ::windows::core::Result<f32>;
    fn TiltY(&mut self) -> ::windows::core::Result<f32>;
    fn Timestamp(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPoint2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPoint2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPoint2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPoint2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPoint2_Vtbl {
        unsafe extern "system" fn TiltX<Impl: IInkPoint2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiltY<Impl: IInkPoint2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IInkPoint2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPoint2, BASE_OFFSET>(),
            TiltX: TiltX::<Impl, IMPL_OFFSET>,
            TiltY: TiltY::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPoint2 as ::windows::core::Interface>::IID
    }
}
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkPointFactory2_Impl: Sized {
    fn CreateInkPointWithTiltAndTimestamp(&mut self, position: &super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::core::Result<InkPoint>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPointFactory2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPointFactory2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkPointFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPointFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPointFactory2_Vtbl {
        unsafe extern "system" fn CreateInkPointWithTiltAndTimestamp<Impl: IInkPointFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInkPointWithTiltAndTimestamp(&*(&position as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), pressure, tiltx, tilty, timestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPointFactory2, BASE_OFFSET>(),
            CreateInkPointWithTiltAndTimestamp: CreateInkPointWithTiltAndTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPointFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IInkPresenter_Impl: Sized {
    fn IsInputEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InputDeviceTypes(&mut self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes>;
    fn SetInputDeviceTypes(&mut self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()>;
    fn UnprocessedInput(&mut self) -> ::windows::core::Result<InkUnprocessedInput>;
    fn StrokeInput(&mut self) -> ::windows::core::Result<InkStrokeInput>;
    fn InputProcessingConfiguration(&mut self) -> ::windows::core::Result<InkInputProcessingConfiguration>;
    fn StrokeContainer(&mut self) -> ::windows::core::Result<InkStrokeContainer>;
    fn SetStrokeContainer(&mut self, value: &::core::option::Option<InkStrokeContainer>) -> ::windows::core::Result<()>;
    fn CopyDefaultDrawingAttributes(&mut self) -> ::windows::core::Result<InkDrawingAttributes>;
    fn UpdateDefaultDrawingAttributes(&mut self, value: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn ActivateCustomDrying(&mut self) -> ::windows::core::Result<InkSynchronizer>;
    fn SetPredefinedConfiguration(&mut self, value: InkPresenterPredefinedConfiguration) -> ::windows::core::Result<()>;
    fn StrokesCollected(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokesCollected(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokesErased(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokesErased(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenter";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkPresenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenter_Vtbl {
        unsafe extern "system" fn IsInputEnabled<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInputEnabled(value).into()
        }
        unsafe extern "system" fn InputDeviceTypes<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDeviceTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputDeviceTypes<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputDeviceTypes(value).into()
        }
        unsafe extern "system" fn UnprocessedInput<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprocessedInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeInput<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputProcessingConfiguration<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputProcessingConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeContainer<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeContainer<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeContainer(&*(&value as *const <InkStrokeContainer as ::windows::core::Abi>::Abi as *const <InkStrokeContainer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyDefaultDrawingAttributes<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyDefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDefaultDrawingAttributes<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateDefaultDrawingAttributes(&*(&value as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActivateCustomDrying<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateCustomDrying() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPredefinedConfiguration<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkPresenterPredefinedConfiguration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredefinedConfiguration(value).into()
        }
        unsafe extern "system" fn StrokesCollected<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokesCollected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokesCollected<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokesCollected(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokesErased<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokesErased(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokesErased<Impl: IInkPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokesErased(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenter, BASE_OFFSET>(),
            IsInputEnabled: IsInputEnabled::<Impl, IMPL_OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Impl, IMPL_OFFSET>,
            InputDeviceTypes: InputDeviceTypes::<Impl, IMPL_OFFSET>,
            SetInputDeviceTypes: SetInputDeviceTypes::<Impl, IMPL_OFFSET>,
            UnprocessedInput: UnprocessedInput::<Impl, IMPL_OFFSET>,
            StrokeInput: StrokeInput::<Impl, IMPL_OFFSET>,
            InputProcessingConfiguration: InputProcessingConfiguration::<Impl, IMPL_OFFSET>,
            StrokeContainer: StrokeContainer::<Impl, IMPL_OFFSET>,
            SetStrokeContainer: SetStrokeContainer::<Impl, IMPL_OFFSET>,
            CopyDefaultDrawingAttributes: CopyDefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            UpdateDefaultDrawingAttributes: UpdateDefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            ActivateCustomDrying: ActivateCustomDrying::<Impl, IMPL_OFFSET>,
            SetPredefinedConfiguration: SetPredefinedConfiguration::<Impl, IMPL_OFFSET>,
            StrokesCollected: StrokesCollected::<Impl, IMPL_OFFSET>,
            RemoveStrokesCollected: RemoveStrokesCollected::<Impl, IMPL_OFFSET>,
            StrokesErased: StrokesErased::<Impl, IMPL_OFFSET>,
            RemoveStrokesErased: RemoveStrokesErased::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IInkPresenter2_Impl: Sized + IInkPresenter_Impl {
    fn HighContrastAdjustment(&mut self) -> ::windows::core::Result<InkHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&mut self, value: InkHighContrastAdjustment) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenter2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenter2";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkPresenter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenter2_Vtbl {
        unsafe extern "system" fn HighContrastAdjustment<Impl: IInkPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighContrastAdjustment<Impl: IInkPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighContrastAdjustment(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenter2, BASE_OFFSET>(),
            HighContrastAdjustment: HighContrastAdjustment::<Impl, IMPL_OFFSET>,
            SetHighContrastAdjustment: SetHighContrastAdjustment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenter3_Impl: Sized {
    fn InputConfiguration(&mut self) -> ::windows::core::Result<InkInputConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPresenter3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenter3";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPresenter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenter3_Vtbl {
        unsafe extern "system" fn InputConfiguration<Impl: IInkPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenter3, BASE_OFFSET>(),
            InputConfiguration: InputConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInkPresenterProtractor_Impl: Sized + IInkPresenterStencil_Impl {
    fn AreTickMarksVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetAreTickMarksVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AreRaysVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetAreRaysVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCenterMarkerVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCenterMarkerVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAngleReadoutVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAngleReadoutVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsResizable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsResizable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Radius(&mut self) -> ::windows::core::Result<f64>;
    fn SetRadius(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn AccentColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetAccentColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterProtractor";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkPresenterProtractor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterProtractor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterProtractor_Vtbl {
        unsafe extern "system" fn AreTickMarksVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreTickMarksVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreTickMarksVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreTickMarksVisible(value).into()
        }
        unsafe extern "system" fn AreRaysVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreRaysVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreRaysVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreRaysVisible(value).into()
        }
        unsafe extern "system" fn IsCenterMarkerVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCenterMarkerVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCenterMarkerVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCenterMarkerVisible(value).into()
        }
        unsafe extern "system" fn IsAngleReadoutVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAngleReadoutVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAngleReadoutVisible<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAngleReadoutVisible(value).into()
        }
        unsafe extern "system" fn IsResizable<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResizable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsResizable<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsResizable(value).into()
        }
        unsafe extern "system" fn Radius<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Radius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadius<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadius(value).into()
        }
        unsafe extern "system" fn AccentColor<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccentColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccentColor<Impl: IInkPresenterProtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccentColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenterProtractor, BASE_OFFSET>(),
            AreTickMarksVisible: AreTickMarksVisible::<Impl, IMPL_OFFSET>,
            SetAreTickMarksVisible: SetAreTickMarksVisible::<Impl, IMPL_OFFSET>,
            AreRaysVisible: AreRaysVisible::<Impl, IMPL_OFFSET>,
            SetAreRaysVisible: SetAreRaysVisible::<Impl, IMPL_OFFSET>,
            IsCenterMarkerVisible: IsCenterMarkerVisible::<Impl, IMPL_OFFSET>,
            SetIsCenterMarkerVisible: SetIsCenterMarkerVisible::<Impl, IMPL_OFFSET>,
            IsAngleReadoutVisible: IsAngleReadoutVisible::<Impl, IMPL_OFFSET>,
            SetIsAngleReadoutVisible: SetIsAngleReadoutVisible::<Impl, IMPL_OFFSET>,
            IsResizable: IsResizable::<Impl, IMPL_OFFSET>,
            SetIsResizable: SetIsResizable::<Impl, IMPL_OFFSET>,
            Radius: Radius::<Impl, IMPL_OFFSET>,
            SetRadius: SetRadius::<Impl, IMPL_OFFSET>,
            AccentColor: AccentColor::<Impl, IMPL_OFFSET>,
            SetAccentColor: SetAccentColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterProtractor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterProtractorFactory_Impl: Sized {
    fn Create(&mut self, inkpresenter: &::core::option::Option<InkPresenter>) -> ::windows::core::Result<InkPresenterProtractor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPresenterProtractorFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterProtractorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPresenterProtractorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterProtractorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterProtractorFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IInkPresenterProtractorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenterProtractorFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterProtractorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInkPresenterRuler_Impl: Sized + IInkPresenterStencil_Impl {
    fn Length(&mut self) -> ::windows::core::Result<f64>;
    fn SetLength(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Width(&mut self) -> ::windows::core::Result<f64>;
    fn SetWidth(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRuler";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkPresenterRuler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterRuler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterRuler_Vtbl {
        unsafe extern "system" fn Length<Impl: IInkPresenterRuler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IInkPresenterRuler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        unsafe extern "system" fn Width<Impl: IInkPresenterRuler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: IInkPresenterRuler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenterRuler, BASE_OFFSET>(),
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterRuler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterRuler2_Impl: Sized {
    fn AreTickMarksVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetAreTickMarksVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCompassVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCompassVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPresenterRuler2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRuler2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPresenterRuler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterRuler2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterRuler2_Vtbl {
        unsafe extern "system" fn AreTickMarksVisible<Impl: IInkPresenterRuler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreTickMarksVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreTickMarksVisible<Impl: IInkPresenterRuler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreTickMarksVisible(value).into()
        }
        unsafe extern "system" fn IsCompassVisible<Impl: IInkPresenterRuler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompassVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCompassVisible<Impl: IInkPresenterRuler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCompassVisible(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkPresenterRuler2, BASE_OFFSET>(),
            AreTickMarksVisible: AreTickMarksVisible::<Impl, IMPL_OFFSET>,
            SetAreTickMarksVisible: SetAreTickMarksVisible::<Impl, IMPL_OFFSET>,
            IsCompassVisible: IsCompassVisible::<Impl, IMPL_OFFSET>,
            SetIsCompassVisible: SetIsCompassVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterRuler2 as ::windows::core::Interface>::IID
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkRecognitionResult_Impl: Sized {
    fn BoundingRect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetTextCandidates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetStrokes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognitionResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkRecognitionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognitionResult_Vtbl {
        unsafe extern "system" fn BoundingRect<Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTextCandidates<Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextCandidates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokes<Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkRecognitionResult, BASE_OFFSET>(),
            BoundingRect: BoundingRect::<Impl, IMPL_OFFSET>,
            GetTextCandidates: GetTextCandidates::<Impl, IMPL_OFFSET>,
            GetStrokes: GetStrokes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkRecognizer_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognizer";
}
#[cfg(feature = "implement_exclusive")]
impl IInkRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizer_Vtbl {
        unsafe extern "system" fn Name<Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkRecognizer, BASE_OFFSET>(), Name: Name::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizer as ::windows::core::Interface>::IID
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkStroke_Impl: Sized {
    fn DrawingAttributes(&mut self) -> ::windows::core::Result<InkDrawingAttributes>;
    fn SetDrawingAttributes(&mut self, value: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn BoundingRect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn Selected(&mut self) -> ::windows::core::Result<bool>;
    fn SetSelected(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Recognized(&mut self) -> ::windows::core::Result<bool>;
    fn GetRenderingSegments(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>;
    fn Clone(&mut self) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStroke_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke_Vtbl {
        unsafe extern "system" fn DrawingAttributes<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDrawingAttributes<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDrawingAttributes(&*(&value as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BoundingRect<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Selected<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelected<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelected(value).into()
        }
        unsafe extern "system" fn Recognized<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderingSegments<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderingSegments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IInkStroke_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStroke, BASE_OFFSET>(),
            DrawingAttributes: DrawingAttributes::<Impl, IMPL_OFFSET>,
            SetDrawingAttributes: SetDrawingAttributes::<Impl, IMPL_OFFSET>,
            BoundingRect: BoundingRect::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
            SetSelected: SetSelected::<Impl, IMPL_OFFSET>,
            Recognized: Recognized::<Impl, IMPL_OFFSET>,
            GetRenderingSegments: GetRenderingSegments::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStroke as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInkStroke2_Impl: Sized {
    fn PointTransform(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetPointTransform(&mut self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn GetInkPoints(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStroke2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkStroke2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke2_Vtbl {
        unsafe extern "system" fn PointTransform<Impl: IInkStroke2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointTransform<Impl: IInkStroke2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointTransform(&*(&value as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetInkPoints<Impl: IInkStroke2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInkPoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStroke2, BASE_OFFSET>(),
            PointTransform: PointTransform::<Impl, IMPL_OFFSET>,
            SetPointTransform: SetPointTransform::<Impl, IMPL_OFFSET>,
            GetInkPoints: GetInkPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStroke2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkStroke3_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn StrokeStartedTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetStrokeStartedTime(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn StrokeDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetStrokeDuration(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStroke3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkStroke3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke3_Vtbl {
        unsafe extern "system" fn Id<Impl: IInkStroke3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeStartedTime<Impl: IInkStroke3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeStartedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartedTime<Impl: IInkStroke3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartedTime(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeDuration<Impl: IInkStroke3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDuration<Impl: IInkStroke3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDuration(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStroke3, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            StrokeStartedTime: StrokeStartedTime::<Impl, IMPL_OFFSET>,
            SetStrokeStartedTime: SetStrokeStartedTime::<Impl, IMPL_OFFSET>,
            StrokeDuration: StrokeDuration::<Impl, IMPL_OFFSET>,
            SetStrokeDuration: SetStrokeDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStroke3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStroke4_Impl: Sized {
    fn PointerId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkStroke4 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke4";
}
#[cfg(feature = "implement_exclusive")]
impl IInkStroke4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke4_Vtbl {
        unsafe extern "system" fn PointerId<Impl: IInkStroke4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStroke4, BASE_OFFSET>(), PointerId: PointerId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStroke4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkStrokeBuilder_Impl: Sized {
    fn BeginStroke(&mut self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<()>;
    fn AppendToStroke(&mut self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<super::PointerPoint>;
    fn EndStroke(&mut self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<InkStroke>;
    fn CreateStroke(&mut self, points: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows::core::Result<InkStroke>;
    fn SetDefaultDrawingAttributes(&mut self, drawingattributes: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeBuilder";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeBuilder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeBuilder_Vtbl {
        unsafe extern "system" fn BeginStroke<Impl: IInkStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginStroke(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendToStroke<Impl: IInkStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendToStroke(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStroke<Impl: IInkStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndStroke(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStroke<Impl: IInkStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStroke(&*(&points as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDrawingAttributes<Impl: IInkStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDrawingAttributes(&*(&drawingattributes as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeBuilder, BASE_OFFSET>(),
            BeginStroke: BeginStroke::<Impl, IMPL_OFFSET>,
            AppendToStroke: AppendToStroke::<Impl, IMPL_OFFSET>,
            EndStroke: EndStroke::<Impl, IMPL_OFFSET>,
            CreateStroke: CreateStroke::<Impl, IMPL_OFFSET>,
            SetDefaultDrawingAttributes: SetDefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInkStrokeBuilder2_Impl: Sized {
    fn CreateStrokeFromInkPoints(&mut self, inkpoints: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkPoint>>, transform: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeBuilder2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeBuilder2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkStrokeBuilder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeBuilder2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeBuilder2_Vtbl {
        unsafe extern "system" fn CreateStrokeFromInkPoints<Impl: IInkStrokeBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStrokeFromInkPoints(&*(&inkpoints as *const <super::super::super::Foundation::Collections::IIterable<InkPoint> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InkPoint> as ::windows::core::DefaultType>::DefaultType), &*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeBuilder2, BASE_OFFSET>(),
            CreateStrokeFromInkPoints: CreateStrokeFromInkPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeBuilder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInkStrokeBuilder3_Impl: Sized {
    fn CreateStrokeFromInkPoints(&mut self, inkpoints: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkPoint>>, transform: &super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, strokeduration: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeBuilder3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeBuilder3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkStrokeBuilder3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeBuilder3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeBuilder3_Vtbl {
        unsafe extern "system" fn CreateStrokeFromInkPoints<Impl: IInkStrokeBuilder3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: ::windows::core::RawPtr, strokeduration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStrokeFromInkPoints(
                &*(&inkpoints as *const <super::super::super::Foundation::Collections::IIterable<InkPoint> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InkPoint> as ::windows::core::DefaultType>::DefaultType),
                &*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                &*(&strokestartedtime as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType),
                &*(&strokeduration as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeBuilder3, BASE_OFFSET>(),
            CreateStrokeFromInkPoints: CreateStrokeFromInkPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeBuilder3 as ::windows::core::Interface>::IID
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
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkStrokeContainer2_Impl: Sized {
    fn AddStrokes(&mut self, strokes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkStroke>>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeContainer2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokeContainer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeContainer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeContainer2_Vtbl {
        unsafe extern "system" fn AddStrokes<Impl: IInkStrokeContainer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStrokes(&*(&strokes as *const <super::super::super::Foundation::Collections::IIterable<InkStroke> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InkStroke> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IInkStrokeContainer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeContainer2, BASE_OFFSET>(),
            AddStrokes: AddStrokes::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeContainer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IInkStrokeContainer3_Impl: Sized {
    fn SaveWithFormatAsync(&mut self, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>, inkpersistenceformat: InkPersistenceFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn GetStrokeById(&mut self, id: u32) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeContainer3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer3";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IInkStrokeContainer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeContainer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeContainer3_Vtbl {
        unsafe extern "system" fn SaveWithFormatAsync<Impl: IInkStrokeContainer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, inkpersistenceformat: InkPersistenceFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveWithFormatAsync(&*(&outputstream as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType), inkpersistenceformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeById<Impl: IInkStrokeContainer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeById(id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeContainer3, BASE_OFFSET>(),
            SaveWithFormatAsync: SaveWithFormatAsync::<Impl, IMPL_OFFSET>,
            GetStrokeById: GetStrokeById::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeContainer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IInkStrokeInput_Impl: Sized {
    fn StrokeStarted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeStarted(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeContinued(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeContinued(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeEnded(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeEnded(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeCanceled(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeCanceled(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&mut self) -> ::windows::core::Result<InkPresenter>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeInput";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkStrokeInput_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeInput_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeInput_Vtbl {
        unsafe extern "system" fn StrokeStarted<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeStarted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokeStarted<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeStarted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeContinued<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeContinued(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokeContinued<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeContinued(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeEnded<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeEnded(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokeEnded<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeEnded(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeCanceled<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeCanceled(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokeCanceled<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeCanceled(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InkPresenter<Impl: IInkStrokeInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeInput, BASE_OFFSET>(),
            StrokeStarted: StrokeStarted::<Impl, IMPL_OFFSET>,
            RemoveStrokeStarted: RemoveStrokeStarted::<Impl, IMPL_OFFSET>,
            StrokeContinued: StrokeContinued::<Impl, IMPL_OFFSET>,
            RemoveStrokeContinued: RemoveStrokeContinued::<Impl, IMPL_OFFSET>,
            StrokeEnded: StrokeEnded::<Impl, IMPL_OFFSET>,
            RemoveStrokeEnded: RemoveStrokeEnded::<Impl, IMPL_OFFSET>,
            StrokeCanceled: StrokeCanceled::<Impl, IMPL_OFFSET>,
            RemoveStrokeCanceled: RemoveStrokeCanceled::<Impl, IMPL_OFFSET>,
            InkPresenter: InkPresenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeInput as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkStrokeRenderingSegment_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn BezierControlPoint1(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn BezierControlPoint2(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Pressure(&mut self) -> ::windows::core::Result<f32>;
    fn TiltX(&mut self) -> ::windows::core::Result<f32>;
    fn TiltY(&mut self) -> ::windows::core::Result<f32>;
    fn Twist(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeRenderingSegment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkStrokeRenderingSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeRenderingSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeRenderingSegment_Vtbl {
        unsafe extern "system" fn Position<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierControlPoint1<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BezierControlPoint1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierControlPoint2<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BezierControlPoint2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pressure<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiltX<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiltY<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TiltY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Twist<Impl: IInkStrokeRenderingSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Twist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokeRenderingSegment, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            BezierControlPoint1: BezierControlPoint1::<Impl, IMPL_OFFSET>,
            BezierControlPoint2: BezierControlPoint2::<Impl, IMPL_OFFSET>,
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
            TiltX: TiltX::<Impl, IMPL_OFFSET>,
            TiltY: TiltY::<Impl, IMPL_OFFSET>,
            Twist: Twist::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeRenderingSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkStrokesCollectedEventArgs_Impl: Sized {
    fn Strokes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokesCollectedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokesCollectedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokesCollectedEventArgs_Vtbl {
        unsafe extern "system" fn Strokes<Impl: IInkStrokesCollectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokesCollectedEventArgs, BASE_OFFSET>(), Strokes: Strokes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokesCollectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkStrokesErasedEventArgs_Impl: Sized {
    fn Strokes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokesErasedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokesErasedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokesErasedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokesErasedEventArgs_Vtbl {
        unsafe extern "system" fn Strokes<Impl: IInkStrokesErasedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkStrokesErasedEventArgs, BASE_OFFSET>(), Strokes: Strokes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokesErasedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkSynchronizer_Impl: Sized {
    fn BeginDry(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn EndDry(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkSynchronizer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkSynchronizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkSynchronizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkSynchronizer_Vtbl {
        unsafe extern "system" fn BeginDry<Impl: IInkSynchronizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDry<Impl: IInkSynchronizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndDry().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkSynchronizer, BASE_OFFSET>(),
            BeginDry: BeginDry::<Impl, IMPL_OFFSET>,
            EndDry: EndDry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkSynchronizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IInkUnprocessedInput_Impl: Sized {
    fn PointerEntered(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerHovered(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerHovered(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerLost(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerLost(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&mut self) -> ::windows::core::Result<InkPresenter>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkUnprocessedInput";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkUnprocessedInput_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkUnprocessedInput_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkUnprocessedInput_Vtbl {
        unsafe extern "system" fn PointerEntered<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerEntered(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerEntered(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerHovered<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerHovered(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerHovered<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerHovered(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExited<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerExited(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerExited(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressed<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPressed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerPressed(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoved<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerMoved(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerMoved(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleased<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerReleased(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerReleased(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerLost<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerLost(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerLost<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerLost(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InkPresenter<Impl: IInkUnprocessedInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkUnprocessedInput, BASE_OFFSET>(),
            PointerEntered: PointerEntered::<Impl, IMPL_OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Impl, IMPL_OFFSET>,
            PointerHovered: PointerHovered::<Impl, IMPL_OFFSET>,
            RemovePointerHovered: RemovePointerHovered::<Impl, IMPL_OFFSET>,
            PointerExited: PointerExited::<Impl, IMPL_OFFSET>,
            RemovePointerExited: RemovePointerExited::<Impl, IMPL_OFFSET>,
            PointerPressed: PointerPressed::<Impl, IMPL_OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Impl, IMPL_OFFSET>,
            PointerMoved: PointerMoved::<Impl, IMPL_OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Impl, IMPL_OFFSET>,
            PointerReleased: PointerReleased::<Impl, IMPL_OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Impl, IMPL_OFFSET>,
            PointerLost: PointerLost::<Impl, IMPL_OFFSET>,
            RemovePointerLost: RemovePointerLost::<Impl, IMPL_OFFSET>,
            InkPresenter: InkPresenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkUnprocessedInput as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenAndInkSettings_Impl: Sized {
    fn IsHandwritingDirectlyIntoTextFieldEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn PenHandedness(&mut self) -> ::windows::core::Result<PenHandedness>;
    fn HandwritingLineHeight(&mut self) -> ::windows::core::Result<HandwritingLineHeight>;
    fn FontFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserConsentsToHandwritingTelemetryCollection(&mut self) -> ::windows::core::Result<bool>;
    fn IsTouchHandwritingEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.IPenAndInkSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IPenAndInkSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenAndInkSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenAndInkSettings_Vtbl {
        unsafe extern "system" fn IsHandwritingDirectlyIntoTextFieldEnabled<Impl: IPenAndInkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHandwritingDirectlyIntoTextFieldEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PenHandedness<Impl: IPenAndInkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PenHandedness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenHandedness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandwritingLineHeight<Impl: IPenAndInkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HandwritingLineHeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandwritingLineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontFamilyName<Impl: IPenAndInkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserConsentsToHandwritingTelemetryCollection<Impl: IPenAndInkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserConsentsToHandwritingTelemetryCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTouchHandwritingEnabled<Impl: IPenAndInkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTouchHandwritingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPenAndInkSettings, BASE_OFFSET>(),
            IsHandwritingDirectlyIntoTextFieldEnabled: IsHandwritingDirectlyIntoTextFieldEnabled::<Impl, IMPL_OFFSET>,
            PenHandedness: PenHandedness::<Impl, IMPL_OFFSET>,
            HandwritingLineHeight: HandwritingLineHeight::<Impl, IMPL_OFFSET>,
            FontFamilyName: FontFamilyName::<Impl, IMPL_OFFSET>,
            UserConsentsToHandwritingTelemetryCollection: UserConsentsToHandwritingTelemetryCollection::<Impl, IMPL_OFFSET>,
            IsTouchHandwritingEnabled: IsTouchHandwritingEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenAndInkSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenAndInkSettings2_Impl: Sized {
    fn SetPenHandedness(&mut self, value: PenHandedness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenAndInkSettings2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IPenAndInkSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IPenAndInkSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenAndInkSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenAndInkSettings2_Vtbl {
        unsafe extern "system" fn SetPenHandedness<Impl: IPenAndInkSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PenHandedness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenHandedness(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPenAndInkSettings2, BASE_OFFSET>(),
            SetPenHandedness: SetPenHandedness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenAndInkSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenAndInkSettingsStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<PenAndInkSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenAndInkSettingsStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.IPenAndInkSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenAndInkSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenAndInkSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenAndInkSettingsStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IPenAndInkSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPenAndInkSettingsStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenAndInkSettingsStatics as ::windows::core::Interface>::IID
    }
}
