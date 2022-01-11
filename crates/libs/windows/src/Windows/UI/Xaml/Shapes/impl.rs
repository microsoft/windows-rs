#[cfg(feature = "implement_exclusive")]
pub trait IEllipseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEllipse {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IEllipse";
}
#[cfg(feature = "implement_exclusive")]
impl IEllipseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEllipseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEllipseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEllipse>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEllipse as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineImpl: Sized {
    fn X1(&self) -> ::windows::core::Result<f64>;
    fn SetX1(&self, value: f64) -> ::windows::core::Result<()>;
    fn Y1(&self) -> ::windows::core::Result<f64>;
    fn SetY1(&self, value: f64) -> ::windows::core::Result<()>;
    fn X2(&self) -> ::windows::core::Result<f64>;
    fn SetX2(&self, value: f64) -> ::windows::core::Result<()>;
    fn Y2(&self) -> ::windows::core::Result<f64>;
    fn SetY2(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILine {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.ILine";
}
#[cfg(feature = "implement_exclusive")]
impl ILineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineVtbl {
        unsafe extern "system" fn X1<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetX1<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetX1(value).into()
        }
        unsafe extern "system" fn Y1<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetY1<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetY1(value).into()
        }
        unsafe extern "system" fn X2<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetX2<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetX2(value).into()
        }
        unsafe extern "system" fn Y2<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetY2<Impl: ILineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetY2(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILine>, ::windows::core::GetTrustLevel, X1::<Impl, IMPL_OFFSET>, SetX1::<Impl, IMPL_OFFSET>, Y1::<Impl, IMPL_OFFSET>, SetY1::<Impl, IMPL_OFFSET>, X2::<Impl, IMPL_OFFSET>, SetX2::<Impl, IMPL_OFFSET>, Y2::<Impl, IMPL_OFFSET>, SetY2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineStaticsImpl: Sized {
    fn X1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Y1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn X2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Y2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.ILineStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILineStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineStaticsVtbl {
        unsafe extern "system" fn X1Property<Impl: ILineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y1Property<Impl: ILineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X2Property<Impl: ILineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y2Property<Impl: ILineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILineStatics>, ::windows::core::GetTrustLevel, X1Property::<Impl, IMPL_OFFSET>, Y1Property::<Impl, IMPL_OFFSET>, X2Property::<Impl, IMPL_OFFSET>, Y2Property::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IPathImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::Media::Geometry>;
    fn SetData(&self, value: &::core::option::Option<super::Media::Geometry>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPath {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPath";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathVtbl {
        unsafe extern "system" fn Data<Impl: IPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::Media::Geometry as ::windows::core::Abi>::Abi as *const <super::Media::Geometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPath>, ::windows::core::GetTrustLevel, Data::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPath as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Path>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPathFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPathFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPathFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPathFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathStaticsImpl: Sized {
    fn DataProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPathStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPathStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathStaticsVtbl {
        unsafe extern "system" fn DataProperty<Impl: IPathStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPathStatics>, ::windows::core::GetTrustLevel, DataProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IPolygonImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule>;
    fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()>;
    fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<super::Media::PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPolygon {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPolygon";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IPolygonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolygonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolygonVtbl {
        unsafe extern "system" fn FillRule<Impl: IPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillRule(value).into()
        }
        unsafe extern "system" fn Points<Impl: IPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Impl: IPolygonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <super::Media::PointCollection as ::windows::core::Abi>::Abi as *const <super::Media::PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPolygon>, ::windows::core::GetTrustLevel, FillRule::<Impl, IMPL_OFFSET>, SetFillRule::<Impl, IMPL_OFFSET>, Points::<Impl, IMPL_OFFSET>, SetPoints::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolygon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolygonStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolygonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPolygonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolygonStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolygonStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolygonStaticsVtbl {
        unsafe extern "system" fn FillRuleProperty<Impl: IPolygonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillRuleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointsProperty<Impl: IPolygonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPolygonStatics>, ::windows::core::GetTrustLevel, FillRuleProperty::<Impl, IMPL_OFFSET>, PointsProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolygonStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IPolylineImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule>;
    fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()>;
    fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<super::Media::PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPolyline {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPolyline";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IPolylineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolylineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolylineVtbl {
        unsafe extern "system" fn FillRule<Impl: IPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillRule(value).into()
        }
        unsafe extern "system" fn Points<Impl: IPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Impl: IPolylineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <super::Media::PointCollection as ::windows::core::Abi>::Abi as *const <super::Media::PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPolyline>, ::windows::core::GetTrustLevel, FillRule::<Impl, IMPL_OFFSET>, SetFillRule::<Impl, IMPL_OFFSET>, Points::<Impl, IMPL_OFFSET>, SetPoints::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyline as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolylineStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolylineStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IPolylineStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolylineStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolylineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolylineStaticsVtbl {
        unsafe extern "system" fn FillRuleProperty<Impl: IPolylineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillRuleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointsProperty<Impl: IPolylineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPolylineStatics>, ::windows::core::GetTrustLevel, FillRuleProperty::<Impl, IMPL_OFFSET>, PointsProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolylineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleImpl: Sized {
    fn RadiusX(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RadiusY(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectangle {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IRectangle";
}
#[cfg(feature = "implement_exclusive")]
impl IRectangleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectangleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectangleVtbl {
        unsafe extern "system" fn RadiusX<Impl: IRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadiusX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiusX<Impl: IRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadiusX(value).into()
        }
        unsafe extern "system" fn RadiusY<Impl: IRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadiusY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiusY<Impl: IRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadiusY(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRectangle>, ::windows::core::GetTrustLevel, RadiusX::<Impl, IMPL_OFFSET>, SetRadiusX::<Impl, IMPL_OFFSET>, RadiusY::<Impl, IMPL_OFFSET>, SetRadiusY::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectangle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleStaticsImpl: Sized {
    fn RadiusXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectangleStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IRectangleStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRectangleStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectangleStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectangleStaticsVtbl {
        unsafe extern "system" fn RadiusXProperty<Impl: IRectangleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadiusXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RadiusYProperty<Impl: IRectangleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadiusYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRectangleStatics>, ::windows::core::GetTrustLevel, RadiusXProperty::<Impl, IMPL_OFFSET>, RadiusYProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectangleStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IShapeImpl: Sized {
    fn Fill(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Stroke(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetStroke(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn StrokeMiterLimit(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeMiterLimit(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeStartLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap>;
    fn SetStrokeStartLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()>;
    fn StrokeEndLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap>;
    fn SetStrokeEndLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()>;
    fn StrokeLineJoin(&self) -> ::windows::core::Result<super::Media::PenLineJoin>;
    fn SetStrokeLineJoin(&self, value: super::Media::PenLineJoin) -> ::windows::core::Result<()>;
    fn StrokeDashOffset(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeDashOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashCap(&self) -> ::windows::core::Result<super::Media::PenLineCap>;
    fn SetStrokeDashCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()>;
    fn StrokeDashArray(&self) -> ::windows::core::Result<super::Media::DoubleCollection>;
    fn SetStrokeDashArray(&self, value: &::core::option::Option<super::Media::DoubleCollection>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn GeometryTransform(&self) -> ::windows::core::Result<super::Media::Transform>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShape {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IShape";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IShapeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShapeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShapeVtbl {
        unsafe extern "system" fn Fill<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fill() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFill<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFill(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stroke<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stroke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStroke<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStroke(&*(&value as *const <super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeMiterLimit<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeMiterLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeMiterLimit(value).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        unsafe extern "system" fn StrokeStartLineCap<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeStartLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::PenLineCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartLineCap(value).into()
        }
        unsafe extern "system" fn StrokeEndLineCap<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeEndLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::PenLineCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeEndLineCap(value).into()
        }
        unsafe extern "system" fn StrokeLineJoin<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineJoin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeLineJoin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::PenLineJoin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeLineJoin(value).into()
        }
        unsafe extern "system" fn StrokeDashOffset<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashOffset(value).into()
        }
        unsafe extern "system" fn StrokeDashCap<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::PenLineCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::PenLineCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashCap(value).into()
        }
        unsafe extern "system" fn StrokeDashArray<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashArray<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashArray(&*(&value as *const <super::Media::DoubleCollection as ::windows::core::Abi>::Abi as *const <super::Media::DoubleCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stretch<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Media::Stretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStretch<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Media::Stretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        unsafe extern "system" fn GeometryTransform<Impl: IShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeometryTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IShape>,
            ::windows::core::GetTrustLevel,
            Fill::<Impl, IMPL_OFFSET>,
            SetFill::<Impl, IMPL_OFFSET>,
            Stroke::<Impl, IMPL_OFFSET>,
            SetStroke::<Impl, IMPL_OFFSET>,
            StrokeMiterLimit::<Impl, IMPL_OFFSET>,
            SetStrokeMiterLimit::<Impl, IMPL_OFFSET>,
            StrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness::<Impl, IMPL_OFFSET>,
            StrokeStartLineCap::<Impl, IMPL_OFFSET>,
            SetStrokeStartLineCap::<Impl, IMPL_OFFSET>,
            StrokeEndLineCap::<Impl, IMPL_OFFSET>,
            SetStrokeEndLineCap::<Impl, IMPL_OFFSET>,
            StrokeLineJoin::<Impl, IMPL_OFFSET>,
            SetStrokeLineJoin::<Impl, IMPL_OFFSET>,
            StrokeDashOffset::<Impl, IMPL_OFFSET>,
            SetStrokeDashOffset::<Impl, IMPL_OFFSET>,
            StrokeDashCap::<Impl, IMPL_OFFSET>,
            SetStrokeDashCap::<Impl, IMPL_OFFSET>,
            StrokeDashArray::<Impl, IMPL_OFFSET>,
            SetStrokeDashArray::<Impl, IMPL_OFFSET>,
            Stretch::<Impl, IMPL_OFFSET>,
            SetStretch::<Impl, IMPL_OFFSET>,
            GeometryTransform::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShape as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IShape2Impl: Sized {
    fn GetAlphaMask(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShape2 {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IShape2";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IShape2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShape2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShape2Vtbl {
        unsafe extern "system" fn GetAlphaMask<Impl: IShape2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlphaMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShape2>, ::windows::core::GetTrustLevel, GetAlphaMask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShape2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShapeFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Shape>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShapeFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IShapeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IShapeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShapeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShapeFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IShapeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShapeFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShapeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShapeStaticsImpl: Sized {
    fn FillProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeMiterLimitProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeStartLineCapProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeEndLineCapProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeLineJoinProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeDashOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeDashCapProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeDashArrayProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShapeStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.IShapeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IShapeStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShapeStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShapeStaticsVtbl {
        unsafe extern "system" fn FillProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeMiterLimitProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeMiterLimitProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeThicknessProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeStartLineCapProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeStartLineCapProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeEndLineCapProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeEndLineCapProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeLineJoinProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeLineJoinProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashOffsetProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashCapProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashCapProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashArrayProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashArrayProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StretchProperty<Impl: IShapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StretchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IShapeStatics>,
            ::windows::core::GetTrustLevel,
            FillProperty::<Impl, IMPL_OFFSET>,
            StrokeProperty::<Impl, IMPL_OFFSET>,
            StrokeMiterLimitProperty::<Impl, IMPL_OFFSET>,
            StrokeThicknessProperty::<Impl, IMPL_OFFSET>,
            StrokeStartLineCapProperty::<Impl, IMPL_OFFSET>,
            StrokeEndLineCapProperty::<Impl, IMPL_OFFSET>,
            StrokeLineJoinProperty::<Impl, IMPL_OFFSET>,
            StrokeDashOffsetProperty::<Impl, IMPL_OFFSET>,
            StrokeDashCapProperty::<Impl, IMPL_OFFSET>,
            StrokeDashArrayProperty::<Impl, IMPL_OFFSET>,
            StretchProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShapeStatics as ::windows::core::Interface>::IID
    }
}
