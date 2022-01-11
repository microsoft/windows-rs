#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInkDrawingAttributesImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn PenTip(&self) -> ::windows::core::Result<PenTipShape>;
    fn SetPenTip(&self, value: PenTipShape) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn SetSize(&self, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn IgnorePressure(&self) -> ::windows::core::Result<bool>;
    fn SetIgnorePressure(&self, value: bool) -> ::windows::core::Result<()>;
    fn FitToCurve(&self) -> ::windows::core::Result<bool>;
    fn SetFitToCurve(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkDrawingAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributesVtbl {
        unsafe extern "system" fn Color<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PenTip<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PenTipShape) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPenTip<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PenTipShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenTip(value).into()
        }
        unsafe extern "system" fn Size<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnorePressure<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIgnorePressure<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnorePressure(value).into()
        }
        unsafe extern "system" fn FitToCurve<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFitToCurve<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkDrawingAttributes2Impl: Sized {
    fn PenTipTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetPenTipTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn DrawAsHighlighter(&self) -> ::windows::core::Result<bool>;
    fn SetDrawAsHighlighter(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkDrawingAttributes2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkDrawingAttributes2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes2Vtbl {
        unsafe extern "system" fn PenTipTransform<Impl: IInkDrawingAttributes2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPenTipTransform<Impl: IInkDrawingAttributes2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPenTipTransform(&*(&value as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawAsHighlighter<Impl: IInkDrawingAttributes2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDrawAsHighlighter<Impl: IInkDrawingAttributes2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkDrawingAttributes3Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<InkDrawingAttributesKind>;
    fn PencilProperties(&self) -> ::windows::core::Result<InkDrawingAttributesPencilProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes3";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributes3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes3Vtbl {
        unsafe extern "system" fn Kind<Impl: IInkDrawingAttributes3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkDrawingAttributesKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PencilProperties<Impl: IInkDrawingAttributes3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkDrawingAttributes4Impl: Sized {
    fn IgnoreTilt(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTilt(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes4 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes4";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributes4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes4Vtbl {
        unsafe extern "system" fn IgnoreTilt<Impl: IInkDrawingAttributes4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIgnoreTilt<Impl: IInkDrawingAttributes4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkDrawingAttributes5Impl: Sized {
    fn ModelerAttributes(&self) -> ::windows::core::Result<InkModelerAttributes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes5 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributes5";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributes5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributes5Vtbl {
        unsafe extern "system" fn ModelerAttributes<Impl: IInkDrawingAttributes5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkDrawingAttributesPencilPropertiesImpl: Sized {
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributesPencilPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesPencilPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributesPencilPropertiesVtbl {
        unsafe extern "system" fn Opacity<Impl: IInkDrawingAttributesPencilPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpacity<Impl: IInkDrawingAttributesPencilPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IInkDrawingAttributesStaticsImpl: Sized {
    fn CreateForPencil(&self) -> ::windows::core::Result<InkDrawingAttributes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkDrawingAttributesStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkDrawingAttributesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInkDrawingAttributesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributesStaticsVtbl {
        unsafe extern "system" fn CreateForPencil<Impl: IInkDrawingAttributesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkInputConfigurationImpl: Sized {
    fn IsPrimaryBarrelButtonInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEraserInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEraserInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkInputConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IInkInputConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkInputConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkInputConfigurationVtbl {
        unsafe extern "system" fn IsPrimaryBarrelButtonInputEnabled<Impl: IInkInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPrimaryBarrelButtonInputEnabled<Impl: IInkInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPrimaryBarrelButtonInputEnabled(value).into()
        }
        unsafe extern "system" fn IsEraserInputEnabled<Impl: IInkInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEraserInputEnabled<Impl: IInkInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkInputConfiguration2Impl: Sized {
    fn IsPenHapticFeedbackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkInputConfiguration2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkInputConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkInputConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkInputConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkInputConfiguration2Vtbl {
        unsafe extern "system" fn IsPenHapticFeedbackEnabled<Impl: IInkInputConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPenHapticFeedbackEnabled<Impl: IInkInputConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkInputProcessingConfigurationImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<InkInputProcessingMode>;
    fn SetMode(&self, value: InkInputProcessingMode) -> ::windows::core::Result<()>;
    fn RightDragAction(&self) -> ::windows::core::Result<InkInputRightDragAction>;
    fn SetRightDragAction(&self, value: InkInputRightDragAction) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkInputProcessingConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IInkInputProcessingConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkInputProcessingConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkInputProcessingConfigurationVtbl {
        unsafe extern "system" fn Mode<Impl: IInkInputProcessingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkInputProcessingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMode<Impl: IInkInputProcessingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkInputProcessingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn RightDragAction<Impl: IInkInputProcessingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkInputRightDragAction) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightDragAction<Impl: IInkInputProcessingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkInputRightDragAction) -> ::windows::core::HRESULT {
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
pub trait IInkManagerImpl: Sized + IInkRecognizerContainerImpl + IInkStrokeContainerImpl {
    fn Mode(&self) -> ::windows::core::Result<InkManipulationMode>;
    fn SetMode(&self, value: InkManipulationMode) -> ::windows::core::Result<()>;
    fn ProcessPointerDown(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessPointerUpdate(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ProcessPointerUp(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetDefaultDrawingAttributes(&self, drawingattributes: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IInkManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkManagerVtbl {
        unsafe extern "system" fn Mode<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkManipulationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMode<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkManipulationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn ProcessPointerDown<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessPointerDown(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessPointerUpdate<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProcessPointerUp<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultDrawingAttributes<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDrawingAttributes(&*(&drawingattributes as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecognizeAsync2<Impl: IInkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkModelerAttributesImpl: Sized {
    fn PredictionTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetPredictionTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScalingFactor(&self) -> ::windows::core::Result<f32>;
    fn SetScalingFactor(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkModelerAttributes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkModelerAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkModelerAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkModelerAttributesVtbl {
        unsafe extern "system" fn PredictionTime<Impl: IInkModelerAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPredictionTime<Impl: IInkModelerAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredictionTime(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScalingFactor<Impl: IInkModelerAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScalingFactor<Impl: IInkModelerAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IInkModelerAttributes2Impl: Sized {
    fn UseVelocityBasedPressure(&self) -> ::windows::core::Result<bool>;
    fn SetUseVelocityBasedPressure(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkModelerAttributes2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkModelerAttributes2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkModelerAttributes2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkModelerAttributes2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkModelerAttributes2Vtbl {
        unsafe extern "system" fn UseVelocityBasedPressure<Impl: IInkModelerAttributes2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUseVelocityBasedPressure<Impl: IInkModelerAttributes2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkPointImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Pressure(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPoint";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkPointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPointVtbl {
        unsafe extern "system" fn Position<Impl: IInkPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pressure<Impl: IInkPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IInkPoint2Impl: Sized {
    fn TiltX(&self) -> ::windows::core::Result<f32>;
    fn TiltY(&self) -> ::windows::core::Result<f32>;
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPoint2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPoint2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPoint2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPoint2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPoint2Vtbl {
        unsafe extern "system" fn TiltX<Impl: IInkPoint2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TiltY<Impl: IInkPoint2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamp<Impl: IInkPoint2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
pub trait IInkPointFactoryImpl: Sized {
    fn CreateInkPoint(&self, position: &super::super::super::Foundation::Point, pressure: f32) -> ::windows::core::Result<InkPoint>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInkPointFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPointFactory";
}
#[cfg(feature = "Foundation")]
impl IInkPointFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPointFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPointFactoryVtbl {
        unsafe extern "system" fn CreateInkPoint<Impl: IInkPointFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkPointFactory2Impl: Sized {
    fn CreateInkPointWithTiltAndTimestamp(&self, position: &super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::core::Result<InkPoint>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPointFactory2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPointFactory2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkPointFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPointFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPointFactory2Vtbl {
        unsafe extern "system" fn CreateInkPointWithTiltAndTimestamp<Impl: IInkPointFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterImpl: Sized {
    fn IsInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDeviceTypes(&self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes>;
    fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()>;
    fn UnprocessedInput(&self) -> ::windows::core::Result<InkUnprocessedInput>;
    fn StrokeInput(&self) -> ::windows::core::Result<InkStrokeInput>;
    fn InputProcessingConfiguration(&self) -> ::windows::core::Result<InkInputProcessingConfiguration>;
    fn StrokeContainer(&self) -> ::windows::core::Result<InkStrokeContainer>;
    fn SetStrokeContainer(&self, value: &::core::option::Option<InkStrokeContainer>) -> ::windows::core::Result<()>;
    fn CopyDefaultDrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes>;
    fn UpdateDefaultDrawingAttributes(&self, value: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn ActivateCustomDrying(&self) -> ::windows::core::Result<InkSynchronizer>;
    fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> ::windows::core::Result<()>;
    fn StrokesCollected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokesCollected(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokesErased(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokesErased(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenter";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkPresenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterVtbl {
        unsafe extern "system" fn IsInputEnabled<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsInputEnabled<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInputEnabled(value).into()
        }
        unsafe extern "system" fn InputDeviceTypes<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInputDeviceTypes<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputDeviceTypes(value).into()
        }
        unsafe extern "system" fn UnprocessedInput<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeInput<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputProcessingConfiguration<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeContainer<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeContainer<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeContainer(&*(&value as *const <InkStrokeContainer as ::windows::core::Abi>::Abi as *const <InkStrokeContainer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyDefaultDrawingAttributes<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateDefaultDrawingAttributes<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateDefaultDrawingAttributes(&*(&value as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActivateCustomDrying<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPredefinedConfiguration<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkPresenterPredefinedConfiguration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredefinedConfiguration(value).into()
        }
        unsafe extern "system" fn StrokesCollected<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStrokesCollected<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokesCollected(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokesErased<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStrokesErased<Impl: IInkPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IInkPresenter2Impl: Sized + IInkPresenterImpl {
    fn HighContrastAdjustment(&self) -> ::windows::core::Result<InkHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenter2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenter2";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkPresenter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenter2Vtbl {
        unsafe extern "system" fn HighContrastAdjustment<Impl: IInkPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkHighContrastAdjustment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHighContrastAdjustment<Impl: IInkPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InkHighContrastAdjustment) -> ::windows::core::HRESULT {
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
pub trait IInkPresenter3Impl: Sized {
    fn InputConfiguration(&self) -> ::windows::core::Result<InkInputConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPresenter3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenter3";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPresenter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenter3Vtbl {
        unsafe extern "system" fn InputConfiguration<Impl: IInkPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterProtractorImpl: Sized + IInkPresenterStencilImpl {
    fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn AreRaysVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAreRaysVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCenterMarkerVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCenterMarkerVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAngleReadoutVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsAngleReadoutVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsResizable(&self) -> ::windows::core::Result<bool>;
    fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()>;
    fn Radius(&self) -> ::windows::core::Result<f64>;
    fn SetRadius(&self, value: f64) -> ::windows::core::Result<()>;
    fn AccentColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetAccentColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterProtractor";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkPresenterProtractorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterProtractorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterProtractorVtbl {
        unsafe extern "system" fn AreTickMarksVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAreTickMarksVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreTickMarksVisible(value).into()
        }
        unsafe extern "system" fn AreRaysVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAreRaysVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreRaysVisible(value).into()
        }
        unsafe extern "system" fn IsCenterMarkerVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsCenterMarkerVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCenterMarkerVisible(value).into()
        }
        unsafe extern "system" fn IsAngleReadoutVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsAngleReadoutVisible<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAngleReadoutVisible(value).into()
        }
        unsafe extern "system" fn IsResizable<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsResizable<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsResizable(value).into()
        }
        unsafe extern "system" fn Radius<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRadius<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadius(value).into()
        }
        unsafe extern "system" fn AccentColor<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccentColor<Impl: IInkPresenterProtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterProtractorFactoryImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<InkPresenter>) -> ::windows::core::Result<InkPresenterProtractor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPresenterProtractorFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterProtractorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPresenterProtractorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterProtractorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterProtractorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IInkPresenterProtractorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterRulerImpl: Sized + IInkPresenterStencilImpl {
    fn Length(&self) -> ::windows::core::Result<f64>;
    fn SetLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<f64>;
    fn SetWidth(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRuler";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkPresenterRulerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterRulerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterRulerVtbl {
        unsafe extern "system" fn Length<Impl: IInkPresenterRulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLength<Impl: IInkPresenterRulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        unsafe extern "system" fn Width<Impl: IInkPresenterRulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWidth<Impl: IInkPresenterRulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterRuler2Impl: Sized {
    fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCompassVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompassVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkPresenterRuler2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRuler2";
}
#[cfg(feature = "implement_exclusive")]
impl IInkPresenterRuler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterRuler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterRuler2Vtbl {
        unsafe extern "system" fn AreTickMarksVisible<Impl: IInkPresenterRuler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAreTickMarksVisible<Impl: IInkPresenterRuler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreTickMarksVisible(value).into()
        }
        unsafe extern "system" fn IsCompassVisible<Impl: IInkPresenterRuler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsCompassVisible<Impl: IInkPresenterRuler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterRulerFactoryImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<InkPresenter>) -> ::windows::core::Result<InkPresenterRuler>;
}
impl ::windows::core::RuntimeName for IInkPresenterRulerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterRulerFactory";
}
impl IInkPresenterRulerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterRulerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterRulerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IInkPresenterRulerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkPresenterStencilImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetBackgroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetForegroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::RuntimeName for IInkPresenterStencil {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkPresenterStencil";
}
#[cfg(feature = "Foundation_Numerics")]
impl IInkPresenterStencilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterStencilImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterStencilVtbl {
        unsafe extern "system" fn Kind<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkPresenterStencilKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVisible<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsVisible<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVisible(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundColor<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForegroundColor<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Transform<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransform<Impl: IInkPresenterStencilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
pub trait IInkRecognitionResultImpl: Sized {
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetTextCandidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognitionResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkRecognitionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognitionResultVtbl {
        unsafe extern "system" fn BoundingRect<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTextCandidates<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStrokes<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkRecognizerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognizer";
}
#[cfg(feature = "implement_exclusive")]
impl IInkRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerVtbl {
        unsafe extern "system" fn Name<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IInkRecognizerContainerImpl: Sized {
    fn SetDefaultRecognizer(&self, recognizer: &::core::option::Option<InkRecognizer>) -> ::windows::core::Result<()>;
    fn RecognizeAsync(&self, strokecollection: &::core::option::Option<InkStrokeContainer>, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
    fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IInkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkRecognizerContainer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IInkRecognizerContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerContainerVtbl {
        unsafe extern "system" fn SetDefaultRecognizer<Impl: IInkRecognizerContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultRecognizer(&*(&recognizer as *const <InkRecognizer as ::windows::core::Abi>::Abi as *const <InkRecognizer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecognizeAsync<Impl: IInkRecognizerContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokecollection: ::windows::core::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRecognizers<Impl: IInkRecognizerContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeImpl: Sized {
    fn DrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes>;
    fn SetDrawingAttributes(&self, value: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn Selected(&self) -> ::windows::core::Result<bool>;
    fn SetSelected(&self, value: bool) -> ::windows::core::Result<()>;
    fn Recognized(&self) -> ::windows::core::Result<bool>;
    fn GetRenderingSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>;
    fn Clone(&self) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeVtbl {
        unsafe extern "system" fn DrawingAttributes<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDrawingAttributes<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDrawingAttributes(&*(&value as *const <InkDrawingAttributes as ::windows::core::Abi>::Abi as *const <InkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BoundingRect<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Selected<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelected<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelected(value).into()
        }
        unsafe extern "system" fn Recognized<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRenderingSegments<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IInkStrokeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStroke2Impl: Sized {
    fn PointTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetPointTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn GetInkPoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStroke2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkStroke2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke2Vtbl {
        unsafe extern "system" fn PointTransform<Impl: IInkStroke2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointTransform<Impl: IInkStroke2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointTransform(&*(&value as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetInkPoints<Impl: IInkStroke2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStroke3Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn StrokeStartedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetStrokeStartedTime(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn StrokeDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetStrokeDuration(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStroke3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkStroke3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke3Vtbl {
        unsafe extern "system" fn Id<Impl: IInkStroke3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeStartedTime<Impl: IInkStroke3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeStartedTime<Impl: IInkStroke3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartedTime(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeDuration<Impl: IInkStroke3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeDuration<Impl: IInkStroke3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStroke4Impl: Sized {
    fn PointerId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkStroke4 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStroke4";
}
#[cfg(feature = "implement_exclusive")]
impl IInkStroke4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStroke4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStroke4Vtbl {
        unsafe extern "system" fn PointerId<Impl: IInkStroke4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeBuilderImpl: Sized {
    fn BeginStroke(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<()>;
    fn AppendToStroke(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<super::PointerPoint>;
    fn EndStroke(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<InkStroke>;
    fn CreateStroke(&self, points: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows::core::Result<InkStroke>;
    fn SetDefaultDrawingAttributes(&self, drawingattributes: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeBuilder";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeBuilderVtbl {
        unsafe extern "system" fn BeginStroke<Impl: IInkStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginStroke(&*(&pointerpoint as *const <super::PointerPoint as ::windows::core::Abi>::Abi as *const <super::PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendToStroke<Impl: IInkStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndStroke<Impl: IInkStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateStroke<Impl: IInkStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultDrawingAttributes<Impl: IInkStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeBuilder2Impl: Sized {
    fn CreateStrokeFromInkPoints(&self, inkpoints: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkPoint>>, transform: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeBuilder2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeBuilder2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkStrokeBuilder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeBuilder2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeBuilder2Vtbl {
        unsafe extern "system" fn CreateStrokeFromInkPoints<Impl: IInkStrokeBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeBuilder3Impl: Sized {
    fn CreateStrokeFromInkPoints(&self, inkpoints: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkPoint>>, transform: &super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, strokeduration: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeBuilder3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeBuilder3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInkStrokeBuilder3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeBuilder3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeBuilder3Vtbl {
        unsafe extern "system" fn CreateStrokeFromInkPoints<Impl: IInkStrokeBuilder3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: ::windows::core::RawPtr, strokeduration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeContainerImpl: Sized {
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn AddStroke(&self, stroke: &::core::option::Option<InkStroke>) -> ::windows::core::Result<()>;
    fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn MoveSelected(&self, translation: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithPolyLine(&self, polyline: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithLine(&self, from: &super::super::super::Foundation::Point, to: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()>;
    fn PasteFromClipboard(&self, position: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool>;
    fn LoadAsync(&self, inputstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>;
    fn SaveAsync(&self, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn UpdateRecognitionResults(&self, recognitionresults: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>) -> ::windows::core::Result<()>;
    fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IInkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IInkStrokeContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeContainerVtbl {
        unsafe extern "system" fn BoundingRect<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddStroke<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStroke(&*(&stroke as *const <InkStroke as ::windows::core::Abi>::Abi as *const <InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteSelected<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MoveSelected<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectWithPolyLine<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, polyline: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectWithLine<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CopySelectedToClipboard<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySelectedToClipboard().into()
        }
        unsafe extern "system" fn PasteFromClipboard<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanPasteFromClipboard<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadAsync<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SaveAsync<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateRecognitionResults<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateRecognitionResults(&*(&recognitionresults as *const <super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStrokes<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRecognitionResults<Impl: IInkStrokeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeContainer2Impl: Sized {
    fn AddStrokes(&self, strokes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkStroke>>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeContainer2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokeContainer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeContainer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeContainer2Vtbl {
        unsafe extern "system" fn AddStrokes<Impl: IInkStrokeContainer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStrokes(&*(&strokes as *const <super::super::super::Foundation::Collections::IIterable<InkStroke> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InkStroke> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IInkStrokeContainer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeContainer3Impl: Sized {
    fn SaveWithFormatAsync(&self, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>, inkpersistenceformat: InkPersistenceFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn GetStrokeById(&self, id: u32) -> ::windows::core::Result<InkStroke>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeContainer3 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeContainer3";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IInkStrokeContainer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeContainer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeContainer3Vtbl {
        unsafe extern "system" fn SaveWithFormatAsync<Impl: IInkStrokeContainer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, inkpersistenceformat: InkPersistenceFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStrokeById<Impl: IInkStrokeContainer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeInputImpl: Sized {
    fn StrokeStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeStarted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeContinued(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeContinued(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeEnded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeEnded(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeCanceled(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeInput";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkStrokeInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeInputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeInputVtbl {
        unsafe extern "system" fn StrokeStarted<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStrokeStarted<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeStarted(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeContinued<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStrokeContinued<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeContinued(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeEnded<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStrokeEnded<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeEnded(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeCanceled<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStrokeCanceled<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStrokeCanceled(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InkPresenter<Impl: IInkStrokeInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokeRenderingSegmentImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn BezierControlPoint1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn BezierControlPoint2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Pressure(&self) -> ::windows::core::Result<f32>;
    fn TiltX(&self) -> ::windows::core::Result<f32>;
    fn TiltY(&self) -> ::windows::core::Result<f32>;
    fn Twist(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokeRenderingSegment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInkStrokeRenderingSegmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeRenderingSegmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeRenderingSegmentVtbl {
        unsafe extern "system" fn Position<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BezierControlPoint1<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BezierControlPoint2<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pressure<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TiltX<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TiltY<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Twist<Impl: IInkStrokeRenderingSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IInkStrokesCollectedEventArgsImpl: Sized {
    fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokesCollectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokesCollectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokesCollectedEventArgsVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkStrokesCollectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkStrokesErasedEventArgsImpl: Sized {
    fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkStrokesErasedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkStrokesErasedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokesErasedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokesErasedEventArgsVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkStrokesErasedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInkSynchronizerImpl: Sized {
    fn BeginDry(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn EndDry(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkSynchronizer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkSynchronizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkSynchronizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkSynchronizerVtbl {
        unsafe extern "system" fn BeginDry<Impl: IInkSynchronizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndDry<Impl: IInkSynchronizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInkUnprocessedInputImpl: Sized {
    fn PointerEntered(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerHovered(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerHovered(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerLost(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerLost(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.IInkUnprocessedInput";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IInkUnprocessedInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkUnprocessedInputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkUnprocessedInputVtbl {
        unsafe extern "system" fn PointerEntered<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerEntered<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerEntered(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerHovered<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerHovered<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerHovered(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExited<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerExited<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerExited(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressed<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerPressed<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerPressed(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoved<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerMoved<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerMoved(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleased<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerReleased<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerReleased(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerLost<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePointerLost<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerLost(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InkPresenter<Impl: IInkUnprocessedInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPenAndInkSettingsImpl: Sized {
    fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> ::windows::core::Result<bool>;
    fn PenHandedness(&self) -> ::windows::core::Result<PenHandedness>;
    fn HandwritingLineHeight(&self) -> ::windows::core::Result<HandwritingLineHeight>;
    fn FontFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserConsentsToHandwritingTelemetryCollection(&self) -> ::windows::core::Result<bool>;
    fn IsTouchHandwritingEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.IPenAndInkSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IPenAndInkSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenAndInkSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenAndInkSettingsVtbl {
        unsafe extern "system" fn IsHandwritingDirectlyIntoTextFieldEnabled<Impl: IPenAndInkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PenHandedness<Impl: IPenAndInkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PenHandedness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HandwritingLineHeight<Impl: IPenAndInkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HandwritingLineHeight) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FontFamilyName<Impl: IPenAndInkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UserConsentsToHandwritingTelemetryCollection<Impl: IPenAndInkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTouchHandwritingEnabled<Impl: IPenAndInkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPenAndInkSettings2Impl: Sized {
    fn SetPenHandedness(&self, value: PenHandedness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenAndInkSettings2 {
    const NAME: &'static str = "Windows.UI.Input.Inking.IPenAndInkSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IPenAndInkSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenAndInkSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenAndInkSettings2Vtbl {
        unsafe extern "system" fn SetPenHandedness<Impl: IPenAndInkSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PenHandedness) -> ::windows::core::HRESULT {
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
pub trait IPenAndInkSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PenAndInkSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPenAndInkSettingsStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.IPenAndInkSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPenAndInkSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenAndInkSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenAndInkSettingsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IPenAndInkSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
