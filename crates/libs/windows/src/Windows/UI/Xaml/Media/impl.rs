#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAcrylicBrush_Impl: Sized {
    fn BackgroundSource(&mut self) -> ::windows::core::Result<AcrylicBackgroundSource>;
    fn SetBackgroundSource(&mut self, value: AcrylicBackgroundSource) -> ::windows::core::Result<()>;
    fn TintColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetTintColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn TintOpacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetTintOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn TintTransitionDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTintTransitionDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AlwaysUseFallback(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlwaysUseFallback(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAcrylicBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrush";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAcrylicBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcrylicBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcrylicBrush_Vtbl {
        unsafe extern "system" fn BackgroundSource<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AcrylicBackgroundSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundSource<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AcrylicBackgroundSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundSource(value).into()
        }
        unsafe extern "system" fn TintColor<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintColor<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTintColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TintOpacity<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintOpacity<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTintOpacity(value).into()
        }
        unsafe extern "system" fn TintTransitionDuration<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintTransitionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintTransitionDuration<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTintTransitionDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlwaysUseFallback<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysUseFallback<Impl: IAcrylicBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysUseFallback(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAcrylicBrush, BASE_OFFSET>(),
            BackgroundSource: BackgroundSource::<Impl, IMPL_OFFSET>,
            SetBackgroundSource: SetBackgroundSource::<Impl, IMPL_OFFSET>,
            TintColor: TintColor::<Impl, IMPL_OFFSET>,
            SetTintColor: SetTintColor::<Impl, IMPL_OFFSET>,
            TintOpacity: TintOpacity::<Impl, IMPL_OFFSET>,
            SetTintOpacity: SetTintOpacity::<Impl, IMPL_OFFSET>,
            TintTransitionDuration: TintTransitionDuration::<Impl, IMPL_OFFSET>,
            SetTintTransitionDuration: SetTintTransitionDuration::<Impl, IMPL_OFFSET>,
            AlwaysUseFallback: AlwaysUseFallback::<Impl, IMPL_OFFSET>,
            SetAlwaysUseFallback: SetAlwaysUseFallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcrylicBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAcrylicBrush2_Impl: Sized {
    fn TintLuminosityOpacity(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetTintLuminosityOpacity(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAcrylicBrush2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrush2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAcrylicBrush2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcrylicBrush2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcrylicBrush2_Vtbl {
        unsafe extern "system" fn TintLuminosityOpacity<Impl: IAcrylicBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintLuminosityOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintLuminosityOpacity<Impl: IAcrylicBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTintLuminosityOpacity(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAcrylicBrush2, BASE_OFFSET>(),
            TintLuminosityOpacity: TintLuminosityOpacity::<Impl, IMPL_OFFSET>,
            SetTintLuminosityOpacity: SetTintLuminosityOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcrylicBrush2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AcrylicBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcrylicBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcrylicBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAcrylicBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAcrylicBrushFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcrylicBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushStatics_Impl: Sized {
    fn BackgroundSourceProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintColorProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintOpacityProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintTransitionDurationProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysUseFallbackProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcrylicBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcrylicBrushStatics_Vtbl {
        unsafe extern "system" fn BackgroundSourceProperty<Impl: IAcrylicBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TintColorProperty<Impl: IAcrylicBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TintOpacityProperty<Impl: IAcrylicBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TintTransitionDurationProperty<Impl: IAcrylicBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintTransitionDurationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlwaysUseFallbackProperty<Impl: IAcrylicBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallbackProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAcrylicBrushStatics, BASE_OFFSET>(),
            BackgroundSourceProperty: BackgroundSourceProperty::<Impl, IMPL_OFFSET>,
            TintColorProperty: TintColorProperty::<Impl, IMPL_OFFSET>,
            TintOpacityProperty: TintOpacityProperty::<Impl, IMPL_OFFSET>,
            TintTransitionDurationProperty: TintTransitionDurationProperty::<Impl, IMPL_OFFSET>,
            AlwaysUseFallbackProperty: AlwaysUseFallbackProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcrylicBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushStatics2_Impl: Sized {
    fn TintLuminosityOpacityProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrushStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrushStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcrylicBrushStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcrylicBrushStatics2_Vtbl {
        unsafe extern "system" fn TintLuminosityOpacityProperty<Impl: IAcrylicBrushStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TintLuminosityOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAcrylicBrushStatics2, BASE_OFFSET>(),
            TintLuminosityOpacityProperty: TintLuminosityOpacityProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcrylicBrushStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IArcSegment_Impl: Sized {
    fn Point(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn SetSize(&mut self, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn RotationAngle(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotationAngle(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn IsLargeArc(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsLargeArc(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SweepDirection(&mut self) -> ::windows::core::Result<SweepDirection>;
    fn SetSweepDirection(&mut self, value: SweepDirection) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IArcSegment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IArcSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcSegment_Vtbl {
        unsafe extern "system" fn Point<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn IsLargeArc<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLargeArc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLargeArc<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLargeArc(value).into()
        }
        unsafe extern "system" fn SweepDirection<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SweepDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SweepDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSweepDirection<Impl: IArcSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SweepDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSweepDirection(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IArcSegment, BASE_OFFSET>(),
            Point: Point::<Impl, IMPL_OFFSET>,
            SetPoint: SetPoint::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            RotationAngle: RotationAngle::<Impl, IMPL_OFFSET>,
            SetRotationAngle: SetRotationAngle::<Impl, IMPL_OFFSET>,
            IsLargeArc: IsLargeArc::<Impl, IMPL_OFFSET>,
            SetIsLargeArc: SetIsLargeArc::<Impl, IMPL_OFFSET>,
            SweepDirection: SweepDirection::<Impl, IMPL_OFFSET>,
            SetSweepDirection: SetSweepDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IArcSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcSegmentStatics_Impl: Sized {
    fn PointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SizeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationAngleProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsLargeArcProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SweepDirectionProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IArcSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IArcSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IArcSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcSegmentStatics_Vtbl {
        unsafe extern "system" fn PointProperty<Impl: IArcSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeProperty<Impl: IArcSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationAngleProperty<Impl: IArcSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLargeArcProperty<Impl: IArcSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLargeArcProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SweepDirectionProperty<Impl: IArcSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SweepDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IArcSegmentStatics, BASE_OFFSET>(),
            PointProperty: PointProperty::<Impl, IMPL_OFFSET>,
            SizeProperty: SizeProperty::<Impl, IMPL_OFFSET>,
            RotationAngleProperty: RotationAngleProperty::<Impl, IMPL_OFFSET>,
            IsLargeArcProperty: IsLargeArcProperty::<Impl, IMPL_OFFSET>,
            SweepDirectionProperty: SweepDirectionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IArcSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBezierSegment_Impl: Sized {
    fn Point1(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint1(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point2(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint2(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point3(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint3(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBezierSegment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBezierSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBezierSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBezierSegment_Vtbl {
        unsafe extern "system" fn Point1<Impl: IBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint1<Impl: IBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint1(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point2<Impl: IBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint2<Impl: IBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint2(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point3<Impl: IBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint3<Impl: IBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint3(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBezierSegment, BASE_OFFSET>(),
            Point1: Point1::<Impl, IMPL_OFFSET>,
            SetPoint1: SetPoint1::<Impl, IMPL_OFFSET>,
            Point2: Point2::<Impl, IMPL_OFFSET>,
            SetPoint2: SetPoint2::<Impl, IMPL_OFFSET>,
            Point3: Point3::<Impl, IMPL_OFFSET>,
            SetPoint3: SetPoint3::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBezierSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBezierSegmentStatics_Impl: Sized {
    fn Point1Property(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point2Property(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point3Property(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBezierSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBezierSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBezierSegmentStatics_Vtbl {
        unsafe extern "system" fn Point1Property<Impl: IBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point2Property<Impl: IBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point3Property<Impl: IBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point3Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBezierSegmentStatics, BASE_OFFSET>(),
            Point1Property: Point1Property::<Impl, IMPL_OFFSET>,
            Point2Property: Point2Property::<Impl, IMPL_OFFSET>,
            Point3Property: Point3Property::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBezierSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapCache_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapCache {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBitmapCache";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapCache_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapCache_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBitmapCache, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrush_Impl: Sized {
    fn Opacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Transform(&mut self) -> ::windows::core::Result<Transform>;
    fn SetTransform(&mut self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
    fn RelativeTransform(&mut self) -> ::windows::core::Result<Transform>;
    fn SetRelativeTransform(&mut self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrush_Vtbl {
        unsafe extern "system" fn Opacity<Impl: IBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpacity<Impl: IBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Transform<Impl: IBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransform<Impl: IBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&value as *const <Transform as ::windows::core::Abi>::Abi as *const <Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeTransform<Impl: IBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTransform<Impl: IBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeTransform(&*(&value as *const <Transform as ::windows::core::Abi>::Abi as *const <Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrush, BASE_OFFSET>(),
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            Transform: Transform::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            RelativeTransform: RelativeTransform::<Impl, IMPL_OFFSET>,
            SetRelativeTransform: SetRelativeTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Brush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBrushFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IBrushOverrides2_Impl: Sized {
    fn PopulatePropertyInfoOverride(&mut self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBrushOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushOverrides2";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IBrushOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrushOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrushOverrides2_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Impl: IBrushOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfoOverride(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animationpropertyinfo as *const <super::super::Composition::AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <super::super::Composition::AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrushOverrides2, BASE_OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushStatics_Impl: Sized {
    fn OpacityProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TransformProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RelativeTransformProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrushStatics_Vtbl {
        unsafe extern "system" fn OpacityProperty<Impl: IBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformProperty<Impl: IBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTransformProperty<Impl: IBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrushStatics, BASE_OFFSET>(),
            OpacityProperty: OpacityProperty::<Impl, IMPL_OFFSET>,
            TransformProperty: TransformProperty::<Impl, IMPL_OFFSET>,
            RelativeTransformProperty: RelativeTransformProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICacheMode_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICacheMode {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICacheMode";
}
#[cfg(feature = "implement_exclusive")]
impl ICacheMode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICacheMode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICacheMode_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICacheMode, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICacheMode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICacheModeFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CacheMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICacheModeFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICacheModeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICacheModeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICacheModeFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICacheModeFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICacheModeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICacheModeFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICacheModeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransform_Impl: Sized {
    fn CenterX(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&mut self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&mut self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SkewX(&mut self) -> ::windows::core::Result<f64>;
    fn SetSkewX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SkewY(&mut self) -> ::windows::core::Result<f64>;
    fn SetSkewY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Rotation(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotation(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateX(&mut self) -> ::windows::core::Result<f64>;
    fn SetTranslateX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateY(&mut self) -> ::windows::core::Result<f64>;
    fn SetTranslateY(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositeTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositeTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositeTransform_Vtbl {
        unsafe extern "system" fn CenterX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn ScaleX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(value).into()
        }
        unsafe extern "system" fn ScaleY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(value).into()
        }
        unsafe extern "system" fn SkewX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkewX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSkewX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSkewX(value).into()
        }
        unsafe extern "system" fn SkewY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkewY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSkewY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSkewY(value).into()
        }
        unsafe extern "system" fn Rotation<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn TranslateX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateX<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslateX(value).into()
        }
        unsafe extern "system" fn TranslateY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateY<Impl: ICompositeTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslateY(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositeTransform, BASE_OFFSET>(),
            CenterX: CenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            CenterY: CenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            ScaleX: ScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            ScaleY: ScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SkewX: SkewX::<Impl, IMPL_OFFSET>,
            SetSkewX: SetSkewX::<Impl, IMPL_OFFSET>,
            SkewY: SkewY::<Impl, IMPL_OFFSET>,
            SetSkewY: SetSkewY::<Impl, IMPL_OFFSET>,
            Rotation: Rotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            TranslateX: TranslateX::<Impl, IMPL_OFFSET>,
            SetTranslateX: SetTranslateX::<Impl, IMPL_OFFSET>,
            TranslateY: TranslateY::<Impl, IMPL_OFFSET>,
            SetTranslateY: SetTranslateY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositeTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransformStatics_Impl: Sized {
    fn CenterXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SkewXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SkewYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TranslateXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TranslateYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositeTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeTransformStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositeTransformStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositeTransformStatics_Vtbl {
        unsafe extern "system" fn CenterXProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleXProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleYProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkewXProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkewXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkewYProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkewYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateXProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateYProperty<Impl: ICompositeTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositeTransformStatics, BASE_OFFSET>(),
            CenterXProperty: CenterXProperty::<Impl, IMPL_OFFSET>,
            CenterYProperty: CenterYProperty::<Impl, IMPL_OFFSET>,
            ScaleXProperty: ScaleXProperty::<Impl, IMPL_OFFSET>,
            ScaleYProperty: ScaleYProperty::<Impl, IMPL_OFFSET>,
            SkewXProperty: SkewXProperty::<Impl, IMPL_OFFSET>,
            SkewYProperty: SkewYProperty::<Impl, IMPL_OFFSET>,
            RotationProperty: RotationProperty::<Impl, IMPL_OFFSET>,
            TranslateXProperty: TranslateXProperty::<Impl, IMPL_OFFSET>,
            TranslateYProperty: TranslateYProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositeTransformStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTarget_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTarget {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositionTarget";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTarget_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTarget, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionTargetStatics_Impl: Sized {
    fn Rendering(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRendering(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SurfaceContentsLost(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSurfaceContentsLost(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionTargetStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositionTargetStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionTargetStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTargetStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTargetStatics_Vtbl {
        unsafe extern "system" fn Rendering<Impl: ICompositionTargetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rendering(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRendering<Impl: ICompositionTargetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRendering(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SurfaceContentsLost<Impl: ICompositionTargetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SurfaceContentsLost(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSurfaceContentsLost<Impl: ICompositionTargetStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSurfaceContentsLost(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTargetStatics, BASE_OFFSET>(),
            Rendering: Rendering::<Impl, IMPL_OFFSET>,
            RemoveRendering: RemoveRendering::<Impl, IMPL_OFFSET>,
            SurfaceContentsLost: SurfaceContentsLost::<Impl, IMPL_OFFSET>,
            RemoveSurfaceContentsLost: RemoveSurfaceContentsLost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTargetStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionTargetStatics3_Impl: Sized {
    fn Rendered(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<RenderedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRendered(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionTargetStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositionTargetStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionTargetStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTargetStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTargetStatics3_Vtbl {
        unsafe extern "system" fn Rendered<Impl: ICompositionTargetStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rendered(&*(&handler as *const <super::super::super::Foundation::EventHandler<RenderedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<RenderedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRendered<Impl: ICompositionTargetStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRendered(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTargetStatics3, BASE_OFFSET>(),
            Rendered: Rendered::<Impl, IMPL_OFFSET>,
            RemoveRendered: RemoveRendered::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTargetStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEllipseGeometry_Impl: Sized {
    fn Center(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetCenter(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RadiusX(&mut self) -> ::windows::core::Result<f64>;
    fn SetRadiusX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RadiusY(&mut self) -> ::windows::core::Result<f64>;
    fn SetRadiusY(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEllipseGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IEllipseGeometry";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEllipseGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEllipseGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEllipseGeometry_Vtbl {
        unsafe extern "system" fn Center<Impl: IEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RadiusX<Impl: IEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRadiusX<Impl: IEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadiusX(value).into()
        }
        unsafe extern "system" fn RadiusY<Impl: IEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRadiusY<Impl: IEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadiusY(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEllipseGeometry, BASE_OFFSET>(),
            Center: Center::<Impl, IMPL_OFFSET>,
            SetCenter: SetCenter::<Impl, IMPL_OFFSET>,
            RadiusX: RadiusX::<Impl, IMPL_OFFSET>,
            SetRadiusX: SetRadiusX::<Impl, IMPL_OFFSET>,
            RadiusY: RadiusY::<Impl, IMPL_OFFSET>,
            SetRadiusY: SetRadiusY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEllipseGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEllipseGeometryStatics_Impl: Sized {
    fn CenterProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEllipseGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IEllipseGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEllipseGeometryStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEllipseGeometryStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEllipseGeometryStatics_Vtbl {
        unsafe extern "system" fn CenterProperty<Impl: IEllipseGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RadiusXProperty<Impl: IEllipseGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RadiusYProperty<Impl: IEllipseGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEllipseGeometryStatics, BASE_OFFSET>(),
            CenterProperty: CenterProperty::<Impl, IMPL_OFFSET>,
            RadiusXProperty: RadiusXProperty::<Impl, IMPL_OFFSET>,
            RadiusYProperty: RadiusYProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEllipseGeometryStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamily_Impl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontFamily {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IFontFamily";
}
#[cfg(feature = "implement_exclusive")]
impl IFontFamily_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontFamily_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontFamily_Vtbl {
        unsafe extern "system" fn Source<Impl: IFontFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFontFamily, BASE_OFFSET>(), Source: Source::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontFamily as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyFactory_Impl: Sized {
    fn CreateInstanceWithName(&mut self, familyname: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FontFamily>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontFamilyFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IFontFamilyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFontFamilyFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontFamilyFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontFamilyFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithName<Impl: IFontFamilyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithName(&*(&familyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFontFamilyFactory, BASE_OFFSET>(),
            CreateInstanceWithName: CreateInstanceWithName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontFamilyFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyStatics2_Impl: Sized {
    fn XamlAutoFontFamily(&mut self) -> ::windows::core::Result<FontFamily>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontFamilyStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IFontFamilyStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFontFamilyStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontFamilyStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontFamilyStatics2_Vtbl {
        unsafe extern "system" fn XamlAutoFontFamily<Impl: IFontFamilyStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XamlAutoFontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFontFamilyStatics2, BASE_OFFSET>(),
            XamlAutoFontFamily: XamlAutoFontFamily::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontFamilyStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeneralTransform_Impl: Sized {
    fn Inverse(&mut self) -> ::windows::core::Result<GeneralTransform>;
    fn TransformPoint(&mut self, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn TryTransform(&mut self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&mut self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeneralTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransform";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeneralTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneralTransform_Vtbl {
        unsafe extern "system" fn Inverse<Impl: IGeneralTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformPoint<Impl: IGeneralTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformPoint(&*(&point as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<Impl: IGeneralTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTransform(&*(&inpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBounds<Impl: IGeneralTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformBounds(&*(&rect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneralTransform, BASE_OFFSET>(),
            Inverse: Inverse::<Impl, IMPL_OFFSET>,
            TransformPoint: TransformPoint::<Impl, IMPL_OFFSET>,
            TryTransform: TryTransform::<Impl, IMPL_OFFSET>,
            TransformBounds: TransformBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneralTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GeneralTransform>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneralTransformFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransformFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneralTransformFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransformFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneralTransformFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGeneralTransformFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneralTransformFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneralTransformFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeneralTransformOverrides_Impl: Sized {
    fn InverseCore(&mut self) -> ::windows::core::Result<GeneralTransform>;
    fn TryTransformCore(&mut self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBoundsCore(&mut self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeneralTransformOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransformOverrides";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeneralTransformOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransformOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneralTransformOverrides_Vtbl {
        unsafe extern "system" fn InverseCore<Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InverseCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformCore<Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTransformCore(&*(&inpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBoundsCore<Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformBoundsCore(&*(&rect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneralTransformOverrides, BASE_OFFSET>(),
            InverseCore: InverseCore::<Impl, IMPL_OFFSET>,
            TryTransformCore: TryTransformCore::<Impl, IMPL_OFFSET>,
            TransformBoundsCore: TransformBoundsCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneralTransformOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeometry_Impl: Sized {
    fn Transform(&mut self) -> ::windows::core::Result<Transform>;
    fn SetTransform(&mut self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
    fn Bounds(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometry";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometry_Vtbl {
        unsafe extern "system" fn Transform<Impl: IGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransform<Impl: IGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&value as *const <Transform as ::windows::core::Abi>::Abi as *const <Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bounds<Impl: IGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeometry, BASE_OFFSET>(),
            Transform: Transform::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometryFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometryFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeometryFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGeometryGroup_Impl: Sized {
    fn FillRule(&mut self) -> ::windows::core::Result<FillRule>;
    fn SetFillRule(&mut self, value: FillRule) -> ::windows::core::Result<()>;
    fn Children(&mut self) -> ::windows::core::Result<GeometryCollection>;
    fn SetChildren(&mut self, value: &::core::option::Option<GeometryCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeometryGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGeometryGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometryGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometryGroup_Vtbl {
        unsafe extern "system" fn FillRule<Impl: IGeometryGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FillRule) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFillRule<Impl: IGeometryGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillRule(value).into()
        }
        unsafe extern "system" fn Children<Impl: IGeometryGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildren<Impl: IGeometryGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChildren(&*(&value as *const <GeometryCollection as ::windows::core::Abi>::Abi as *const <GeometryCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeometryGroup, BASE_OFFSET>(),
            FillRule: FillRule::<Impl, IMPL_OFFSET>,
            SetFillRule: SetFillRule::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            SetChildren: SetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometryGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryGroupStatics_Impl: Sized {
    fn FillRuleProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChildrenProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryGroupStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryGroupStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometryGroupStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometryGroupStatics_Vtbl {
        unsafe extern "system" fn FillRuleProperty<Impl: IGeometryGroupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChildrenProperty<Impl: IGeometryGroupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildrenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeometryGroupStatics, BASE_OFFSET>(),
            FillRuleProperty: FillRuleProperty::<Impl, IMPL_OFFSET>,
            ChildrenProperty: ChildrenProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometryGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryStatics_Impl: Sized {
    fn Empty(&mut self) -> ::windows::core::Result<Geometry>;
    fn StandardFlatteningTolerance(&mut self) -> ::windows::core::Result<f64>;
    fn TransformProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometryStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometryStatics_Vtbl {
        unsafe extern "system" fn Empty<Impl: IGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StandardFlatteningTolerance<Impl: IGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StandardFlatteningTolerance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformProperty<Impl: IGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeometryStatics, BASE_OFFSET>(),
            Empty: Empty::<Impl, IMPL_OFFSET>,
            StandardFlatteningTolerance: StandardFlatteningTolerance::<Impl, IMPL_OFFSET>,
            TransformProperty: TransformProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometryStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGradientBrush_Impl: Sized {
    fn SpreadMethod(&mut self) -> ::windows::core::Result<GradientSpreadMethod>;
    fn SetSpreadMethod(&mut self, value: GradientSpreadMethod) -> ::windows::core::Result<()>;
    fn MappingMode(&mut self) -> ::windows::core::Result<BrushMappingMode>;
    fn SetMappingMode(&mut self, value: BrushMappingMode) -> ::windows::core::Result<()>;
    fn ColorInterpolationMode(&mut self) -> ::windows::core::Result<ColorInterpolationMode>;
    fn SetColorInterpolationMode(&mut self, value: ColorInterpolationMode) -> ::windows::core::Result<()>;
    fn GradientStops(&mut self) -> ::windows::core::Result<GradientStopCollection>;
    fn SetGradientStops(&mut self, value: &::core::option::Option<GradientStopCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientBrush";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGradientBrush_Vtbl {
        unsafe extern "system" fn SpreadMethod<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GradientSpreadMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpreadMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GradientSpreadMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpreadMethod(value).into()
        }
        unsafe extern "system" fn MappingMode<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BrushMappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MappingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMappingMode<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BrushMappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMappingMode(value).into()
        }
        unsafe extern "system" fn ColorInterpolationMode<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ColorInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ColorInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorInterpolationMode(value).into()
        }
        unsafe extern "system" fn GradientStops<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GradientStops() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientStops<Impl: IGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGradientStops(&*(&value as *const <GradientStopCollection as ::windows::core::Abi>::Abi as *const <GradientStopCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGradientBrush, BASE_OFFSET>(),
            SpreadMethod: SpreadMethod::<Impl, IMPL_OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Impl, IMPL_OFFSET>,
            MappingMode: MappingMode::<Impl, IMPL_OFFSET>,
            SetMappingMode: SetMappingMode::<Impl, IMPL_OFFSET>,
            ColorInterpolationMode: ColorInterpolationMode::<Impl, IMPL_OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            GradientStops: GradientStops::<Impl, IMPL_OFFSET>,
            SetGradientStops: SetGradientStops::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGradientBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGradientBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGradientBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGradientBrushFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGradientBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushStatics_Impl: Sized {
    fn SpreadMethodProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MappingModeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorInterpolationModeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GradientStopsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGradientBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGradientBrushStatics_Vtbl {
        unsafe extern "system" fn SpreadMethodProperty<Impl: IGradientBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpreadMethodProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MappingModeProperty<Impl: IGradientBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MappingModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorInterpolationModeProperty<Impl: IGradientBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorInterpolationModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GradientStopsProperty<Impl: IGradientBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GradientStopsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGradientBrushStatics, BASE_OFFSET>(),
            SpreadMethodProperty: SpreadMethodProperty::<Impl, IMPL_OFFSET>,
            MappingModeProperty: MappingModeProperty::<Impl, IMPL_OFFSET>,
            ColorInterpolationModeProperty: ColorInterpolationModeProperty::<Impl, IMPL_OFFSET>,
            GradientStopsProperty: GradientStopsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGradientBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientStop_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<f64>;
    fn SetOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientStop {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientStop";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientStop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGradientStop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGradientStop_Vtbl {
        unsafe extern "system" fn Color<Impl: IGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: IGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGradientStop, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGradientStop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientStopStatics_Impl: Sized {
    fn ColorProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OffsetProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientStopStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientStopStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientStopStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGradientStopStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGradientStopStatics_Vtbl {
        unsafe extern "system" fn ColorProperty<Impl: IGradientStopStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetProperty<Impl: IGradientStopStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGradientStopStatics, BASE_OFFSET>(),
            ColorProperty: ColorProperty::<Impl, IMPL_OFFSET>,
            OffsetProperty: OffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGradientStopStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IImageBrush_Impl: Sized {
    fn ImageSource(&mut self) -> ::windows::core::Result<ImageSource>;
    fn SetImageSource(&mut self, value: &::core::option::Option<ImageSource>) -> ::windows::core::Result<()>;
    fn ImageFailed(&mut self, handler: &::core::option::Option<super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&mut self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageBrush";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IImageBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageBrush_Vtbl {
        unsafe extern "system" fn ImageSource<Impl: IImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageSource<Impl: IImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImageSource(&*(&value as *const <ImageSource as ::windows::core::Abi>::Abi as *const <ImageSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageFailed<Impl: IImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageFailed(&*(&handler as *const <super::ExceptionRoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::ExceptionRoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageFailed<Impl: IImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImageFailed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageOpened<Impl: IImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageOpened(&*(&handler as *const <super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageOpened<Impl: IImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImageOpened(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageBrush, BASE_OFFSET>(),
            ImageSource: ImageSource::<Impl, IMPL_OFFSET>,
            SetImageSource: SetImageSource::<Impl, IMPL_OFFSET>,
            ImageFailed: ImageFailed::<Impl, IMPL_OFFSET>,
            RemoveImageFailed: RemoveImageFailed::<Impl, IMPL_OFFSET>,
            ImageOpened: ImageOpened::<Impl, IMPL_OFFSET>,
            RemoveImageOpened: RemoveImageOpened::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageBrushStatics_Impl: Sized {
    fn ImageSourceProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IImageBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageBrushStatics_Vtbl {
        unsafe extern "system" fn ImageSourceProperty<Impl: IImageBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageBrushStatics, BASE_OFFSET>(),
            ImageSourceProperty: ImageSourceProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageSource_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl IImageSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageSourceFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IImageSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageSourceFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageSourceFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineGeometry_Impl: Sized {
    fn StartPoint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn EndPoint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetEndPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineGeometry";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineGeometry_Vtbl {
        unsafe extern "system" fn StartPoint<Impl: ILineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: ILineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPoint<Impl: ILineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: ILineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineGeometry, BASE_OFFSET>(),
            StartPoint: StartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint: SetStartPoint::<Impl, IMPL_OFFSET>,
            EndPoint: EndPoint::<Impl, IMPL_OFFSET>,
            SetEndPoint: SetEndPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineGeometryStatics_Impl: Sized {
    fn StartPointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn EndPointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILineGeometryStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineGeometryStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineGeometryStatics_Vtbl {
        unsafe extern "system" fn StartPointProperty<Impl: ILineGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPointProperty<Impl: ILineGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineGeometryStatics, BASE_OFFSET>(),
            StartPointProperty: StartPointProperty::<Impl, IMPL_OFFSET>,
            EndPointProperty: EndPointProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineGeometryStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineSegment_Impl: Sized {
    fn Point(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineSegment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineSegment_Vtbl {
        unsafe extern "system" fn Point<Impl: ILineSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint<Impl: ILineSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineSegment, BASE_OFFSET>(),
            Point: Point::<Impl, IMPL_OFFSET>,
            SetPoint: SetPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineSegmentStatics_Impl: Sized {
    fn PointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILineSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineSegmentStatics_Vtbl {
        unsafe extern "system" fn PointProperty<Impl: ILineSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILineSegmentStatics, BASE_OFFSET>(), PointProperty: PointProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILinearGradientBrush_Impl: Sized {
    fn StartPoint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn EndPoint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetEndPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILinearGradientBrush";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILinearGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearGradientBrush_Vtbl {
        unsafe extern "system" fn StartPoint<Impl: ILinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: ILinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPoint<Impl: ILinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: ILinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearGradientBrush, BASE_OFFSET>(),
            StartPoint: StartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint: SetStartPoint::<Impl, IMPL_OFFSET>,
            EndPoint: EndPoint::<Impl, IMPL_OFFSET>,
            SetEndPoint: SetEndPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILinearGradientBrushFactory_Impl: Sized {
    fn CreateInstanceWithGradientStopCollectionAndAngle(&mut self, gradientstopcollection: &::core::option::Option<GradientStopCollection>, angle: f64) -> ::windows::core::Result<LinearGradientBrush>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILinearGradientBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILinearGradientBrushFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILinearGradientBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearGradientBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearGradientBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithGradientStopCollectionAndAngle<Impl: ILinearGradientBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstopcollection: ::windows::core::RawPtr, angle: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithGradientStopCollectionAndAngle(&*(&gradientstopcollection as *const <GradientStopCollection as ::windows::core::Abi>::Abi as *const <GradientStopCollection as ::windows::core::DefaultType>::DefaultType), angle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearGradientBrushFactory, BASE_OFFSET>(),
            CreateInstanceWithGradientStopCollectionAndAngle: CreateInstanceWithGradientStopCollectionAndAngle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearGradientBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushStatics_Impl: Sized {
    fn StartPointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn EndPointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearGradientBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILinearGradientBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearGradientBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearGradientBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearGradientBrushStatics_Vtbl {
        unsafe extern "system" fn StartPointProperty<Impl: ILinearGradientBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPointProperty<Impl: ILinearGradientBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearGradientBrushStatics, BASE_OFFSET>(),
            StartPointProperty: StartPointProperty::<Impl, IMPL_OFFSET>,
            EndPointProperty: EndPointProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearGradientBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSourceLoadCompletedEventArgs_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<LoadedImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoadedImageSourceLoadCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILoadedImageSourceLoadCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILoadedImageSourceLoadCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadedImageSourceLoadCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoadedImageSourceLoadCompletedEventArgs_Vtbl {
        unsafe extern "system" fn Status<Impl: ILoadedImageSourceLoadCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoadedImageSourceLoadStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoadedImageSourceLoadCompletedEventArgs, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadedImageSourceLoadCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILoadedImageSurface_Impl: Sized {
    fn DecodedPhysicalSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn DecodedSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn NaturalSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn LoadCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILoadedImageSurface {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILoadedImageSurface";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILoadedImageSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadedImageSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoadedImageSurface_Vtbl {
        unsafe extern "system" fn DecodedPhysicalSize<Impl: ILoadedImageSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodedPhysicalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodedSize<Impl: ILoadedImageSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalSize<Impl: ILoadedImageSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCompleted<Impl: ILoadedImageSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoadCompleted<Impl: ILoadedImageSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoadCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoadedImageSurface, BASE_OFFSET>(),
            DecodedPhysicalSize: DecodedPhysicalSize::<Impl, IMPL_OFFSET>,
            DecodedSize: DecodedSize::<Impl, IMPL_OFFSET>,
            NaturalSize: NaturalSize::<Impl, IMPL_OFFSET>,
            LoadCompleted: LoadCompleted::<Impl, IMPL_OFFSET>,
            RemoveLoadCompleted: RemoveLoadCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadedImageSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILoadedImageSurfaceStatics_Impl: Sized {
    fn StartLoadFromUriWithSize(&mut self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, desiredmaxsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromUri(&mut self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromStreamWithSize(&mut self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>, desiredmaxsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromStream(&mut self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<LoadedImageSurface>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILoadedImageSurfaceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILoadedImageSurfaceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILoadedImageSurfaceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadedImageSurfaceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoadedImageSurfaceStatics_Vtbl {
        unsafe extern "system" fn StartLoadFromUriWithSize<Impl: ILoadedImageSurfaceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLoadFromUriWithSize(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&desiredmaxsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartLoadFromUri<Impl: ILoadedImageSurfaceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLoadFromUri(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartLoadFromStreamWithSize<Impl: ILoadedImageSurfaceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLoadFromStreamWithSize(&*(&stream as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&desiredmaxsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartLoadFromStream<Impl: ILoadedImageSurfaceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLoadFromStream(&*(&stream as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoadedImageSurfaceStatics, BASE_OFFSET>(),
            StartLoadFromUriWithSize: StartLoadFromUriWithSize::<Impl, IMPL_OFFSET>,
            StartLoadFromUri: StartLoadFromUri::<Impl, IMPL_OFFSET>,
            StartLoadFromStreamWithSize: StartLoadFromStreamWithSize::<Impl, IMPL_OFFSET>,
            StartLoadFromStream: StartLoadFromStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadedImageSurfaceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
pub trait IMatrix3DProjection_Impl: Sized {
    fn ProjectionMatrix(&mut self) -> ::windows::core::Result<Media3D::Matrix3D>;
    fn SetProjectionMatrix(&mut self, value: &Media3D::Matrix3D) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMatrix3DProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrix3DProjection";
}
#[cfg(all(feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
impl IMatrix3DProjection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrix3DProjection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrix3DProjection_Vtbl {
        unsafe extern "system" fn ProjectionMatrix<Impl: IMatrix3DProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjectionMatrix<Impl: IMatrix3DProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProjectionMatrix(&*(&value as *const <Media3D::Matrix3D as ::windows::core::Abi>::Abi as *const <Media3D::Matrix3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMatrix3DProjection, BASE_OFFSET>(),
            ProjectionMatrix: ProjectionMatrix::<Impl, IMPL_OFFSET>,
            SetProjectionMatrix: SetProjectionMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrix3DProjection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DProjectionStatics_Impl: Sized {
    fn ProjectionMatrixProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrix3DProjectionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrix3DProjectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrix3DProjectionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrix3DProjectionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrix3DProjectionStatics_Vtbl {
        unsafe extern "system" fn ProjectionMatrixProperty<Impl: IMatrix3DProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrixProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMatrix3DProjectionStatics, BASE_OFFSET>(),
            ProjectionMatrixProperty: ProjectionMatrixProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrix3DProjectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrixHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrixHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMatrixHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrixHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMatrixHelperStatics_Impl: Sized {
    fn Identity(&mut self) -> ::windows::core::Result<Matrix>;
    fn FromElements(&mut self, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64) -> ::windows::core::Result<Matrix>;
    fn GetIsIdentity(&mut self, target: &Matrix) -> ::windows::core::Result<bool>;
    fn Transform(&mut self, target: &Matrix, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMatrixHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMatrixHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrixHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrixHelperStatics_Vtbl {
        unsafe extern "system" fn Identity<Impl: IMatrixHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromElements<Impl: IMatrixHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromElements(m11, m12, m21, m22, offsetx, offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsIdentity<Impl: IMatrixHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Matrix, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsIdentity(&*(&target as *const <Matrix as ::windows::core::Abi>::Abi as *const <Matrix as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Impl: IMatrixHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Matrix, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform(&*(&target as *const <Matrix as ::windows::core::Abi>::Abi as *const <Matrix as ::windows::core::DefaultType>::DefaultType), &*(&point as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMatrixHelperStatics, BASE_OFFSET>(),
            Identity: Identity::<Impl, IMPL_OFFSET>,
            FromElements: FromElements::<Impl, IMPL_OFFSET>,
            GetIsIdentity: GetIsIdentity::<Impl, IMPL_OFFSET>,
            Transform: Transform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrixHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixTransform_Impl: Sized {
    fn Matrix(&mut self) -> ::windows::core::Result<Matrix>;
    fn SetMatrix(&mut self, value: &Matrix) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrixTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrixTransform_Vtbl {
        unsafe extern "system" fn Matrix<Impl: IMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Matrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Impl: IMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(&*(&value as *const <Matrix as ::windows::core::Abi>::Abi as *const <Matrix as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMatrixTransform, BASE_OFFSET>(),
            Matrix: Matrix::<Impl, IMPL_OFFSET>,
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrixTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixTransformStatics_Impl: Sized {
    fn MatrixProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixTransformStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrixTransformStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrixTransformStatics_Vtbl {
        unsafe extern "system" fn MatrixProperty<Impl: IMatrixTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatrixProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMatrixTransformStatics, BASE_OFFSET>(),
            MatrixProperty: MatrixProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrixTransformStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaTransportControlsThumbnailRequestedEventArgs_Impl: Sized {
    fn SetThumbnailImage(&mut self, source: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTransportControlsThumbnailRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMediaTransportControlsThumbnailRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsThumbnailRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl {
        unsafe extern "system" fn SetThumbnailImage<Impl: IMediaTransportControlsThumbnailRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailImage(&*(&source as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaTransportControlsThumbnailRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTransportControlsThumbnailRequestedEventArgs, BASE_OFFSET>(),
            SetThumbnailImage: SetThumbnailImage::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTransportControlsThumbnailRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IPartialMediaFailureDetectedEventArgs_Impl: Sized {
    fn StreamKind(&mut self) -> ::windows::core::Result<super::super::super::Media::Playback::FailedMediaStreamKind>;
}
#[cfg(all(feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPartialMediaFailureDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPartialMediaFailureDetectedEventArgs";
}
#[cfg(all(feature = "Media_Playback", feature = "implement_exclusive"))]
impl IPartialMediaFailureDetectedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPartialMediaFailureDetectedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPartialMediaFailureDetectedEventArgs_Vtbl {
        unsafe extern "system" fn StreamKind<Impl: IPartialMediaFailureDetectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Media::Playback::FailedMediaStreamKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPartialMediaFailureDetectedEventArgs, BASE_OFFSET>(),
            StreamKind: StreamKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPartialMediaFailureDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPartialMediaFailureDetectedEventArgs2_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPartialMediaFailureDetectedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPartialMediaFailureDetectedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IPartialMediaFailureDetectedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPartialMediaFailureDetectedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPartialMediaFailureDetectedEventArgs2_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IPartialMediaFailureDetectedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPartialMediaFailureDetectedEventArgs2, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPartialMediaFailureDetectedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPathFigure_Impl: Sized {
    fn Segments(&mut self) -> ::windows::core::Result<PathSegmentCollection>;
    fn SetSegments(&mut self, value: &::core::option::Option<PathSegmentCollection>) -> ::windows::core::Result<()>;
    fn StartPoint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsClosed(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsClosed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsFilled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsFilled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPathFigure {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathFigure";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPathFigure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathFigure_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathFigure_Vtbl {
        unsafe extern "system" fn Segments<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Segments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegments<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegments(&*(&value as *const <PathSegmentCollection as ::windows::core::Abi>::Abi as *const <PathSegmentCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPoint<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsClosed<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsClosed(value).into()
        }
        unsafe extern "system" fn IsFilled<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFilled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Impl: IPathFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFilled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPathFigure, BASE_OFFSET>(),
            Segments: Segments::<Impl, IMPL_OFFSET>,
            SetSegments: SetSegments::<Impl, IMPL_OFFSET>,
            StartPoint: StartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint: SetStartPoint::<Impl, IMPL_OFFSET>,
            IsClosed: IsClosed::<Impl, IMPL_OFFSET>,
            SetIsClosed: SetIsClosed::<Impl, IMPL_OFFSET>,
            IsFilled: IsFilled::<Impl, IMPL_OFFSET>,
            SetIsFilled: SetIsFilled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathFigure as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFigureStatics_Impl: Sized {
    fn SegmentsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StartPointProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsClosedProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFilledProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathFigureStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathFigureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPathFigureStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathFigureStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathFigureStatics_Vtbl {
        unsafe extern "system" fn SegmentsProperty<Impl: IPathFigureStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SegmentsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPointProperty<Impl: IPathFigureStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClosedProperty<Impl: IPathFigureStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClosedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFilledProperty<Impl: IPathFigureStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFilledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPathFigureStatics, BASE_OFFSET>(),
            SegmentsProperty: SegmentsProperty::<Impl, IMPL_OFFSET>,
            StartPointProperty: StartPointProperty::<Impl, IMPL_OFFSET>,
            IsClosedProperty: IsClosedProperty::<Impl, IMPL_OFFSET>,
            IsFilledProperty: IsFilledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathFigureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPathGeometry_Impl: Sized {
    fn FillRule(&mut self) -> ::windows::core::Result<FillRule>;
    fn SetFillRule(&mut self, value: FillRule) -> ::windows::core::Result<()>;
    fn Figures(&mut self) -> ::windows::core::Result<PathFigureCollection>;
    fn SetFigures(&mut self, value: &::core::option::Option<PathFigureCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPathGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathGeometry";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPathGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathGeometry_Vtbl {
        unsafe extern "system" fn FillRule<Impl: IPathGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FillRule) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFillRule<Impl: IPathGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillRule(value).into()
        }
        unsafe extern "system" fn Figures<Impl: IPathGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Figures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFigures<Impl: IPathGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFigures(&*(&value as *const <PathFigureCollection as ::windows::core::Abi>::Abi as *const <PathFigureCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPathGeometry, BASE_OFFSET>(),
            FillRule: FillRule::<Impl, IMPL_OFFSET>,
            SetFillRule: SetFillRule::<Impl, IMPL_OFFSET>,
            Figures: Figures::<Impl, IMPL_OFFSET>,
            SetFigures: SetFigures::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathGeometryStatics_Impl: Sized {
    fn FillRuleProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FiguresProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPathGeometryStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathGeometryStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathGeometryStatics_Vtbl {
        unsafe extern "system" fn FillRuleProperty<Impl: IPathGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FiguresProperty<Impl: IPathGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FiguresProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPathGeometryStatics, BASE_OFFSET>(),
            FillRuleProperty: FillRuleProperty::<Impl, IMPL_OFFSET>,
            FiguresProperty: FiguresProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathGeometryStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathSegment_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IPathSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathSegment_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPathSegment, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathSegmentFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathSegmentFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathSegmentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPathSegmentFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathSegmentFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathSegmentFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPathSegmentFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathSegmentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
pub trait IPlaneProjection_Impl: Sized {
    fn LocalOffsetX(&mut self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn LocalOffsetY(&mut self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn LocalOffsetZ(&mut self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetZ(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RotationX(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotationX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RotationY(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotationY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RotationZ(&mut self) -> ::windows::core::Result<f64>;
    fn SetRotationZ(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationX(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationY(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationZ(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationZ(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetX(&mut self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetY(&mut self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetZ(&mut self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetZ(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ProjectionMatrix(&mut self) -> ::windows::core::Result<Media3D::Matrix3D>;
}
#[cfg(all(feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaneProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPlaneProjection";
}
#[cfg(all(feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
impl IPlaneProjection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaneProjection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaneProjection_Vtbl {
        unsafe extern "system" fn LocalOffsetX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalOffsetX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalOffsetX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalOffsetX(value).into()
        }
        unsafe extern "system" fn LocalOffsetY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalOffsetY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalOffsetY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalOffsetY(value).into()
        }
        unsafe extern "system" fn LocalOffsetZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalOffsetZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalOffsetZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalOffsetZ(value).into()
        }
        unsafe extern "system" fn RotationX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationX(value).into()
        }
        unsafe extern "system" fn RotationY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationY(value).into()
        }
        unsafe extern "system" fn RotationZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationZ(value).into()
        }
        unsafe extern "system" fn CenterOfRotationX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterOfRotationX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterOfRotationX(value).into()
        }
        unsafe extern "system" fn CenterOfRotationY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterOfRotationY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterOfRotationY(value).into()
        }
        unsafe extern "system" fn CenterOfRotationZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterOfRotationZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterOfRotationZ(value).into()
        }
        unsafe extern "system" fn GlobalOffsetX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalOffsetX<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalOffsetX(value).into()
        }
        unsafe extern "system" fn GlobalOffsetY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalOffsetY<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalOffsetY(value).into()
        }
        unsafe extern "system" fn GlobalOffsetZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalOffsetZ<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalOffsetZ(value).into()
        }
        unsafe extern "system" fn ProjectionMatrix<Impl: IPlaneProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaneProjection, BASE_OFFSET>(),
            LocalOffsetX: LocalOffsetX::<Impl, IMPL_OFFSET>,
            SetLocalOffsetX: SetLocalOffsetX::<Impl, IMPL_OFFSET>,
            LocalOffsetY: LocalOffsetY::<Impl, IMPL_OFFSET>,
            SetLocalOffsetY: SetLocalOffsetY::<Impl, IMPL_OFFSET>,
            LocalOffsetZ: LocalOffsetZ::<Impl, IMPL_OFFSET>,
            SetLocalOffsetZ: SetLocalOffsetZ::<Impl, IMPL_OFFSET>,
            RotationX: RotationX::<Impl, IMPL_OFFSET>,
            SetRotationX: SetRotationX::<Impl, IMPL_OFFSET>,
            RotationY: RotationY::<Impl, IMPL_OFFSET>,
            SetRotationY: SetRotationY::<Impl, IMPL_OFFSET>,
            RotationZ: RotationZ::<Impl, IMPL_OFFSET>,
            SetRotationZ: SetRotationZ::<Impl, IMPL_OFFSET>,
            CenterOfRotationX: CenterOfRotationX::<Impl, IMPL_OFFSET>,
            SetCenterOfRotationX: SetCenterOfRotationX::<Impl, IMPL_OFFSET>,
            CenterOfRotationY: CenterOfRotationY::<Impl, IMPL_OFFSET>,
            SetCenterOfRotationY: SetCenterOfRotationY::<Impl, IMPL_OFFSET>,
            CenterOfRotationZ: CenterOfRotationZ::<Impl, IMPL_OFFSET>,
            SetCenterOfRotationZ: SetCenterOfRotationZ::<Impl, IMPL_OFFSET>,
            GlobalOffsetX: GlobalOffsetX::<Impl, IMPL_OFFSET>,
            SetGlobalOffsetX: SetGlobalOffsetX::<Impl, IMPL_OFFSET>,
            GlobalOffsetY: GlobalOffsetY::<Impl, IMPL_OFFSET>,
            SetGlobalOffsetY: SetGlobalOffsetY::<Impl, IMPL_OFFSET>,
            GlobalOffsetZ: GlobalOffsetZ::<Impl, IMPL_OFFSET>,
            SetGlobalOffsetZ: SetGlobalOffsetZ::<Impl, IMPL_OFFSET>,
            ProjectionMatrix: ProjectionMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaneProjection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaneProjectionStatics_Impl: Sized {
    fn LocalOffsetXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LocalOffsetYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LocalOffsetZProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationZProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationZProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetZProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProjectionMatrixProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaneProjectionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPlaneProjectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaneProjectionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaneProjectionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaneProjectionStatics_Vtbl {
        unsafe extern "system" fn LocalOffsetXProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalOffsetXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalOffsetYProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalOffsetYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalOffsetZProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalOffsetZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationXProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationYProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationZProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterOfRotationXProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterOfRotationYProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterOfRotationZProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalOffsetXProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalOffsetYProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalOffsetZProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionMatrixProperty<Impl: IPlaneProjectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrixProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaneProjectionStatics, BASE_OFFSET>(),
            LocalOffsetXProperty: LocalOffsetXProperty::<Impl, IMPL_OFFSET>,
            LocalOffsetYProperty: LocalOffsetYProperty::<Impl, IMPL_OFFSET>,
            LocalOffsetZProperty: LocalOffsetZProperty::<Impl, IMPL_OFFSET>,
            RotationXProperty: RotationXProperty::<Impl, IMPL_OFFSET>,
            RotationYProperty: RotationYProperty::<Impl, IMPL_OFFSET>,
            RotationZProperty: RotationZProperty::<Impl, IMPL_OFFSET>,
            CenterOfRotationXProperty: CenterOfRotationXProperty::<Impl, IMPL_OFFSET>,
            CenterOfRotationYProperty: CenterOfRotationYProperty::<Impl, IMPL_OFFSET>,
            CenterOfRotationZProperty: CenterOfRotationZProperty::<Impl, IMPL_OFFSET>,
            GlobalOffsetXProperty: GlobalOffsetXProperty::<Impl, IMPL_OFFSET>,
            GlobalOffsetYProperty: GlobalOffsetYProperty::<Impl, IMPL_OFFSET>,
            GlobalOffsetZProperty: GlobalOffsetZProperty::<Impl, IMPL_OFFSET>,
            ProjectionMatrixProperty: ProjectionMatrixProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaneProjectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPolyBezierSegment_Impl: Sized {
    fn Points(&mut self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&mut self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPolyBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyBezierSegment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPolyBezierSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolyBezierSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolyBezierSegment_Vtbl {
        unsafe extern "system" fn Points<Impl: IPolyBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPoints<Impl: IPolyBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <PointCollection as ::windows::core::Abi>::Abi as *const <PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPolyBezierSegment, BASE_OFFSET>(),
            Points: Points::<Impl, IMPL_OFFSET>,
            SetPoints: SetPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyBezierSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyBezierSegmentStatics_Impl: Sized {
    fn PointsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyBezierSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolyBezierSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolyBezierSegmentStatics_Vtbl {
        unsafe extern "system" fn PointsProperty<Impl: IPolyBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPolyBezierSegmentStatics, BASE_OFFSET>(),
            PointsProperty: PointsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyBezierSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPolyLineSegment_Impl: Sized {
    fn Points(&mut self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&mut self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPolyLineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyLineSegment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPolyLineSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolyLineSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolyLineSegment_Vtbl {
        unsafe extern "system" fn Points<Impl: IPolyLineSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPoints<Impl: IPolyLineSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <PointCollection as ::windows::core::Abi>::Abi as *const <PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPolyLineSegment, BASE_OFFSET>(),
            Points: Points::<Impl, IMPL_OFFSET>,
            SetPoints: SetPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyLineSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyLineSegmentStatics_Impl: Sized {
    fn PointsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyLineSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyLineSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyLineSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolyLineSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolyLineSegmentStatics_Vtbl {
        unsafe extern "system" fn PointsProperty<Impl: IPolyLineSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPolyLineSegmentStatics, BASE_OFFSET>(),
            PointsProperty: PointsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyLineSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPolyQuadraticBezierSegment_Impl: Sized {
    fn Points(&mut self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&mut self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPolyQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyQuadraticBezierSegment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPolyQuadraticBezierSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolyQuadraticBezierSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolyQuadraticBezierSegment_Vtbl {
        unsafe extern "system" fn Points<Impl: IPolyQuadraticBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPoints<Impl: IPolyQuadraticBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <PointCollection as ::windows::core::Abi>::Abi as *const <PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPolyQuadraticBezierSegment, BASE_OFFSET>(),
            Points: Points::<Impl, IMPL_OFFSET>,
            SetPoints: SetPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyQuadraticBezierSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyQuadraticBezierSegmentStatics_Impl: Sized {
    fn PointsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyQuadraticBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyQuadraticBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyQuadraticBezierSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolyQuadraticBezierSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolyQuadraticBezierSegmentStatics_Vtbl {
        unsafe extern "system" fn PointsProperty<Impl: IPolyQuadraticBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPolyQuadraticBezierSegmentStatics, BASE_OFFSET>(),
            PointsProperty: PointsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolyQuadraticBezierSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjection_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IProjection";
}
#[cfg(feature = "implement_exclusive")]
impl IProjection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProjection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProjection_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProjection, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProjection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Projection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProjectionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IProjectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProjectionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProjectionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProjectionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IProjectionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProjectionFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProjectionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IQuadraticBezierSegment_Impl: Sized {
    fn Point1(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint1(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point2(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint2(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IQuadraticBezierSegment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IQuadraticBezierSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuadraticBezierSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuadraticBezierSegment_Vtbl {
        unsafe extern "system" fn Point1<Impl: IQuadraticBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint1<Impl: IQuadraticBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint1(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point2<Impl: IQuadraticBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint2<Impl: IQuadraticBezierSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint2(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IQuadraticBezierSegment, BASE_OFFSET>(),
            Point1: Point1::<Impl, IMPL_OFFSET>,
            SetPoint1: SetPoint1::<Impl, IMPL_OFFSET>,
            Point2: Point2::<Impl, IMPL_OFFSET>,
            SetPoint2: SetPoint2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuadraticBezierSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticBezierSegmentStatics_Impl: Sized {
    fn Point1Property(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point2Property(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuadraticBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IQuadraticBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IQuadraticBezierSegmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuadraticBezierSegmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuadraticBezierSegmentStatics_Vtbl {
        unsafe extern "system" fn Point1Property<Impl: IQuadraticBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point2Property<Impl: IQuadraticBezierSegmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IQuadraticBezierSegmentStatics, BASE_OFFSET>(),
            Point1Property: Point1Property::<Impl, IMPL_OFFSET>,
            Point2Property: Point2Property::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuadraticBezierSegmentStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRateChangedRoutedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRateChangedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRateChangedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRateChangedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRateChangedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRateChangedRoutedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRateChangedRoutedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRateChangedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRectangleGeometry_Impl: Sized {
    fn Rect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetRect(&mut self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRectangleGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRectangleGeometry";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRectangleGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectangleGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectangleGeometry_Vtbl {
        unsafe extern "system" fn Rect<Impl: IRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Impl: IRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRectangleGeometry, BASE_OFFSET>(),
            Rect: Rect::<Impl, IMPL_OFFSET>,
            SetRect: SetRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectangleGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleGeometryStatics_Impl: Sized {
    fn RectProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectangleGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRectangleGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRectangleGeometryStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectangleGeometryStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectangleGeometryStatics_Vtbl {
        unsafe extern "system" fn RectProperty<Impl: IRectangleGeometryStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRectangleGeometryStatics, BASE_OFFSET>(),
            RectProperty: RectProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectangleGeometryStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRenderedEventArgs_Impl: Sized {
    fn FrameDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRenderedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRenderedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRenderedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderedEventArgs_Vtbl {
        unsafe extern "system" fn FrameDuration<Impl: IRenderedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRenderedEventArgs, BASE_OFFSET>(), FrameDuration: FrameDuration::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRenderedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRenderingEventArgs_Impl: Sized {
    fn RenderingTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRenderingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRenderingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRenderingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderingEventArgs_Vtbl {
        unsafe extern "system" fn RenderingTime<Impl: IRenderingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRenderingEventArgs, BASE_OFFSET>(), RenderingTime: RenderingTime::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRenderingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBackgroundBrush_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBackgroundBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBackgroundBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBackgroundBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBackgroundBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBackgroundBrush_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBackgroundBrush, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBackgroundBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBackgroundBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBackgroundBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBackgroundBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBackgroundBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBackgroundBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBackgroundBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBackgroundBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRevealBackgroundBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBackgroundBrushFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBackgroundBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBorderBrush_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBorderBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBorderBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBorderBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBorderBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBorderBrush_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBorderBrush, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBorderBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBorderBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBorderBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBorderBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBorderBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBorderBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBorderBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBorderBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRevealBorderBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBorderBrushFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBorderBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrush_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn TargetTheme(&mut self) -> ::windows::core::Result<super::ApplicationTheme>;
    fn SetTargetTheme(&mut self, value: super::ApplicationTheme) -> ::windows::core::Result<()>;
    fn AlwaysUseFallback(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlwaysUseFallback(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBrush_Vtbl {
        unsafe extern "system" fn Color<Impl: IRevealBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IRevealBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetTheme<Impl: IRevealBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetTheme<Impl: IRevealBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetTheme(value).into()
        }
        unsafe extern "system" fn AlwaysUseFallback<Impl: IRevealBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysUseFallback<Impl: IRevealBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysUseFallback(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBrush, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            TargetTheme: TargetTheme::<Impl, IMPL_OFFSET>,
            SetTargetTheme: SetTargetTheme::<Impl, IMPL_OFFSET>,
            AlwaysUseFallback: AlwaysUseFallback::<Impl, IMPL_OFFSET>,
            SetAlwaysUseFallback: SetAlwaysUseFallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRevealBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBrushFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushStatics_Impl: Sized {
    fn ColorProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TargetThemeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysUseFallbackProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StateProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SetState(&mut self, element: &::core::option::Option<super::UIElement>, value: RevealBrushState) -> ::windows::core::Result<()>;
    fn GetState(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<RevealBrushState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevealBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevealBrushStatics_Vtbl {
        unsafe extern "system" fn ColorProperty<Impl: IRevealBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetThemeProperty<Impl: IRevealBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetThemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlwaysUseFallbackProperty<Impl: IRevealBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallbackProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateProperty<Impl: IRevealBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IRevealBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: RevealBrushState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn GetState<Impl: IRevealBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut RevealBrushState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRevealBrushStatics, BASE_OFFSET>(),
            ColorProperty: ColorProperty::<Impl, IMPL_OFFSET>,
            TargetThemeProperty: TargetThemeProperty::<Impl, IMPL_OFFSET>,
            AlwaysUseFallbackProperty: AlwaysUseFallbackProperty::<Impl, IMPL_OFFSET>,
            StateProperty: StateProperty::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            GetState: GetState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevealBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRotateTransform_Impl: Sized {
    fn CenterX(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Angle(&mut self) -> ::windows::core::Result<f64>;
    fn SetAngle(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRotateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRotateTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IRotateTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRotateTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRotateTransform_Vtbl {
        unsafe extern "system" fn CenterX<Impl: IRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: IRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn Angle<Impl: IRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Angle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngle<Impl: IRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRotateTransform, BASE_OFFSET>(),
            CenterX: CenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            CenterY: CenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            Angle: Angle::<Impl, IMPL_OFFSET>,
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRotateTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRotateTransformStatics_Impl: Sized {
    fn CenterXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRotateTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRotateTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRotateTransformStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRotateTransformStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRotateTransformStatics_Vtbl {
        unsafe extern "system" fn CenterXProperty<Impl: IRotateTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: IRotateTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleProperty<Impl: IRotateTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRotateTransformStatics, BASE_OFFSET>(),
            CenterXProperty: CenterXProperty::<Impl, IMPL_OFFSET>,
            CenterYProperty: CenterYProperty::<Impl, IMPL_OFFSET>,
            AngleProperty: AngleProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRotateTransformStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleTransform_Impl: Sized {
    fn CenterX(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&mut self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&mut self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScaleTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IScaleTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IScaleTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScaleTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScaleTransform_Vtbl {
        unsafe extern "system" fn CenterX<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn ScaleX<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(value).into()
        }
        unsafe extern "system" fn ScaleY<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: IScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScaleTransform, BASE_OFFSET>(),
            CenterX: CenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            CenterY: CenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            ScaleX: ScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            ScaleY: ScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScaleTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleTransformStatics_Impl: Sized {
    fn CenterXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScaleTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IScaleTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScaleTransformStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScaleTransformStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScaleTransformStatics_Vtbl {
        unsafe extern "system" fn CenterXProperty<Impl: IScaleTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: IScaleTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleXProperty<Impl: IScaleTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleYProperty<Impl: IScaleTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScaleTransformStatics, BASE_OFFSET>(),
            CenterXProperty: CenterXProperty::<Impl, IMPL_OFFSET>,
            CenterYProperty: CenterYProperty::<Impl, IMPL_OFFSET>,
            ScaleXProperty: ScaleXProperty::<Impl, IMPL_OFFSET>,
            ScaleYProperty: ScaleYProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScaleTransformStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShadow_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IShadow";
}
#[cfg(feature = "implement_exclusive")]
impl IShadow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShadow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShadow_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IShadow, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShadowFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShadowFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IShadowFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IShadowFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShadowFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShadowFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IShadowFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShadowFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISkewTransform_Impl: Sized {
    fn CenterX(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&mut self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn AngleX(&mut self) -> ::windows::core::Result<f64>;
    fn SetAngleX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn AngleY(&mut self) -> ::windows::core::Result<f64>;
    fn SetAngleY(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISkewTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISkewTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ISkewTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISkewTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISkewTransform_Vtbl {
        unsafe extern "system" fn CenterX<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn AngleX<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleX<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleX(value).into()
        }
        unsafe extern "system" fn AngleY<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleY<Impl: ISkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleY(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISkewTransform, BASE_OFFSET>(),
            CenterX: CenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            CenterY: CenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            AngleX: AngleX::<Impl, IMPL_OFFSET>,
            SetAngleX: SetAngleX::<Impl, IMPL_OFFSET>,
            AngleY: AngleY::<Impl, IMPL_OFFSET>,
            SetAngleY: SetAngleY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISkewTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISkewTransformStatics_Impl: Sized {
    fn CenterXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISkewTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISkewTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISkewTransformStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISkewTransformStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISkewTransformStatics_Vtbl {
        unsafe extern "system" fn CenterXProperty<Impl: ISkewTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: ISkewTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleXProperty<Impl: ISkewTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleYProperty<Impl: ISkewTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISkewTransformStatics, BASE_OFFSET>(),
            CenterXProperty: CenterXProperty::<Impl, IMPL_OFFSET>,
            CenterYProperty: CenterYProperty::<Impl, IMPL_OFFSET>,
            AngleXProperty: AngleXProperty::<Impl, IMPL_OFFSET>,
            AngleYProperty: AngleYProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISkewTransformStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrush_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISolidColorBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISolidColorBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ISolidColorBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISolidColorBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISolidColorBrush_Vtbl {
        unsafe extern "system" fn Color<Impl: ISolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: ISolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISolidColorBrush, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISolidColorBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushFactory_Impl: Sized {
    fn CreateInstanceWithColor(&mut self, color: &super::super::Color) -> ::windows::core::Result<SolidColorBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISolidColorBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISolidColorBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISolidColorBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISolidColorBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISolidColorBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithColor<Impl: ISolidColorBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: super::super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithColor(&*(&color as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISolidColorBrushFactory, BASE_OFFSET>(),
            CreateInstanceWithColor: CreateInstanceWithColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISolidColorBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushStatics_Impl: Sized {
    fn ColorProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISolidColorBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISolidColorBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISolidColorBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISolidColorBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISolidColorBrushStatics_Vtbl {
        unsafe extern "system" fn ColorProperty<Impl: ISolidColorBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISolidColorBrushStatics, BASE_OFFSET>(),
            ColorProperty: ColorProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISolidColorBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThemeShadow_Impl: Sized {
    fn Receivers(&mut self) -> ::windows::core::Result<super::UIElementWeakCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThemeShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IThemeShadow";
}
#[cfg(feature = "implement_exclusive")]
impl IThemeShadow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThemeShadow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThemeShadow_Vtbl {
        unsafe extern "system" fn Receivers<Impl: IThemeShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receivers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IThemeShadow, BASE_OFFSET>(), Receivers: Receivers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThemeShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThemeShadowFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThemeShadow>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThemeShadowFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IThemeShadowFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IThemeShadowFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThemeShadowFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThemeShadowFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IThemeShadowFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThemeShadowFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThemeShadowFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrush_Impl: Sized {
    fn AlignmentX(&mut self) -> ::windows::core::Result<AlignmentX>;
    fn SetAlignmentX(&mut self, value: AlignmentX) -> ::windows::core::Result<()>;
    fn AlignmentY(&mut self) -> ::windows::core::Result<AlignmentY>;
    fn SetAlignmentY(&mut self, value: AlignmentY) -> ::windows::core::Result<()>;
    fn Stretch(&mut self) -> ::windows::core::Result<Stretch>;
    fn SetStretch(&mut self, value: Stretch) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITileBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ITileBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileBrush_Vtbl {
        unsafe extern "system" fn AlignmentX<Impl: ITileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AlignmentX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlignmentX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignmentX<Impl: ITileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AlignmentX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignmentX(value).into()
        }
        unsafe extern "system" fn AlignmentY<Impl: ITileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AlignmentY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlignmentY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignmentY<Impl: ITileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AlignmentY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignmentY(value).into()
        }
        unsafe extern "system" fn Stretch<Impl: ITileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Stretch) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStretch<Impl: ITileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Stretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileBrush, BASE_OFFSET>(),
            AlignmentX: AlignmentX::<Impl, IMPL_OFFSET>,
            SetAlignmentX: SetAlignmentX::<Impl, IMPL_OFFSET>,
            AlignmentY: AlignmentY::<Impl, IMPL_OFFSET>,
            SetAlignmentY: SetAlignmentY::<Impl, IMPL_OFFSET>,
            Stretch: Stretch::<Impl, IMPL_OFFSET>,
            SetStretch: SetStretch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TileBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITileBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITileBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileBrushFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITileBrushFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITileBrushFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushStatics_Impl: Sized {
    fn AlignmentXProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlignmentYProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITileBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITileBrushStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileBrushStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileBrushStatics_Vtbl {
        unsafe extern "system" fn AlignmentXProperty<Impl: ITileBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlignmentXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlignmentYProperty<Impl: ITileBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlignmentYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StretchProperty<Impl: ITileBrushStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileBrushStatics, BASE_OFFSET>(),
            AlignmentXProperty: AlignmentXProperty::<Impl, IMPL_OFFSET>,
            AlignmentYProperty: AlignmentYProperty::<Impl, IMPL_OFFSET>,
            StretchProperty: StretchProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileBrushStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITimelineMarker_Impl: Sized {
    fn Time(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTime(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimelineMarker {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITimelineMarker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITimelineMarker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineMarker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineMarker_Vtbl {
        unsafe extern "system" fn Time<Impl: ITimelineMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTime<Impl: ITimelineMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTime(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Type<Impl: ITimelineMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: ITimelineMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Text<Impl: ITimelineMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetText<Impl: ITimelineMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimelineMarker, BASE_OFFSET>(),
            Time: Time::<Impl, IMPL_OFFSET>,
            SetTime: SetTime::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineMarker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerRoutedEventArgs_Impl: Sized {
    fn Marker(&mut self) -> ::windows::core::Result<TimelineMarker>;
    fn SetMarker(&mut self, value: &::core::option::Option<TimelineMarker>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineMarkerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITimelineMarkerRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineMarkerRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineMarkerRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineMarkerRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Marker<Impl: ITimelineMarkerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Marker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarker<Impl: ITimelineMarkerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(&*(&value as *const <TimelineMarker as ::windows::core::Abi>::Abi as *const <TimelineMarker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimelineMarkerRoutedEventArgs, BASE_OFFSET>(),
            Marker: Marker::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineMarkerRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerStatics_Impl: Sized {
    fn TimeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TypeProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineMarkerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITimelineMarkerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineMarkerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineMarkerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineMarkerStatics_Vtbl {
        unsafe extern "system" fn TimeProperty<Impl: ITimelineMarkerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeProperty<Impl: ITimelineMarkerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextProperty<Impl: ITimelineMarkerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimelineMarkerStatics, BASE_OFFSET>(),
            TimeProperty: TimeProperty::<Impl, IMPL_OFFSET>,
            TypeProperty: TypeProperty::<Impl, IMPL_OFFSET>,
            TextProperty: TextProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineMarkerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransform_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransform";
}
#[cfg(feature = "implement_exclusive")]
impl ITransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransform_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransform, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransformFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITransformGroup_Impl: Sized {
    fn Children(&mut self) -> ::windows::core::Result<TransformCollection>;
    fn SetChildren(&mut self, value: &::core::option::Option<TransformCollection>) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<Matrix>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITransformGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransformGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITransformGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformGroup_Vtbl {
        unsafe extern "system" fn Children<Impl: ITransformGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildren<Impl: ITransformGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChildren(&*(&value as *const <TransformCollection as ::windows::core::Abi>::Abi as *const <TransformCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ITransformGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformGroup, BASE_OFFSET>(),
            Children: Children::<Impl, IMPL_OFFSET>,
            SetChildren: SetChildren::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformGroupStatics_Impl: Sized {
    fn ChildrenProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformGroupStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransformGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformGroupStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformGroupStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformGroupStatics_Vtbl {
        unsafe extern "system" fn ChildrenProperty<Impl: ITransformGroupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildrenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITransformGroupStatics, BASE_OFFSET>(),
            ChildrenProperty: ChildrenProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITranslateTransform_Impl: Sized {
    fn X(&mut self) -> ::windows::core::Result<f64>;
    fn SetX(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Y(&mut self) -> ::windows::core::Result<f64>;
    fn SetY(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITranslateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITranslateTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ITranslateTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITranslateTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITranslateTransform_Vtbl {
        unsafe extern "system" fn X<Impl: ITranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetX<Impl: ITranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetX(value).into()
        }
        unsafe extern "system" fn Y<Impl: ITranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetY<Impl: ITranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetY(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITranslateTransform, BASE_OFFSET>(),
            X: X::<Impl, IMPL_OFFSET>,
            SetX: SetX::<Impl, IMPL_OFFSET>,
            Y: Y::<Impl, IMPL_OFFSET>,
            SetY: SetY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITranslateTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITranslateTransformStatics_Impl: Sized {
    fn XProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITranslateTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITranslateTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITranslateTransformStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITranslateTransformStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITranslateTransformStatics_Vtbl {
        unsafe extern "system" fn XProperty<Impl: ITranslateTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YProperty<Impl: ITranslateTransformStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITranslateTransformStatics, BASE_OFFSET>(),
            XProperty: XProperty::<Impl, IMPL_OFFSET>,
            YProperty: YProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITranslateTransformStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTreeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTreeHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualTreeHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVisualTreeHelperStatics_Impl: Sized {
    fn FindElementsInHostCoordinatesPoint(&mut self, intersectingpoint: &super::super::super::Foundation::Point, subtree: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindElementsInHostCoordinatesRect(&mut self, intersectingrect: &super::super::super::Foundation::Rect, subtree: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindAllElementsInHostCoordinatesPoint(&mut self, intersectingpoint: &super::super::super::Foundation::Point, subtree: &::core::option::Option<super::UIElement>, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindAllElementsInHostCoordinatesRect(&mut self, intersectingrect: &super::super::super::Foundation::Rect, subtree: &::core::option::Option<super::UIElement>, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn GetChild(&mut self, reference: &::core::option::Option<super::DependencyObject>, childindex: i32) -> ::windows::core::Result<super::DependencyObject>;
    fn GetChildrenCount(&mut self, reference: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn GetParent(&mut self, reference: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn DisconnectChildrenRecursive(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualTreeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVisualTreeHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeHelperStatics_Vtbl {
        unsafe extern "system" fn FindElementsInHostCoordinatesPoint<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersectingpoint: super::super::super::Foundation::Point, subtree: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindElementsInHostCoordinatesPoint(&*(&intersectingpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindElementsInHostCoordinatesRect<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersectingrect: super::super::super::Foundation::Rect, subtree: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindElementsInHostCoordinatesRect(&*(&intersectingrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllElementsInHostCoordinatesPoint<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersectingpoint: super::super::super::Foundation::Point, subtree: ::windows::core::RawPtr, includeallelements: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllElementsInHostCoordinatesPoint(&*(&intersectingpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), includeallelements) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllElementsInHostCoordinatesRect<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersectingrect: super::super::super::Foundation::Rect, subtree: ::windows::core::RawPtr, includeallelements: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllElementsInHostCoordinatesRect(&*(&intersectingrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), includeallelements) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr, childindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChild(&*(&reference as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), childindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenCount<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenCount(&*(&reference as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent(&*(&reference as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectChildrenRecursive<Impl: IVisualTreeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectChildrenRecursive(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualTreeHelperStatics, BASE_OFFSET>(),
            FindElementsInHostCoordinatesPoint: FindElementsInHostCoordinatesPoint::<Impl, IMPL_OFFSET>,
            FindElementsInHostCoordinatesRect: FindElementsInHostCoordinatesRect::<Impl, IMPL_OFFSET>,
            FindAllElementsInHostCoordinatesPoint: FindAllElementsInHostCoordinatesPoint::<Impl, IMPL_OFFSET>,
            FindAllElementsInHostCoordinatesRect: FindAllElementsInHostCoordinatesRect::<Impl, IMPL_OFFSET>,
            GetChild: GetChild::<Impl, IMPL_OFFSET>,
            GetChildrenCount: GetChildrenCount::<Impl, IMPL_OFFSET>,
            GetParent: GetParent::<Impl, IMPL_OFFSET>,
            DisconnectChildrenRecursive: DisconnectChildrenRecursive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IVisualTreeHelperStatics2_Impl: Sized {
    fn GetOpenPopups(&mut self, window: &::core::option::Option<super::Window>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualTreeHelperStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelperStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IVisualTreeHelperStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeHelperStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeHelperStatics2_Vtbl {
        unsafe extern "system" fn GetOpenPopups<Impl: IVisualTreeHelperStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpenPopups(&*(&window as *const <super::Window as ::windows::core::Abi>::Abi as *const <super::Window as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualTreeHelperStatics2, BASE_OFFSET>(),
            GetOpenPopups: GetOpenPopups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeHelperStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IVisualTreeHelperStatics3_Impl: Sized {
    fn GetOpenPopupsForXamlRoot(&mut self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualTreeHelperStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelperStatics3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IVisualTreeHelperStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeHelperStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeHelperStatics3_Vtbl {
        unsafe extern "system" fn GetOpenPopupsForXamlRoot<Impl: IVisualTreeHelperStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamlroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpenPopupsForXamlRoot(&*(&xamlroot as *const <super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::XamlRoot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualTreeHelperStatics3, BASE_OFFSET>(),
            GetOpenPopupsForXamlRoot: GetOpenPopupsForXamlRoot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeHelperStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBase_Impl: Sized {
    fn FallbackColor(&mut self) -> ::windows::core::Result<super::super::Color>;
    fn SetFallbackColor(&mut self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBase";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlCompositionBrushBase_Vtbl {
        unsafe extern "system" fn FallbackColor<Impl: IXamlCompositionBrushBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FallbackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackColor<Impl: IXamlCompositionBrushBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlCompositionBrushBase, BASE_OFFSET>(),
            FallbackColor: FallbackColor::<Impl, IMPL_OFFSET>,
            SetFallbackColor: SetFallbackColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlCompositionBrushBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlCompositionBrushBaseFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlCompositionBrushBaseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlCompositionBrushBaseFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseOverrides_Impl: Sized {
    fn OnConnected(&mut self) -> ::windows::core::Result<()>;
    fn OnDisconnected(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlCompositionBrushBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConnected<Impl: IXamlCompositionBrushBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnected().into()
        }
        unsafe extern "system" fn OnDisconnected<Impl: IXamlCompositionBrushBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnected().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlCompositionBrushBaseOverrides, BASE_OFFSET>(),
            OnConnected: OnConnected::<Impl, IMPL_OFFSET>,
            OnDisconnected: OnDisconnected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IXamlCompositionBrushBaseProtected_Impl: Sized {
    fn CompositionBrush(&mut self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
    fn SetCompositionBrush(&mut self, value: &::core::option::Option<super::super::Composition::CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseProtected";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IXamlCompositionBrushBaseProtected_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseProtected_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlCompositionBrushBaseProtected_Vtbl {
        unsafe extern "system" fn CompositionBrush<Impl: IXamlCompositionBrushBaseProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositionBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionBrush<Impl: IXamlCompositionBrushBaseProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositionBrush(&*(&value as *const <super::super::Composition::CompositionBrush as ::windows::core::Abi>::Abi as *const <super::super::Composition::CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlCompositionBrushBaseProtected, BASE_OFFSET>(),
            CompositionBrush: CompositionBrush::<Impl, IMPL_OFFSET>,
            SetCompositionBrush: SetCompositionBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseProtected as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseStatics_Impl: Sized {
    fn FallbackColorProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlCompositionBrushBaseStatics_Vtbl {
        unsafe extern "system" fn FallbackColorProperty<Impl: IXamlCompositionBrushBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FallbackColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlCompositionBrushBaseStatics, BASE_OFFSET>(),
            FallbackColorProperty: FallbackColorProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLight_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLight {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLight";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLight_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLight_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlLight_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLight, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLight as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlLight>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlLightFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlLightFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLightFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLightFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightOverrides_Impl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnConnected(&mut self, newelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn OnDisconnected(&mut self, oldelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlLightOverrides_Vtbl {
        unsafe extern "system" fn GetId<Impl: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnected<Impl: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnected(&*(&newelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDisconnected<Impl: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnected(&*(&oldelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLightOverrides, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            OnConnected: OnConnected::<Impl, IMPL_OFFSET>,
            OnDisconnected: OnDisconnected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLightOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IXamlLightProtected_Impl: Sized {
    fn CompositionLight(&mut self) -> ::windows::core::Result<super::super::Composition::CompositionLight>;
    fn SetCompositionLight(&mut self, value: &::core::option::Option<super::super::Composition::CompositionLight>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlLightProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightProtected";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IXamlLightProtected_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightProtected_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlLightProtected_Vtbl {
        unsafe extern "system" fn CompositionLight<Impl: IXamlLightProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositionLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionLight<Impl: IXamlLightProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositionLight(&*(&value as *const <super::super::Composition::CompositionLight as ::windows::core::Abi>::Abi as *const <super::super::Composition::CompositionLight as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLightProtected, BASE_OFFSET>(),
            CompositionLight: CompositionLight::<Impl, IMPL_OFFSET>,
            SetCompositionLight: SetCompositionLight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLightProtected as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightStatics_Impl: Sized {
    fn AddTargetElement(&mut self, lightid: &::windows::core::HSTRING, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn RemoveTargetElement(&mut self, lightid: &::windows::core::HSTRING, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AddTargetBrush(&mut self, lightid: &::windows::core::HSTRING, brush: &::core::option::Option<Brush>) -> ::windows::core::Result<()>;
    fn RemoveTargetBrush(&mut self, lightid: &::windows::core::HSTRING, brush: &::core::option::Option<Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlLightStatics_Vtbl {
        unsafe extern "system" fn AddTargetElement<Impl: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTargetElement(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTargetElement<Impl: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetElement(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddTargetBrush<Impl: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTargetBrush(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <Brush as ::windows::core::Abi>::Abi as *const <Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTargetBrush<Impl: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetBrush(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <Brush as ::windows::core::Abi>::Abi as *const <Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLightStatics, BASE_OFFSET>(),
            AddTargetElement: AddTargetElement::<Impl, IMPL_OFFSET>,
            RemoveTargetElement: RemoveTargetElement::<Impl, IMPL_OFFSET>,
            AddTargetBrush: AddTargetBrush::<Impl, IMPL_OFFSET>,
            RemoveTargetBrush: RemoveTargetBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLightStatics as ::windows::core::Interface>::IID
    }
}
