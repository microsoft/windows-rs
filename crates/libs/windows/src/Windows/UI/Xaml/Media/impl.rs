#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushImpl: Sized {
    fn BackgroundSource(&self) -> ::windows::core::Result<AcrylicBackgroundSource>;
    fn SetBackgroundSource(&self, value: AcrylicBackgroundSource) -> ::windows::core::Result<()>;
    fn TintColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetTintColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn TintOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetTintOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn TintTransitionDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTintTransitionDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AlwaysUseFallback(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysUseFallback(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushVtbl {
    pub const fn new<Impl: IAcrylicBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAcrylicBrushVtbl {
        unsafe extern "system" fn BackgroundSource<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AcrylicBackgroundSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundSource<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AcrylicBackgroundSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBackgroundSource(value).into()
        }
        unsafe extern "system" fn TintColor<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintColor<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTintColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TintOpacity<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintOpacity<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTintOpacity(value).into()
        }
        unsafe extern "system" fn TintTransitionDuration<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintTransitionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintTransitionDuration<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTintTransitionDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlwaysUseFallback<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysUseFallback<Impl: IAcrylicBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlwaysUseFallback(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAcrylicBrush>, base.5, BackgroundSource::<Impl, OFFSET>, SetBackgroundSource::<Impl, OFFSET>, TintColor::<Impl, OFFSET>, SetTintColor::<Impl, OFFSET>, TintOpacity::<Impl, OFFSET>, SetTintOpacity::<Impl, OFFSET>, TintTransitionDuration::<Impl, OFFSET>, SetTintTransitionDuration::<Impl, OFFSET>, AlwaysUseFallback::<Impl, OFFSET>, SetAlwaysUseFallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrush2Impl: Sized {
    fn TintLuminosityOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetTintLuminosityOpacity(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrush2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrush2";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrush2Vtbl {
    pub const fn new<Impl: IAcrylicBrush2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAcrylicBrush2Vtbl {
        unsafe extern "system" fn TintLuminosityOpacity<Impl: IAcrylicBrush2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintLuminosityOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTintLuminosityOpacity<Impl: IAcrylicBrush2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTintLuminosityOpacity(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAcrylicBrush2>, base.5, TintLuminosityOpacity::<Impl, OFFSET>, SetTintLuminosityOpacity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AcrylicBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushFactoryVtbl {
    pub const fn new<Impl: IAcrylicBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAcrylicBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAcrylicBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAcrylicBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushStaticsImpl: Sized {
    fn BackgroundSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintOpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintTransitionDurationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysUseFallbackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushStaticsVtbl {
    pub const fn new<Impl: IAcrylicBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAcrylicBrushStaticsVtbl {
        unsafe extern "system" fn BackgroundSourceProperty<Impl: IAcrylicBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TintColorProperty<Impl: IAcrylicBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TintOpacityProperty<Impl: IAcrylicBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TintTransitionDurationProperty<Impl: IAcrylicBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintTransitionDurationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlwaysUseFallbackProperty<Impl: IAcrylicBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallbackProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAcrylicBrushStatics>, base.5, BackgroundSourceProperty::<Impl, OFFSET>, TintColorProperty::<Impl, OFFSET>, TintOpacityProperty::<Impl, OFFSET>, TintTransitionDurationProperty::<Impl, OFFSET>, AlwaysUseFallbackProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushStatics2Impl: Sized {
    fn TintLuminosityOpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcrylicBrushStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IAcrylicBrushStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAcrylicBrushStatics2Vtbl {
    pub const fn new<Impl: IAcrylicBrushStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAcrylicBrushStatics2Vtbl {
        unsafe extern "system" fn TintLuminosityOpacityProperty<Impl: IAcrylicBrushStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TintLuminosityOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAcrylicBrushStatics2>, base.5, TintLuminosityOpacityProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcSegmentImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn SetSize(&self, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f64>;
    fn SetRotationAngle(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsLargeArc(&self) -> ::windows::core::Result<bool>;
    fn SetIsLargeArc(&self, value: bool) -> ::windows::core::Result<()>;
    fn SweepDirection(&self) -> ::windows::core::Result<SweepDirection>;
    fn SetSweepDirection(&self, value: SweepDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IArcSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IArcSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IArcSegmentVtbl {
    pub const fn new<Impl: IArcSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IArcSegmentVtbl {
        unsafe extern "system" fn Point<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn IsLargeArc<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLargeArc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLargeArc<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsLargeArc(value).into()
        }
        unsafe extern "system" fn SweepDirection<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SweepDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SweepDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSweepDirection<Impl: IArcSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SweepDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSweepDirection(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IArcSegment>, base.5, Point::<Impl, OFFSET>, SetPoint::<Impl, OFFSET>, Size::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, RotationAngle::<Impl, OFFSET>, SetRotationAngle::<Impl, OFFSET>, IsLargeArc::<Impl, OFFSET>, SetIsLargeArc::<Impl, OFFSET>, SweepDirection::<Impl, OFFSET>, SetSweepDirection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcSegmentStaticsImpl: Sized {
    fn PointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationAngleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsLargeArcProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SweepDirectionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IArcSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IArcSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IArcSegmentStaticsVtbl {
    pub const fn new<Impl: IArcSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IArcSegmentStaticsVtbl {
        unsafe extern "system" fn PointProperty<Impl: IArcSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeProperty<Impl: IArcSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationAngleProperty<Impl: IArcSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationAngleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLargeArcProperty<Impl: IArcSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLargeArcProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SweepDirectionProperty<Impl: IArcSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SweepDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IArcSegmentStatics>, base.5, PointProperty::<Impl, OFFSET>, SizeProperty::<Impl, OFFSET>, RotationAngleProperty::<Impl, OFFSET>, IsLargeArcProperty::<Impl, OFFSET>, SweepDirectionProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBezierSegmentImpl: Sized {
    fn Point1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint1(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint2(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point3(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint3(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBezierSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IBezierSegmentVtbl {
    pub const fn new<Impl: IBezierSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBezierSegmentVtbl {
        unsafe extern "system" fn Point1<Impl: IBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint1<Impl: IBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint1(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point2<Impl: IBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint2<Impl: IBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint2(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point3<Impl: IBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint3<Impl: IBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint3(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBezierSegment>, base.5, Point1::<Impl, OFFSET>, SetPoint1::<Impl, OFFSET>, Point2::<Impl, OFFSET>, SetPoint2::<Impl, OFFSET>, Point3::<Impl, OFFSET>, SetPoint3::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBezierSegmentStaticsImpl: Sized {
    fn Point1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point3Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBezierSegmentStaticsVtbl {
    pub const fn new<Impl: IBezierSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBezierSegmentStaticsVtbl {
        unsafe extern "system" fn Point1Property<Impl: IBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point2Property<Impl: IBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point3Property<Impl: IBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point3Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBezierSegmentStatics>, base.5, Point1Property::<Impl, OFFSET>, Point2Property::<Impl, OFFSET>, Point3Property::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapCacheImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBitmapCache {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBitmapCache";
}
#[cfg(feature = "implement_exclusive")]
impl IBitmapCacheVtbl {
    pub const fn new<Impl: IBitmapCacheImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBitmapCacheVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBitmapCache>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushImpl: Sized {
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Transform(&self) -> ::windows::core::Result<Transform>;
    fn SetTransform(&self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
    fn RelativeTransform(&self) -> ::windows::core::Result<Transform>;
    fn SetRelativeTransform(&self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushVtbl {
    pub const fn new<Impl: IBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrushVtbl {
        unsafe extern "system" fn Opacity<Impl: IBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Transform<Impl: IBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&value as *const <Transform as ::windows::core::Abi>::Abi as *const <Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeTransform<Impl: IBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTransform<Impl: IBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelativeTransform(&*(&value as *const <Transform as ::windows::core::Abi>::Abi as *const <Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrush>, base.5, Opacity::<Impl, OFFSET>, SetOpacity::<Impl, OFFSET>, Transform::<Impl, OFFSET>, SetTransform::<Impl, OFFSET>, RelativeTransform::<Impl, OFFSET>, SetRelativeTransform::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Brush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushFactoryVtbl {
    pub const fn new<Impl: IBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushOverrides2Impl: Sized {
    fn PopulatePropertyInfoOverride(&self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushOverrides2Vtbl {
    pub const fn new<Impl: IBrushOverrides2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrushOverrides2Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Impl: IBrushOverrides2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfoOverride(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animationpropertyinfo as *const <super::super::Composition::AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <super::super::Composition::AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrushOverrides2>, base.5, PopulatePropertyInfoOverride::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushStaticsImpl: Sized {
    fn OpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TransformProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RelativeTransformProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushStaticsVtbl {
    pub const fn new<Impl: IBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrushStaticsVtbl {
        unsafe extern "system" fn OpacityProperty<Impl: IBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformProperty<Impl: IBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTransformProperty<Impl: IBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrushStatics>, base.5, OpacityProperty::<Impl, OFFSET>, TransformProperty::<Impl, OFFSET>, RelativeTransformProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICacheModeImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICacheMode {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICacheMode";
}
#[cfg(feature = "implement_exclusive")]
impl ICacheModeVtbl {
    pub const fn new<Impl: ICacheModeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICacheModeVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICacheMode>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICacheModeFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CacheMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICacheModeFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICacheModeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICacheModeFactoryVtbl {
    pub const fn new<Impl: ICacheModeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICacheModeFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICacheModeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICacheModeFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()>;
    fn SkewX(&self) -> ::windows::core::Result<f64>;
    fn SetSkewX(&self, value: f64) -> ::windows::core::Result<()>;
    fn SkewY(&self) -> ::windows::core::Result<f64>;
    fn SetSkewY(&self, value: f64) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<f64>;
    fn SetRotation(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateX(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateX(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateY(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositeTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeTransformVtbl {
    pub const fn new<Impl: ICompositeTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositeTransformVtbl {
        unsafe extern "system" fn CenterX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn ScaleX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleX(value).into()
        }
        unsafe extern "system" fn ScaleY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleY(value).into()
        }
        unsafe extern "system" fn SkewX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SkewX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSkewX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSkewX(value).into()
        }
        unsafe extern "system" fn SkewY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SkewY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSkewY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSkewY(value).into()
        }
        unsafe extern "system" fn Rotation<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn TranslateX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateX<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslateX(value).into()
        }
        unsafe extern "system" fn TranslateY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateY<Impl: ICompositeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslateY(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICompositeTransform>,
            base.5,
            CenterX::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            CenterY::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            ScaleX::<Impl, OFFSET>,
            SetScaleX::<Impl, OFFSET>,
            ScaleY::<Impl, OFFSET>,
            SetScaleY::<Impl, OFFSET>,
            SkewX::<Impl, OFFSET>,
            SetSkewX::<Impl, OFFSET>,
            SkewY::<Impl, OFFSET>,
            SetSkewY::<Impl, OFFSET>,
            Rotation::<Impl, OFFSET>,
            SetRotation::<Impl, OFFSET>,
            TranslateX::<Impl, OFFSET>,
            SetTranslateX::<Impl, OFFSET>,
            TranslateY::<Impl, OFFSET>,
            SetTranslateY::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SkewXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SkewYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TranslateXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TranslateYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositeTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeTransformStaticsVtbl {
    pub const fn new<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositeTransformStaticsVtbl {
        unsafe extern "system" fn CenterXProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleXProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleYProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkewXProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SkewXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkewYProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SkewYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateXProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateYProperty<Impl: ICompositeTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositeTransformStatics>, base.5, CenterXProperty::<Impl, OFFSET>, CenterYProperty::<Impl, OFFSET>, ScaleXProperty::<Impl, OFFSET>, ScaleYProperty::<Impl, OFFSET>, SkewXProperty::<Impl, OFFSET>, SkewYProperty::<Impl, OFFSET>, RotationProperty::<Impl, OFFSET>, TranslateXProperty::<Impl, OFFSET>, TranslateYProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTarget {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositionTarget";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTargetVtbl {
    pub const fn new<Impl: ICompositionTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionTargetVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionTarget>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetStaticsImpl: Sized {
    fn Rendering(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRendering(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SurfaceContentsLost(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSurfaceContentsLost(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTargetStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositionTargetStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTargetStaticsVtbl {
    pub const fn new<Impl: ICompositionTargetStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionTargetStaticsVtbl {
        unsafe extern "system" fn Rendering<Impl: ICompositionTargetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rendering(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRendering<Impl: ICompositionTargetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRendering(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SurfaceContentsLost<Impl: ICompositionTargetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SurfaceContentsLost(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSurfaceContentsLost<Impl: ICompositionTargetStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSurfaceContentsLost(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionTargetStatics>, base.5, Rendering::<Impl, OFFSET>, RemoveRendering::<Impl, OFFSET>, SurfaceContentsLost::<Impl, OFFSET>, RemoveSurfaceContentsLost::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetStatics3Impl: Sized {
    fn Rendered(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<RenderedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRendered(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTargetStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ICompositionTargetStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTargetStatics3Vtbl {
    pub const fn new<Impl: ICompositionTargetStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionTargetStatics3Vtbl {
        unsafe extern "system" fn Rendered<Impl: ICompositionTargetStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rendered(&*(&handler as *const <super::super::super::Foundation::EventHandler<RenderedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<RenderedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRendered<Impl: ICompositionTargetStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRendered(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionTargetStatics3>, base.5, Rendered::<Impl, OFFSET>, RemoveRendered::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEllipseGeometryImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetCenter(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RadiusX(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RadiusY(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEllipseGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IEllipseGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl IEllipseGeometryVtbl {
    pub const fn new<Impl: IEllipseGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEllipseGeometryVtbl {
        unsafe extern "system" fn Center<Impl: IEllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IEllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RadiusX<Impl: IEllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RadiusX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiusX<Impl: IEllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRadiusX(value).into()
        }
        unsafe extern "system" fn RadiusY<Impl: IEllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RadiusY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiusY<Impl: IEllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRadiusY(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEllipseGeometry>, base.5, Center::<Impl, OFFSET>, SetCenter::<Impl, OFFSET>, RadiusX::<Impl, OFFSET>, SetRadiusX::<Impl, OFFSET>, RadiusY::<Impl, OFFSET>, SetRadiusY::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEllipseGeometryStaticsImpl: Sized {
    fn CenterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEllipseGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IEllipseGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEllipseGeometryStaticsVtbl {
    pub const fn new<Impl: IEllipseGeometryStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEllipseGeometryStaticsVtbl {
        unsafe extern "system" fn CenterProperty<Impl: IEllipseGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RadiusXProperty<Impl: IEllipseGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RadiusXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RadiusYProperty<Impl: IEllipseGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RadiusYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEllipseGeometryStatics>, base.5, CenterProperty::<Impl, OFFSET>, RadiusXProperty::<Impl, OFFSET>, RadiusYProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontFamily {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IFontFamily";
}
#[cfg(feature = "implement_exclusive")]
impl IFontFamilyVtbl {
    pub const fn new<Impl: IFontFamilyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFontFamilyVtbl {
        unsafe extern "system" fn Source<Impl: IFontFamilyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFontFamily>, base.5, Source::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyFactoryImpl: Sized {
    fn CreateInstanceWithName(&self, familyname: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FontFamily>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontFamilyFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IFontFamilyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFontFamilyFactoryVtbl {
    pub const fn new<Impl: IFontFamilyFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFontFamilyFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithName<Impl: IFontFamilyFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, familyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithName(&*(&familyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFontFamilyFactory>, base.5, CreateInstanceWithName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyStatics2Impl: Sized {
    fn XamlAutoFontFamily(&self) -> ::windows::core::Result<FontFamily>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontFamilyStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IFontFamilyStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFontFamilyStatics2Vtbl {
    pub const fn new<Impl: IFontFamilyStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFontFamilyStatics2Vtbl {
        unsafe extern "system" fn XamlAutoFontFamily<Impl: IFontFamilyStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XamlAutoFontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFontFamilyStatics2>, base.5, XamlAutoFontFamily::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformImpl: Sized {
    fn Inverse(&self) -> ::windows::core::Result<GeneralTransform>;
    fn TransformPoint(&self, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn TryTransform(&self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneralTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneralTransformVtbl {
    pub const fn new<Impl: IGeneralTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeneralTransformVtbl {
        unsafe extern "system" fn Inverse<Impl: IGeneralTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformPoint<Impl: IGeneralTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformPoint(&*(&point as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<Impl: IGeneralTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryTransform(&*(&inpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBounds<Impl: IGeneralTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformBounds(&*(&rect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeneralTransform>, base.5, Inverse::<Impl, OFFSET>, TransformPoint::<Impl, OFFSET>, TryTransform::<Impl, OFFSET>, TransformBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GeneralTransform>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneralTransformFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransformFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneralTransformFactoryVtbl {
    pub const fn new<Impl: IGeneralTransformFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeneralTransformFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGeneralTransformFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeneralTransformFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformOverridesImpl: Sized {
    fn InverseCore(&self) -> ::windows::core::Result<GeneralTransform>;
    fn TryTransformCore(&self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBoundsCore(&self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneralTransformOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransformOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneralTransformOverridesVtbl {
    pub const fn new<Impl: IGeneralTransformOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeneralTransformOverridesVtbl {
        unsafe extern "system" fn InverseCore<Impl: IGeneralTransformOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InverseCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformCore<Impl: IGeneralTransformOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryTransformCore(&*(&inpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBoundsCore<Impl: IGeneralTransformOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformBoundsCore(&*(&rect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeneralTransformOverrides>, base.5, InverseCore::<Impl, OFFSET>, TryTransformCore::<Impl, OFFSET>, TransformBoundsCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryImpl: Sized {
    fn Transform(&self) -> ::windows::core::Result<Transform>;
    fn SetTransform(&self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryVtbl {
    pub const fn new<Impl: IGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeometryVtbl {
        unsafe extern "system" fn Transform<Impl: IGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&value as *const <Transform as ::windows::core::Abi>::Abi as *const <Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bounds<Impl: IGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeometry>, base.5, Transform::<Impl, OFFSET>, SetTransform::<Impl, OFFSET>, Bounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryFactoryVtbl {
    pub const fn new<Impl: IGeometryFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeometryFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeometryFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryGroupImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<FillRule>;
    fn SetFillRule(&self, value: FillRule) -> ::windows::core::Result<()>;
    fn Children(&self) -> ::windows::core::Result<GeometryCollection>;
    fn SetChildren(&self, value: &::core::option::Option<GeometryCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryGroupVtbl {
    pub const fn new<Impl: IGeometryGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeometryGroupVtbl {
        unsafe extern "system" fn FillRule<Impl: IGeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IGeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFillRule(value).into()
        }
        unsafe extern "system" fn Children<Impl: IGeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildren<Impl: IGeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChildren(&*(&value as *const <GeometryCollection as ::windows::core::Abi>::Abi as *const <GeometryCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeometryGroup>, base.5, FillRule::<Impl, OFFSET>, SetFillRule::<Impl, OFFSET>, Children::<Impl, OFFSET>, SetChildren::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryGroupStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryGroupStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryGroupStaticsVtbl {
    pub const fn new<Impl: IGeometryGroupStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeometryGroupStaticsVtbl {
        unsafe extern "system" fn FillRuleProperty<Impl: IGeometryGroupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillRuleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildrenProperty<Impl: IGeometryGroupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildrenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeometryGroupStatics>, base.5, FillRuleProperty::<Impl, OFFSET>, ChildrenProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryStaticsImpl: Sized {
    fn Empty(&self) -> ::windows::core::Result<Geometry>;
    fn StandardFlatteningTolerance(&self) -> ::windows::core::Result<f64>;
    fn TransformProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeometryStaticsVtbl {
    pub const fn new<Impl: IGeometryStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeometryStaticsVtbl {
        unsafe extern "system" fn Empty<Impl: IGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StandardFlatteningTolerance<Impl: IGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StandardFlatteningTolerance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformProperty<Impl: IGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeometryStatics>, base.5, Empty::<Impl, OFFSET>, StandardFlatteningTolerance::<Impl, OFFSET>, TransformProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushImpl: Sized {
    fn SpreadMethod(&self) -> ::windows::core::Result<GradientSpreadMethod>;
    fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows::core::Result<()>;
    fn MappingMode(&self) -> ::windows::core::Result<BrushMappingMode>;
    fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows::core::Result<()>;
    fn ColorInterpolationMode(&self) -> ::windows::core::Result<ColorInterpolationMode>;
    fn SetColorInterpolationMode(&self, value: ColorInterpolationMode) -> ::windows::core::Result<()>;
    fn GradientStops(&self) -> ::windows::core::Result<GradientStopCollection>;
    fn SetGradientStops(&self, value: &::core::option::Option<GradientStopCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientBrushVtbl {
    pub const fn new<Impl: IGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGradientBrushVtbl {
        unsafe extern "system" fn SpreadMethod<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GradientSpreadMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpreadMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: GradientSpreadMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpreadMethod(value).into()
        }
        unsafe extern "system" fn MappingMode<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BrushMappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MappingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMappingMode<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BrushMappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMappingMode(value).into()
        }
        unsafe extern "system" fn ColorInterpolationMode<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ColorInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ColorInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColorInterpolationMode(value).into()
        }
        unsafe extern "system" fn GradientStops<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GradientStops() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientStops<Impl: IGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGradientStops(&*(&value as *const <GradientStopCollection as ::windows::core::Abi>::Abi as *const <GradientStopCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGradientBrush>, base.5, SpreadMethod::<Impl, OFFSET>, SetSpreadMethod::<Impl, OFFSET>, MappingMode::<Impl, OFFSET>, SetMappingMode::<Impl, OFFSET>, ColorInterpolationMode::<Impl, OFFSET>, SetColorInterpolationMode::<Impl, OFFSET>, GradientStops::<Impl, OFFSET>, SetGradientStops::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientBrushFactoryVtbl {
    pub const fn new<Impl: IGradientBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGradientBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGradientBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGradientBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushStaticsImpl: Sized {
    fn SpreadMethodProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MappingModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorInterpolationModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GradientStopsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientBrushStaticsVtbl {
    pub const fn new<Impl: IGradientBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGradientBrushStaticsVtbl {
        unsafe extern "system" fn SpreadMethodProperty<Impl: IGradientBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpreadMethodProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MappingModeProperty<Impl: IGradientBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MappingModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorInterpolationModeProperty<Impl: IGradientBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorInterpolationModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GradientStopsProperty<Impl: IGradientBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GradientStopsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGradientBrushStatics>, base.5, SpreadMethodProperty::<Impl, OFFSET>, MappingModeProperty::<Impl, OFFSET>, ColorInterpolationModeProperty::<Impl, OFFSET>, GradientStopsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientStopImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<f64>;
    fn SetOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientStop {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientStop";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientStopVtbl {
    pub const fn new<Impl: IGradientStopImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGradientStopVtbl {
        unsafe extern "system" fn Color<Impl: IGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: IGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGradientStop>, base.5, Color::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, Offset::<Impl, OFFSET>, SetOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientStopStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGradientStopStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGradientStopStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGradientStopStaticsVtbl {
    pub const fn new<Impl: IGradientStopStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGradientStopStaticsVtbl {
        unsafe extern "system" fn ColorProperty<Impl: IGradientStopStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetProperty<Impl: IGradientStopStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGradientStopStatics>, base.5, ColorProperty::<Impl, OFFSET>, OffsetProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageBrushImpl: Sized {
    fn ImageSource(&self) -> ::windows::core::Result<ImageSource>;
    fn SetImageSource(&self, value: &::core::option::Option<ImageSource>) -> ::windows::core::Result<()>;
    fn ImageFailed(&self, handler: &::core::option::Option<super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IImageBrushVtbl {
    pub const fn new<Impl: IImageBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageBrushVtbl {
        unsafe extern "system" fn ImageSource<Impl: IImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageSource<Impl: IImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetImageSource(&*(&value as *const <ImageSource as ::windows::core::Abi>::Abi as *const <ImageSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageFailed<Impl: IImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageFailed(&*(&handler as *const <super::ExceptionRoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::ExceptionRoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageFailed<Impl: IImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveImageFailed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageOpened<Impl: IImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageOpened(&*(&handler as *const <super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageOpened<Impl: IImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveImageOpened(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageBrush>, base.5, ImageSource::<Impl, OFFSET>, SetImageSource::<Impl, OFFSET>, ImageFailed::<Impl, OFFSET>, RemoveImageFailed::<Impl, OFFSET>, ImageOpened::<Impl, OFFSET>, RemoveImageOpened::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageBrushStaticsImpl: Sized {
    fn ImageSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IImageBrushStaticsVtbl {
    pub const fn new<Impl: IImageBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageBrushStaticsVtbl {
        unsafe extern "system" fn ImageSourceProperty<Impl: IImageBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageBrushStatics>, base.5, ImageSourceProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageSource";
}
#[cfg(feature = "implement_exclusive")]
impl IImageSourceVtbl {
    pub const fn new<Impl: IImageSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageSourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageSource>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageSourceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IImageSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IImageSourceFactoryVtbl {
    pub const fn new<Impl: IImageSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageSourceFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageSourceFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineGeometryImpl: Sized {
    fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn EndPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetEndPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl ILineGeometryVtbl {
    pub const fn new<Impl: ILineGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineGeometryVtbl {
        unsafe extern "system" fn StartPoint<Impl: ILineGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: ILineGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPoint<Impl: ILineGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: ILineGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineGeometry>, base.5, StartPoint::<Impl, OFFSET>, SetStartPoint::<Impl, OFFSET>, EndPoint::<Impl, OFFSET>, SetEndPoint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineGeometryStaticsImpl: Sized {
    fn StartPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn EndPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILineGeometryStaticsVtbl {
    pub const fn new<Impl: ILineGeometryStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineGeometryStaticsVtbl {
        unsafe extern "system" fn StartPointProperty<Impl: ILineGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPointProperty<Impl: ILineGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineGeometryStatics>, base.5, StartPointProperty::<Impl, OFFSET>, EndPointProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineSegmentImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineSegment";
}
#[cfg(feature = "implement_exclusive")]
impl ILineSegmentVtbl {
    pub const fn new<Impl: ILineSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineSegmentVtbl {
        unsafe extern "system" fn Point<Impl: ILineSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint<Impl: ILineSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineSegment>, base.5, Point::<Impl, OFFSET>, SetPoint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineSegmentStaticsImpl: Sized {
    fn PointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILineSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILineSegmentStaticsVtbl {
    pub const fn new<Impl: ILineSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineSegmentStaticsVtbl {
        unsafe extern "system" fn PointProperty<Impl: ILineSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineSegmentStatics>, base.5, PointProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushImpl: Sized {
    fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn EndPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetEndPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILinearGradientBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearGradientBrushVtbl {
    pub const fn new<Impl: ILinearGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILinearGradientBrushVtbl {
        unsafe extern "system" fn StartPoint<Impl: ILinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: ILinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPoint<Impl: ILinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: ILinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILinearGradientBrush>, base.5, StartPoint::<Impl, OFFSET>, SetStartPoint::<Impl, OFFSET>, EndPoint::<Impl, OFFSET>, SetEndPoint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushFactoryImpl: Sized {
    fn CreateInstanceWithGradientStopCollectionAndAngle(&self, gradientstopcollection: &::core::option::Option<GradientStopCollection>, angle: f64) -> ::windows::core::Result<LinearGradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearGradientBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILinearGradientBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearGradientBrushFactoryVtbl {
    pub const fn new<Impl: ILinearGradientBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILinearGradientBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithGradientStopCollectionAndAngle<Impl: ILinearGradientBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstopcollection: ::windows::core::RawPtr, angle: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithGradientStopCollectionAndAngle(&*(&gradientstopcollection as *const <GradientStopCollection as ::windows::core::Abi>::Abi as *const <GradientStopCollection as ::windows::core::DefaultType>::DefaultType), angle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILinearGradientBrushFactory>, base.5, CreateInstanceWithGradientStopCollectionAndAngle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushStaticsImpl: Sized {
    fn StartPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn EndPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearGradientBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILinearGradientBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearGradientBrushStaticsVtbl {
    pub const fn new<Impl: ILinearGradientBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILinearGradientBrushStaticsVtbl {
        unsafe extern "system" fn StartPointProperty<Impl: ILinearGradientBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPointProperty<Impl: ILinearGradientBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILinearGradientBrushStatics>, base.5, StartPointProperty::<Impl, OFFSET>, EndPointProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSourceLoadCompletedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LoadedImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoadedImageSourceLoadCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILoadedImageSourceLoadCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILoadedImageSourceLoadCompletedEventArgsVtbl {
    pub const fn new<Impl: ILoadedImageSourceLoadCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoadedImageSourceLoadCompletedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ILoadedImageSourceLoadCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LoadedImageSourceLoadStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoadedImageSourceLoadCompletedEventArgs>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSurfaceImpl: Sized {
    fn DecodedPhysicalSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn DecodedSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn NaturalSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn LoadCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoadedImageSurface {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILoadedImageSurface";
}
#[cfg(feature = "implement_exclusive")]
impl ILoadedImageSurfaceVtbl {
    pub const fn new<Impl: ILoadedImageSurfaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoadedImageSurfaceVtbl {
        unsafe extern "system" fn DecodedPhysicalSize<Impl: ILoadedImageSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecodedPhysicalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodedSize<Impl: ILoadedImageSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecodedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalSize<Impl: ILoadedImageSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCompleted<Impl: ILoadedImageSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoadCompleted<Impl: ILoadedImageSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLoadCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoadedImageSurface>, base.5, DecodedPhysicalSize::<Impl, OFFSET>, DecodedSize::<Impl, OFFSET>, NaturalSize::<Impl, OFFSET>, LoadCompleted::<Impl, OFFSET>, RemoveLoadCompleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSurfaceStaticsImpl: Sized {
    fn StartLoadFromUriWithSize(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, desiredmaxsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromUri(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromStreamWithSize(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>, desiredmaxsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromStream(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<LoadedImageSurface>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoadedImageSurfaceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ILoadedImageSurfaceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILoadedImageSurfaceStaticsVtbl {
    pub const fn new<Impl: ILoadedImageSurfaceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoadedImageSurfaceStaticsVtbl {
        unsafe extern "system" fn StartLoadFromUriWithSize<Impl: ILoadedImageSurfaceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartLoadFromUriWithSize(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&desiredmaxsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartLoadFromUri<Impl: ILoadedImageSurfaceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartLoadFromUri(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartLoadFromStreamWithSize<Impl: ILoadedImageSurfaceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartLoadFromStreamWithSize(&*(&stream as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&desiredmaxsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartLoadFromStream<Impl: ILoadedImageSurfaceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartLoadFromStream(&*(&stream as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoadedImageSurfaceStatics>, base.5, StartLoadFromUriWithSize::<Impl, OFFSET>, StartLoadFromUri::<Impl, OFFSET>, StartLoadFromStreamWithSize::<Impl, OFFSET>, StartLoadFromStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DProjectionImpl: Sized {
    fn ProjectionMatrix(&self) -> ::windows::core::Result<Media3D::Matrix3D>;
    fn SetProjectionMatrix(&self, value: &Media3D::Matrix3D) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrix3DProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrix3DProjection";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrix3DProjectionVtbl {
    pub const fn new<Impl: IMatrix3DProjectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMatrix3DProjectionVtbl {
        unsafe extern "system" fn ProjectionMatrix<Impl: IMatrix3DProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjectionMatrix<Impl: IMatrix3DProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProjectionMatrix(&*(&value as *const <Media3D::Matrix3D as ::windows::core::Abi>::Abi as *const <Media3D::Matrix3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMatrix3DProjection>, base.5, ProjectionMatrix::<Impl, OFFSET>, SetProjectionMatrix::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DProjectionStaticsImpl: Sized {
    fn ProjectionMatrixProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrix3DProjectionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrix3DProjectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrix3DProjectionStaticsVtbl {
    pub const fn new<Impl: IMatrix3DProjectionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMatrix3DProjectionStaticsVtbl {
        unsafe extern "system" fn ProjectionMatrixProperty<Impl: IMatrix3DProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrixProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMatrix3DProjectionStatics>, base.5, ProjectionMatrixProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixHelperVtbl {
    pub const fn new<Impl: IMatrixHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMatrixHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMatrixHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixHelperStaticsImpl: Sized {
    fn Identity(&self) -> ::windows::core::Result<Matrix>;
    fn FromElements(&self, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64) -> ::windows::core::Result<Matrix>;
    fn GetIsIdentity(&self, target: &Matrix) -> ::windows::core::Result<bool>;
    fn Transform(&self, target: &Matrix, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixHelperStaticsVtbl {
    pub const fn new<Impl: IMatrixHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMatrixHelperStaticsVtbl {
        unsafe extern "system" fn Identity<Impl: IMatrixHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromElements<Impl: IMatrixHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromElements(m11, m12, m21, m22, offsetx, offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsIdentity<Impl: IMatrixHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: Matrix, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsIdentity(&*(&target as *const <Matrix as ::windows::core::Abi>::Abi as *const <Matrix as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Impl: IMatrixHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: Matrix, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transform(&*(&target as *const <Matrix as ::windows::core::Abi>::Abi as *const <Matrix as ::windows::core::DefaultType>::DefaultType), &*(&point as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMatrixHelperStatics>, base.5, Identity::<Impl, OFFSET>, FromElements::<Impl, OFFSET>, GetIsIdentity::<Impl, OFFSET>, Transform::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixTransformImpl: Sized {
    fn Matrix(&self) -> ::windows::core::Result<Matrix>;
    fn SetMatrix(&self, value: &Matrix) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixTransformVtbl {
    pub const fn new<Impl: IMatrixTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMatrixTransformVtbl {
        unsafe extern "system" fn Matrix<Impl: IMatrixTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Matrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Impl: IMatrixTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMatrix(&*(&value as *const <Matrix as ::windows::core::Abi>::Abi as *const <Matrix as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMatrixTransform>, base.5, Matrix::<Impl, OFFSET>, SetMatrix::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixTransformStaticsImpl: Sized {
    fn MatrixProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrixTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMatrixTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrixTransformStaticsVtbl {
    pub const fn new<Impl: IMatrixTransformStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMatrixTransformStaticsVtbl {
        unsafe extern "system" fn MatrixProperty<Impl: IMatrixTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MatrixProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMatrixTransformStatics>, base.5, MatrixProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsThumbnailRequestedEventArgsImpl: Sized {
    fn SetThumbnailImage(&self, source: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTransportControlsThumbnailRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IMediaTransportControlsThumbnailRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTransportControlsThumbnailRequestedEventArgsVtbl {
    pub const fn new<Impl: IMediaTransportControlsThumbnailRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTransportControlsThumbnailRequestedEventArgsVtbl {
        unsafe extern "system" fn SetThumbnailImage<Impl: IMediaTransportControlsThumbnailRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnailImage(&*(&source as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaTransportControlsThumbnailRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTransportControlsThumbnailRequestedEventArgs>, base.5, SetThumbnailImage::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPartialMediaFailureDetectedEventArgsImpl: Sized {
    fn StreamKind(&self) -> ::windows::core::Result<super::super::super::Media::Playback::FailedMediaStreamKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPartialMediaFailureDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPartialMediaFailureDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPartialMediaFailureDetectedEventArgsVtbl {
    pub const fn new<Impl: IPartialMediaFailureDetectedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPartialMediaFailureDetectedEventArgsVtbl {
        unsafe extern "system" fn StreamKind<Impl: IPartialMediaFailureDetectedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Media::Playback::FailedMediaStreamKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPartialMediaFailureDetectedEventArgs>, base.5, StreamKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPartialMediaFailureDetectedEventArgs2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPartialMediaFailureDetectedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPartialMediaFailureDetectedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IPartialMediaFailureDetectedEventArgs2Vtbl {
    pub const fn new<Impl: IPartialMediaFailureDetectedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPartialMediaFailureDetectedEventArgs2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IPartialMediaFailureDetectedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPartialMediaFailureDetectedEventArgs2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFigureImpl: Sized {
    fn Segments(&self) -> ::windows::core::Result<PathSegmentCollection>;
    fn SetSegments(&self, value: &::core::option::Option<PathSegmentCollection>) -> ::windows::core::Result<()>;
    fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsClosed(&self) -> ::windows::core::Result<bool>;
    fn SetIsClosed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFilled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFilled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathFigure {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathFigure";
}
#[cfg(feature = "implement_exclusive")]
impl IPathFigureVtbl {
    pub const fn new<Impl: IPathFigureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPathFigureVtbl {
        unsafe extern "system" fn Segments<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Segments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegments<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSegments(&*(&value as *const <PathSegmentCollection as ::windows::core::Abi>::Abi as *const <PathSegmentCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPoint<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsClosed<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsClosed(value).into()
        }
        unsafe extern "system" fn IsFilled<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFilled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Impl: IPathFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsFilled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPathFigure>, base.5, Segments::<Impl, OFFSET>, SetSegments::<Impl, OFFSET>, StartPoint::<Impl, OFFSET>, SetStartPoint::<Impl, OFFSET>, IsClosed::<Impl, OFFSET>, SetIsClosed::<Impl, OFFSET>, IsFilled::<Impl, OFFSET>, SetIsFilled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFigureStaticsImpl: Sized {
    fn SegmentsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StartPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsClosedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFilledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathFigureStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathFigureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPathFigureStaticsVtbl {
    pub const fn new<Impl: IPathFigureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPathFigureStaticsVtbl {
        unsafe extern "system" fn SegmentsProperty<Impl: IPathFigureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SegmentsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPointProperty<Impl: IPathFigureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPointProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClosedProperty<Impl: IPathFigureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClosedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFilledProperty<Impl: IPathFigureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFilledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPathFigureStatics>, base.5, SegmentsProperty::<Impl, OFFSET>, StartPointProperty::<Impl, OFFSET>, IsClosedProperty::<Impl, OFFSET>, IsFilledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathGeometryImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<FillRule>;
    fn SetFillRule(&self, value: FillRule) -> ::windows::core::Result<()>;
    fn Figures(&self) -> ::windows::core::Result<PathFigureCollection>;
    fn SetFigures(&self, value: &::core::option::Option<PathFigureCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl IPathGeometryVtbl {
    pub const fn new<Impl: IPathGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPathGeometryVtbl {
        unsafe extern "system" fn FillRule<Impl: IPathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IPathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FillRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFillRule(value).into()
        }
        unsafe extern "system" fn Figures<Impl: IPathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Figures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFigures<Impl: IPathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFigures(&*(&value as *const <PathFigureCollection as ::windows::core::Abi>::Abi as *const <PathFigureCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPathGeometry>, base.5, FillRule::<Impl, OFFSET>, SetFillRule::<Impl, OFFSET>, Figures::<Impl, OFFSET>, SetFigures::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathGeometryStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FiguresProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPathGeometryStaticsVtbl {
    pub const fn new<Impl: IPathGeometryStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPathGeometryStaticsVtbl {
        unsafe extern "system" fn FillRuleProperty<Impl: IPathGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillRuleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FiguresProperty<Impl: IPathGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FiguresProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPathGeometryStatics>, base.5, FillRuleProperty::<Impl, OFFSET>, FiguresProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathSegmentImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IPathSegmentVtbl {
    pub const fn new<Impl: IPathSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPathSegmentVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPathSegment>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathSegmentFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathSegmentFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPathSegmentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPathSegmentFactoryVtbl {
    pub const fn new<Impl: IPathSegmentFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPathSegmentFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPathSegmentFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaneProjectionImpl: Sized {
    fn LocalOffsetX(&self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn LocalOffsetY(&self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetY(&self, value: f64) -> ::windows::core::Result<()>;
    fn LocalOffsetZ(&self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationX(&self) -> ::windows::core::Result<f64>;
    fn SetRotationX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationY(&self) -> ::windows::core::Result<f64>;
    fn SetRotationY(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationZ(&self) -> ::windows::core::Result<f64>;
    fn SetRotationZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationY(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationZ(&self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetX(&self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetY(&self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetY(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetZ(&self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn ProjectionMatrix(&self) -> ::windows::core::Result<Media3D::Matrix3D>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaneProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPlaneProjection";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaneProjectionVtbl {
    pub const fn new<Impl: IPlaneProjectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaneProjectionVtbl {
        unsafe extern "system" fn LocalOffsetX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalOffsetX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalOffsetX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocalOffsetX(value).into()
        }
        unsafe extern "system" fn LocalOffsetY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalOffsetY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalOffsetY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocalOffsetY(value).into()
        }
        unsafe extern "system" fn LocalOffsetZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalOffsetZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalOffsetZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocalOffsetZ(value).into()
        }
        unsafe extern "system" fn RotationX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationX(value).into()
        }
        unsafe extern "system" fn RotationY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationY(value).into()
        }
        unsafe extern "system" fn RotationZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationZ(value).into()
        }
        unsafe extern "system" fn CenterOfRotationX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterOfRotationX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterOfRotationX(value).into()
        }
        unsafe extern "system" fn CenterOfRotationY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterOfRotationY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterOfRotationY(value).into()
        }
        unsafe extern "system" fn CenterOfRotationZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterOfRotationZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterOfRotationZ(value).into()
        }
        unsafe extern "system" fn GlobalOffsetX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalOffsetX<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGlobalOffsetX(value).into()
        }
        unsafe extern "system" fn GlobalOffsetY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalOffsetY<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGlobalOffsetY(value).into()
        }
        unsafe extern "system" fn GlobalOffsetZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalOffsetZ<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGlobalOffsetZ(value).into()
        }
        unsafe extern "system" fn ProjectionMatrix<Impl: IPlaneProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPlaneProjection>,
            base.5,
            LocalOffsetX::<Impl, OFFSET>,
            SetLocalOffsetX::<Impl, OFFSET>,
            LocalOffsetY::<Impl, OFFSET>,
            SetLocalOffsetY::<Impl, OFFSET>,
            LocalOffsetZ::<Impl, OFFSET>,
            SetLocalOffsetZ::<Impl, OFFSET>,
            RotationX::<Impl, OFFSET>,
            SetRotationX::<Impl, OFFSET>,
            RotationY::<Impl, OFFSET>,
            SetRotationY::<Impl, OFFSET>,
            RotationZ::<Impl, OFFSET>,
            SetRotationZ::<Impl, OFFSET>,
            CenterOfRotationX::<Impl, OFFSET>,
            SetCenterOfRotationX::<Impl, OFFSET>,
            CenterOfRotationY::<Impl, OFFSET>,
            SetCenterOfRotationY::<Impl, OFFSET>,
            CenterOfRotationZ::<Impl, OFFSET>,
            SetCenterOfRotationZ::<Impl, OFFSET>,
            GlobalOffsetX::<Impl, OFFSET>,
            SetGlobalOffsetX::<Impl, OFFSET>,
            GlobalOffsetY::<Impl, OFFSET>,
            SetGlobalOffsetY::<Impl, OFFSET>,
            GlobalOffsetZ::<Impl, OFFSET>,
            SetGlobalOffsetZ::<Impl, OFFSET>,
            ProjectionMatrix::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaneProjectionStaticsImpl: Sized {
    fn LocalOffsetXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LocalOffsetYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LocalOffsetZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProjectionMatrixProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaneProjectionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPlaneProjectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaneProjectionStaticsVtbl {
    pub const fn new<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaneProjectionStaticsVtbl {
        unsafe extern "system" fn LocalOffsetXProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalOffsetXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalOffsetYProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalOffsetYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalOffsetZProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalOffsetZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationXProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationYProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationZProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterOfRotationXProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterOfRotationYProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterOfRotationZProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterOfRotationZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalOffsetXProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalOffsetYProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalOffsetZProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlobalOffsetZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionMatrixProperty<Impl: IPlaneProjectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionMatrixProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPlaneProjectionStatics>,
            base.5,
            LocalOffsetXProperty::<Impl, OFFSET>,
            LocalOffsetYProperty::<Impl, OFFSET>,
            LocalOffsetZProperty::<Impl, OFFSET>,
            RotationXProperty::<Impl, OFFSET>,
            RotationYProperty::<Impl, OFFSET>,
            RotationZProperty::<Impl, OFFSET>,
            CenterOfRotationXProperty::<Impl, OFFSET>,
            CenterOfRotationYProperty::<Impl, OFFSET>,
            CenterOfRotationZProperty::<Impl, OFFSET>,
            GlobalOffsetXProperty::<Impl, OFFSET>,
            GlobalOffsetYProperty::<Impl, OFFSET>,
            GlobalOffsetZProperty::<Impl, OFFSET>,
            ProjectionMatrixProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyBezierSegmentImpl: Sized {
    fn Points(&self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyBezierSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyBezierSegmentVtbl {
    pub const fn new<Impl: IPolyBezierSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPolyBezierSegmentVtbl {
        unsafe extern "system" fn Points<Impl: IPolyBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Impl: IPolyBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <PointCollection as ::windows::core::Abi>::Abi as *const <PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPolyBezierSegment>, base.5, Points::<Impl, OFFSET>, SetPoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyBezierSegmentStaticsImpl: Sized {
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyBezierSegmentStaticsVtbl {
    pub const fn new<Impl: IPolyBezierSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPolyBezierSegmentStaticsVtbl {
        unsafe extern "system" fn PointsProperty<Impl: IPolyBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPolyBezierSegmentStatics>, base.5, PointsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyLineSegmentImpl: Sized {
    fn Points(&self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyLineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyLineSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyLineSegmentVtbl {
    pub const fn new<Impl: IPolyLineSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPolyLineSegmentVtbl {
        unsafe extern "system" fn Points<Impl: IPolyLineSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Impl: IPolyLineSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <PointCollection as ::windows::core::Abi>::Abi as *const <PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPolyLineSegment>, base.5, Points::<Impl, OFFSET>, SetPoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyLineSegmentStaticsImpl: Sized {
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyLineSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyLineSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyLineSegmentStaticsVtbl {
    pub const fn new<Impl: IPolyLineSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPolyLineSegmentStaticsVtbl {
        unsafe extern "system" fn PointsProperty<Impl: IPolyLineSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPolyLineSegmentStatics>, base.5, PointsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyQuadraticBezierSegmentImpl: Sized {
    fn Points(&self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyQuadraticBezierSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyQuadraticBezierSegmentVtbl {
    pub const fn new<Impl: IPolyQuadraticBezierSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPolyQuadraticBezierSegmentVtbl {
        unsafe extern "system" fn Points<Impl: IPolyQuadraticBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Impl: IPolyQuadraticBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoints(&*(&value as *const <PointCollection as ::windows::core::Abi>::Abi as *const <PointCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPolyQuadraticBezierSegment>, base.5, Points::<Impl, OFFSET>, SetPoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyQuadraticBezierSegmentStaticsImpl: Sized {
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPolyQuadraticBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IPolyQuadraticBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPolyQuadraticBezierSegmentStaticsVtbl {
    pub const fn new<Impl: IPolyQuadraticBezierSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPolyQuadraticBezierSegmentStaticsVtbl {
        unsafe extern "system" fn PointsProperty<Impl: IPolyQuadraticBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPolyQuadraticBezierSegmentStatics>, base.5, PointsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IProjection";
}
#[cfg(feature = "implement_exclusive")]
impl IProjectionVtbl {
    pub const fn new<Impl: IProjectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProjectionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProjection>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Projection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProjectionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IProjectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProjectionFactoryVtbl {
    pub const fn new<Impl: IProjectionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProjectionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IProjectionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProjectionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticBezierSegmentImpl: Sized {
    fn Point1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint1(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint2(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IQuadraticBezierSegment";
}
#[cfg(feature = "implement_exclusive")]
impl IQuadraticBezierSegmentVtbl {
    pub const fn new<Impl: IQuadraticBezierSegmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQuadraticBezierSegmentVtbl {
        unsafe extern "system" fn Point1<Impl: IQuadraticBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint1<Impl: IQuadraticBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint1(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point2<Impl: IQuadraticBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint2<Impl: IQuadraticBezierSegmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPoint2(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQuadraticBezierSegment>, base.5, Point1::<Impl, OFFSET>, SetPoint1::<Impl, OFFSET>, Point2::<Impl, OFFSET>, SetPoint2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticBezierSegmentStaticsImpl: Sized {
    fn Point1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuadraticBezierSegmentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IQuadraticBezierSegmentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IQuadraticBezierSegmentStaticsVtbl {
    pub const fn new<Impl: IQuadraticBezierSegmentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQuadraticBezierSegmentStaticsVtbl {
        unsafe extern "system" fn Point1Property<Impl: IQuadraticBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point1Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point2Property<Impl: IQuadraticBezierSegmentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point2Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQuadraticBezierSegmentStatics>, base.5, Point1Property::<Impl, OFFSET>, Point2Property::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRateChangedRoutedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRateChangedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRateChangedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRateChangedRoutedEventArgsVtbl {
    pub const fn new<Impl: IRateChangedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRateChangedRoutedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRateChangedRoutedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleGeometryImpl: Sized {
    fn Rect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetRect(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectangleGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRectangleGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl IRectangleGeometryVtbl {
    pub const fn new<Impl: IRectangleGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRectangleGeometryVtbl {
        unsafe extern "system" fn Rect<Impl: IRectangleGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Impl: IRectangleGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRectangleGeometry>, base.5, Rect::<Impl, OFFSET>, SetRect::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleGeometryStaticsImpl: Sized {
    fn RectProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectangleGeometryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRectangleGeometryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRectangleGeometryStaticsVtbl {
    pub const fn new<Impl: IRectangleGeometryStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRectangleGeometryStaticsVtbl {
        unsafe extern "system" fn RectProperty<Impl: IRectangleGeometryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRectangleGeometryStatics>, base.5, RectProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderedEventArgsImpl: Sized {
    fn FrameDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRenderedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderedEventArgsVtbl {
    pub const fn new<Impl: IRenderedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRenderedEventArgsVtbl {
        unsafe extern "system" fn FrameDuration<Impl: IRenderedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRenderedEventArgs>, base.5, FrameDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderingEventArgsImpl: Sized {
    fn RenderingTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRenderingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderingEventArgsVtbl {
    pub const fn new<Impl: IRenderingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRenderingEventArgsVtbl {
        unsafe extern "system" fn RenderingTime<Impl: IRenderingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRenderingEventArgs>, base.5, RenderingTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBackgroundBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBackgroundBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBackgroundBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBackgroundBrushVtbl {
    pub const fn new<Impl: IRevealBackgroundBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBackgroundBrushVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBackgroundBrush>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBackgroundBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBackgroundBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBackgroundBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBackgroundBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBackgroundBrushFactoryVtbl {
    pub const fn new<Impl: IRevealBackgroundBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBackgroundBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRevealBackgroundBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBackgroundBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBorderBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBorderBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBorderBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBorderBrushVtbl {
    pub const fn new<Impl: IRevealBorderBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBorderBrushVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBorderBrush>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBorderBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBorderBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBorderBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBorderBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBorderBrushFactoryVtbl {
    pub const fn new<Impl: IRevealBorderBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBorderBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRevealBorderBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBorderBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn TargetTheme(&self) -> ::windows::core::Result<super::ApplicationTheme>;
    fn SetTargetTheme(&self, value: super::ApplicationTheme) -> ::windows::core::Result<()>;
    fn AlwaysUseFallback(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysUseFallback(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBrush";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBrushVtbl {
    pub const fn new<Impl: IRevealBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBrushVtbl {
        unsafe extern "system" fn Color<Impl: IRevealBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IRevealBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetTheme<Impl: IRevealBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetTheme<Impl: IRevealBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetTheme(value).into()
        }
        unsafe extern "system" fn AlwaysUseFallback<Impl: IRevealBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysUseFallback<Impl: IRevealBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlwaysUseFallback(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBrush>, base.5, Color::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, TargetTheme::<Impl, OFFSET>, SetTargetTheme::<Impl, OFFSET>, AlwaysUseFallback::<Impl, OFFSET>, SetAlwaysUseFallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBrushFactoryVtbl {
    pub const fn new<Impl: IRevealBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRevealBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TargetThemeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysUseFallbackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SetState(&self, element: &::core::option::Option<super::UIElement>, value: RevealBrushState) -> ::windows::core::Result<()>;
    fn GetState(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<RevealBrushState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevealBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRevealBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRevealBrushStaticsVtbl {
    pub const fn new<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRevealBrushStaticsVtbl {
        unsafe extern "system" fn ColorProperty<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetThemeProperty<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetThemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlwaysUseFallbackProperty<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysUseFallbackProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateProperty<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: RevealBrushState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetState(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn GetState<Impl: IRevealBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut RevealBrushState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetState(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRevealBrushStatics>, base.5, ColorProperty::<Impl, OFFSET>, TargetThemeProperty::<Impl, OFFSET>, AlwaysUseFallbackProperty::<Impl, OFFSET>, StateProperty::<Impl, OFFSET>, SetState::<Impl, OFFSET>, GetState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRotateTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn Angle(&self) -> ::windows::core::Result<f64>;
    fn SetAngle(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRotateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRotateTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IRotateTransformVtbl {
    pub const fn new<Impl: IRotateTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRotateTransformVtbl {
        unsafe extern "system" fn CenterX<Impl: IRotateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IRotateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: IRotateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IRotateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn Angle<Impl: IRotateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Angle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngle<Impl: IRotateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAngle(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRotateTransform>, base.5, CenterX::<Impl, OFFSET>, SetCenterX::<Impl, OFFSET>, CenterY::<Impl, OFFSET>, SetCenterY::<Impl, OFFSET>, Angle::<Impl, OFFSET>, SetAngle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRotateTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRotateTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IRotateTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRotateTransformStaticsVtbl {
    pub const fn new<Impl: IRotateTransformStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRotateTransformStaticsVtbl {
        unsafe extern "system" fn CenterXProperty<Impl: IRotateTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: IRotateTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleProperty<Impl: IRotateTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AngleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRotateTransformStatics>, base.5, CenterXProperty::<Impl, OFFSET>, CenterYProperty::<Impl, OFFSET>, AngleProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScaleTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IScaleTransform";
}
#[cfg(feature = "implement_exclusive")]
impl IScaleTransformVtbl {
    pub const fn new<Impl: IScaleTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScaleTransformVtbl {
        unsafe extern "system" fn CenterX<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn ScaleX<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleX(value).into()
        }
        unsafe extern "system" fn ScaleY<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: IScaleTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleY(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScaleTransform>, base.5, CenterX::<Impl, OFFSET>, SetCenterX::<Impl, OFFSET>, CenterY::<Impl, OFFSET>, SetCenterY::<Impl, OFFSET>, ScaleX::<Impl, OFFSET>, SetScaleX::<Impl, OFFSET>, ScaleY::<Impl, OFFSET>, SetScaleY::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScaleTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IScaleTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScaleTransformStaticsVtbl {
    pub const fn new<Impl: IScaleTransformStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScaleTransformStaticsVtbl {
        unsafe extern "system" fn CenterXProperty<Impl: IScaleTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: IScaleTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleXProperty<Impl: IScaleTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleYProperty<Impl: IScaleTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScaleTransformStatics>, base.5, CenterXProperty::<Impl, OFFSET>, CenterYProperty::<Impl, OFFSET>, ScaleXProperty::<Impl, OFFSET>, ScaleYProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShadowImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IShadow";
}
#[cfg(feature = "implement_exclusive")]
impl IShadowVtbl {
    pub const fn new<Impl: IShadowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShadowVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShadow>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShadowFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShadowFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IShadowFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IShadowFactoryVtbl {
    pub const fn new<Impl: IShadowFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShadowFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShadowFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISkewTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn AngleX(&self) -> ::windows::core::Result<f64>;
    fn SetAngleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn AngleY(&self) -> ::windows::core::Result<f64>;
    fn SetAngleY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISkewTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISkewTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ISkewTransformVtbl {
    pub const fn new<Impl: ISkewTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISkewTransformVtbl {
        unsafe extern "system" fn CenterX<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn AngleX<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AngleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleX<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAngleX(value).into()
        }
        unsafe extern "system" fn AngleY<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AngleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleY<Impl: ISkewTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAngleY(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISkewTransform>, base.5, CenterX::<Impl, OFFSET>, SetCenterX::<Impl, OFFSET>, CenterY::<Impl, OFFSET>, SetCenterY::<Impl, OFFSET>, AngleX::<Impl, OFFSET>, SetAngleX::<Impl, OFFSET>, AngleY::<Impl, OFFSET>, SetAngleY::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISkewTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISkewTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISkewTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISkewTransformStaticsVtbl {
    pub const fn new<Impl: ISkewTransformStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISkewTransformStaticsVtbl {
        unsafe extern "system" fn CenterXProperty<Impl: ISkewTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: ISkewTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleXProperty<Impl: ISkewTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AngleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleYProperty<Impl: ISkewTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AngleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISkewTransformStatics>, base.5, CenterXProperty::<Impl, OFFSET>, CenterYProperty::<Impl, OFFSET>, AngleXProperty::<Impl, OFFSET>, AngleYProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISolidColorBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISolidColorBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ISolidColorBrushVtbl {
    pub const fn new<Impl: ISolidColorBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISolidColorBrushVtbl {
        unsafe extern "system" fn Color<Impl: ISolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ISolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISolidColorBrush>, base.5, Color::<Impl, OFFSET>, SetColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushFactoryImpl: Sized {
    fn CreateInstanceWithColor(&self, color: &super::super::Color) -> ::windows::core::Result<SolidColorBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISolidColorBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISolidColorBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISolidColorBrushFactoryVtbl {
    pub const fn new<Impl: ISolidColorBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISolidColorBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithColor<Impl: ISolidColorBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: super::super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithColor(&*(&color as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISolidColorBrushFactory>, base.5, CreateInstanceWithColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISolidColorBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ISolidColorBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISolidColorBrushStaticsVtbl {
    pub const fn new<Impl: ISolidColorBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISolidColorBrushStaticsVtbl {
        unsafe extern "system" fn ColorProperty<Impl: ISolidColorBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISolidColorBrushStatics>, base.5, ColorProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThemeShadowImpl: Sized {
    fn Receivers(&self) -> ::windows::core::Result<super::UIElementWeakCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThemeShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IThemeShadow";
}
#[cfg(feature = "implement_exclusive")]
impl IThemeShadowVtbl {
    pub const fn new<Impl: IThemeShadowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThemeShadowVtbl {
        unsafe extern "system" fn Receivers<Impl: IThemeShadowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Receivers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThemeShadow>, base.5, Receivers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThemeShadowFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThemeShadow>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThemeShadowFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IThemeShadowFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IThemeShadowFactoryVtbl {
    pub const fn new<Impl: IThemeShadowFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThemeShadowFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IThemeShadowFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThemeShadowFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushImpl: Sized {
    fn AlignmentX(&self) -> ::windows::core::Result<AlignmentX>;
    fn SetAlignmentX(&self, value: AlignmentX) -> ::windows::core::Result<()>;
    fn AlignmentY(&self) -> ::windows::core::Result<AlignmentY>;
    fn SetAlignmentY(&self, value: AlignmentY) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<Stretch>;
    fn SetStretch(&self, value: Stretch) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITileBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ITileBrushVtbl {
    pub const fn new<Impl: ITileBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileBrushVtbl {
        unsafe extern "system" fn AlignmentX<Impl: ITileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AlignmentX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlignmentX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignmentX<Impl: ITileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AlignmentX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlignmentX(value).into()
        }
        unsafe extern "system" fn AlignmentY<Impl: ITileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AlignmentY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlignmentY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignmentY<Impl: ITileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AlignmentY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlignmentY(value).into()
        }
        unsafe extern "system" fn Stretch<Impl: ITileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Stretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStretch<Impl: ITileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Stretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileBrush>, base.5, AlignmentX::<Impl, OFFSET>, SetAlignmentX::<Impl, OFFSET>, AlignmentY::<Impl, OFFSET>, SetAlignmentY::<Impl, OFFSET>, Stretch::<Impl, OFFSET>, SetStretch::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TileBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileBrushFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITileBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITileBrushFactoryVtbl {
    pub const fn new<Impl: ITileBrushFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileBrushFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITileBrushFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileBrushFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushStaticsImpl: Sized {
    fn AlignmentXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlignmentYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileBrushStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITileBrushStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITileBrushStaticsVtbl {
    pub const fn new<Impl: ITileBrushStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileBrushStaticsVtbl {
        unsafe extern "system" fn AlignmentXProperty<Impl: ITileBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlignmentXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlignmentYProperty<Impl: ITileBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlignmentYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StretchProperty<Impl: ITileBrushStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StretchProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileBrushStatics>, base.5, AlignmentXProperty::<Impl, OFFSET>, AlignmentYProperty::<Impl, OFFSET>, StretchProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineMarker {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITimelineMarker";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineMarkerVtbl {
    pub const fn new<Impl: ITimelineMarkerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimelineMarkerVtbl {
        unsafe extern "system" fn Time<Impl: ITimelineMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Time() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTime<Impl: ITimelineMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTime(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Type<Impl: ITimelineMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ITimelineMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Text<Impl: ITimelineMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITimelineMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimelineMarker>, base.5, Time::<Impl, OFFSET>, SetTime::<Impl, OFFSET>, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerRoutedEventArgsImpl: Sized {
    fn Marker(&self) -> ::windows::core::Result<TimelineMarker>;
    fn SetMarker(&self, value: &::core::option::Option<TimelineMarker>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineMarkerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITimelineMarkerRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineMarkerRoutedEventArgsVtbl {
    pub const fn new<Impl: ITimelineMarkerRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimelineMarkerRoutedEventArgsVtbl {
        unsafe extern "system" fn Marker<Impl: ITimelineMarkerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Marker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarker<Impl: ITimelineMarkerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMarker(&*(&value as *const <TimelineMarker as ::windows::core::Abi>::Abi as *const <TimelineMarker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimelineMarkerRoutedEventArgs>, base.5, Marker::<Impl, OFFSET>, SetMarker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerStaticsImpl: Sized {
    fn TimeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineMarkerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITimelineMarkerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineMarkerStaticsVtbl {
    pub const fn new<Impl: ITimelineMarkerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimelineMarkerStaticsVtbl {
        unsafe extern "system" fn TimeProperty<Impl: ITimelineMarkerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeProperty<Impl: ITimelineMarkerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextProperty<Impl: ITimelineMarkerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimelineMarkerStatics>, base.5, TimeProperty::<Impl, OFFSET>, TypeProperty::<Impl, OFFSET>, TextProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransform";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformVtbl {
    pub const fn new<Impl: ITransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransformVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransform>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransformFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformFactoryVtbl {
    pub const fn new<Impl: ITransformFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransformFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransformFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformGroupImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<TransformCollection>;
    fn SetChildren(&self, value: &::core::option::Option<TransformCollection>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<Matrix>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransformGroup";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformGroupVtbl {
    pub const fn new<Impl: ITransformGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransformGroupVtbl {
        unsafe extern "system" fn Children<Impl: ITransformGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildren<Impl: ITransformGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChildren(&*(&value as *const <TransformCollection as ::windows::core::Abi>::Abi as *const <TransformCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ITransformGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransformGroup>, base.5, Children::<Impl, OFFSET>, SetChildren::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformGroupStaticsImpl: Sized {
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransformGroupStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITransformGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITransformGroupStaticsVtbl {
    pub const fn new<Impl: ITransformGroupStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransformGroupStaticsVtbl {
        unsafe extern "system" fn ChildrenProperty<Impl: ITransformGroupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildrenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransformGroupStatics>, base.5, ChildrenProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITranslateTransformImpl: Sized {
    fn X(&self) -> ::windows::core::Result<f64>;
    fn SetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn Y(&self) -> ::windows::core::Result<f64>;
    fn SetY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITranslateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITranslateTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ITranslateTransformVtbl {
    pub const fn new<Impl: ITranslateTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITranslateTransformVtbl {
        unsafe extern "system" fn X<Impl: ITranslateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetX<Impl: ITranslateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetX(value).into()
        }
        unsafe extern "system" fn Y<Impl: ITranslateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetY<Impl: ITranslateTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetY(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITranslateTransform>, base.5, X::<Impl, OFFSET>, SetX::<Impl, OFFSET>, Y::<Impl, OFFSET>, SetY::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITranslateTransformStaticsImpl: Sized {
    fn XProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITranslateTransformStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ITranslateTransformStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITranslateTransformStaticsVtbl {
    pub const fn new<Impl: ITranslateTransformStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITranslateTransformStaticsVtbl {
        unsafe extern "system" fn XProperty<Impl: ITranslateTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YProperty<Impl: ITranslateTransformStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).YProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITranslateTransformStatics>, base.5, XProperty::<Impl, OFFSET>, YProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTreeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTreeHelperVtbl {
    pub const fn new<Impl: IVisualTreeHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualTreeHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualTreeHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperStaticsImpl: Sized {
    fn FindElementsInHostCoordinatesPoint(&self, intersectingpoint: &super::super::super::Foundation::Point, subtree: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindElementsInHostCoordinatesRect(&self, intersectingrect: &super::super::super::Foundation::Rect, subtree: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindAllElementsInHostCoordinatesPoint(&self, intersectingpoint: &super::super::super::Foundation::Point, subtree: &::core::option::Option<super::UIElement>, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindAllElementsInHostCoordinatesRect(&self, intersectingrect: &super::super::super::Foundation::Rect, subtree: &::core::option::Option<super::UIElement>, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn GetChild(&self, reference: &::core::option::Option<super::DependencyObject>, childindex: i32) -> ::windows::core::Result<super::DependencyObject>;
    fn GetChildrenCount(&self, reference: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn GetParent(&self, reference: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn DisconnectChildrenRecursive(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTreeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTreeHelperStaticsVtbl {
    pub const fn new<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualTreeHelperStaticsVtbl {
        unsafe extern "system" fn FindElementsInHostCoordinatesPoint<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intersectingpoint: super::super::super::Foundation::Point, subtree: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindElementsInHostCoordinatesPoint(&*(&intersectingpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindElementsInHostCoordinatesRect<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intersectingrect: super::super::super::Foundation::Rect, subtree: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindElementsInHostCoordinatesRect(&*(&intersectingrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllElementsInHostCoordinatesPoint<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intersectingpoint: super::super::super::Foundation::Point, subtree: ::windows::core::RawPtr, includeallelements: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllElementsInHostCoordinatesPoint(&*(&intersectingpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), includeallelements) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllElementsInHostCoordinatesRect<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intersectingrect: super::super::super::Foundation::Rect, subtree: ::windows::core::RawPtr, includeallelements: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllElementsInHostCoordinatesRect(&*(&intersectingrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), includeallelements) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr, childindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChild(&*(&reference as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), childindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenCount<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildrenCount(&*(&reference as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParent(&*(&reference as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectChildrenRecursive<Impl: IVisualTreeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DisconnectChildrenRecursive(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualTreeHelperStatics>, base.5, FindElementsInHostCoordinatesPoint::<Impl, OFFSET>, FindElementsInHostCoordinatesRect::<Impl, OFFSET>, FindAllElementsInHostCoordinatesPoint::<Impl, OFFSET>, FindAllElementsInHostCoordinatesRect::<Impl, OFFSET>, GetChild::<Impl, OFFSET>, GetChildrenCount::<Impl, OFFSET>, GetParent::<Impl, OFFSET>, DisconnectChildrenRecursive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperStatics2Impl: Sized {
    fn GetOpenPopups(&self, window: &::core::option::Option<super::Window>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTreeHelperStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelperStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTreeHelperStatics2Vtbl {
    pub const fn new<Impl: IVisualTreeHelperStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualTreeHelperStatics2Vtbl {
        unsafe extern "system" fn GetOpenPopups<Impl: IVisualTreeHelperStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpenPopups(&*(&window as *const <super::Window as ::windows::core::Abi>::Abi as *const <super::Window as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualTreeHelperStatics2>, base.5, GetOpenPopups::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperStatics3Impl: Sized {
    fn GetOpenPopupsForXamlRoot(&self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTreeHelperStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IVisualTreeHelperStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTreeHelperStatics3Vtbl {
    pub const fn new<Impl: IVisualTreeHelperStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualTreeHelperStatics3Vtbl {
        unsafe extern "system" fn GetOpenPopupsForXamlRoot<Impl: IVisualTreeHelperStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xamlroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpenPopupsForXamlRoot(&*(&xamlroot as *const <super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::XamlRoot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualTreeHelperStatics3>, base.5, GetOpenPopupsForXamlRoot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseImpl: Sized {
    fn FallbackColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetFallbackColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBase";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseVtbl {
    pub const fn new<Impl: IXamlCompositionBrushBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlCompositionBrushBaseVtbl {
        unsafe extern "system" fn FallbackColor<Impl: IXamlCompositionBrushBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FallbackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackColor<Impl: IXamlCompositionBrushBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFallbackColor(&*(&value as *const <super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlCompositionBrushBase>, base.5, FallbackColor::<Impl, OFFSET>, SetFallbackColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlCompositionBrushBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseFactoryVtbl {
    pub const fn new<Impl: IXamlCompositionBrushBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlCompositionBrushBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlCompositionBrushBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlCompositionBrushBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseOverridesImpl: Sized {
    fn OnConnected(&self) -> ::windows::core::Result<()>;
    fn OnDisconnected(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseOverridesVtbl {
    pub const fn new<Impl: IXamlCompositionBrushBaseOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlCompositionBrushBaseOverridesVtbl {
        unsafe extern "system" fn OnConnected<Impl: IXamlCompositionBrushBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnConnected().into()
        }
        unsafe extern "system" fn OnDisconnected<Impl: IXamlCompositionBrushBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnDisconnected().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlCompositionBrushBaseOverrides>, base.5, OnConnected::<Impl, OFFSET>, OnDisconnected::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseProtectedImpl: Sized {
    fn CompositionBrush(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
    fn SetCompositionBrush(&self, value: &::core::option::Option<super::super::Composition::CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseProtected";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseProtectedVtbl {
    pub const fn new<Impl: IXamlCompositionBrushBaseProtectedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlCompositionBrushBaseProtectedVtbl {
        unsafe extern "system" fn CompositionBrush<Impl: IXamlCompositionBrushBaseProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositionBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionBrush<Impl: IXamlCompositionBrushBaseProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCompositionBrush(&*(&value as *const <super::super::Composition::CompositionBrush as ::windows::core::Abi>::Abi as *const <super::super::Composition::CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlCompositionBrushBaseProtected>, base.5, CompositionBrush::<Impl, OFFSET>, SetCompositionBrush::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseStaticsImpl: Sized {
    fn FallbackColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlCompositionBrushBaseStaticsVtbl {
    pub const fn new<Impl: IXamlCompositionBrushBaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlCompositionBrushBaseStaticsVtbl {
        unsafe extern "system" fn FallbackColorProperty<Impl: IXamlCompositionBrushBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FallbackColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlCompositionBrushBaseStatics>, base.5, FallbackColorProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLight {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLight";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightVtbl {
    pub const fn new<Impl: IXamlLightImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlLightVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlLight>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlLight>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightFactoryVtbl {
    pub const fn new<Impl: IXamlLightFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlLightFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlLightFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlLightFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightOverridesImpl: Sized {
    fn GetId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnConnected(&self, newelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn OnDisconnected(&self, oldelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightOverridesVtbl {
    pub const fn new<Impl: IXamlLightOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlLightOverridesVtbl {
        unsafe extern "system" fn GetId<Impl: IXamlLightOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnected<Impl: IXamlLightOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnConnected(&*(&newelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDisconnected<Impl: IXamlLightOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnDisconnected(&*(&oldelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlLightOverrides>, base.5, GetId::<Impl, OFFSET>, OnConnected::<Impl, OFFSET>, OnDisconnected::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightProtectedImpl: Sized {
    fn CompositionLight(&self) -> ::windows::core::Result<super::super::Composition::CompositionLight>;
    fn SetCompositionLight(&self, value: &::core::option::Option<super::super::Composition::CompositionLight>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightProtected";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightProtectedVtbl {
    pub const fn new<Impl: IXamlLightProtectedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlLightProtectedVtbl {
        unsafe extern "system" fn CompositionLight<Impl: IXamlLightProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositionLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionLight<Impl: IXamlLightProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCompositionLight(&*(&value as *const <super::super::Composition::CompositionLight as ::windows::core::Abi>::Abi as *const <super::super::Composition::CompositionLight as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlLightProtected>, base.5, CompositionLight::<Impl, OFFSET>, SetCompositionLight::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightStaticsImpl: Sized {
    fn AddTargetElement(&self, lightid: &::windows::core::HSTRING, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn RemoveTargetElement(&self, lightid: &::windows::core::HSTRING, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AddTargetBrush(&self, lightid: &::windows::core::HSTRING, brush: &::core::option::Option<Brush>) -> ::windows::core::Result<()>;
    fn RemoveTargetBrush(&self, lightid: &::windows::core::HSTRING, brush: &::core::option::Option<Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlLightStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlLightStaticsVtbl {
    pub const fn new<Impl: IXamlLightStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlLightStaticsVtbl {
        unsafe extern "system" fn AddTargetElement<Impl: IXamlLightStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddTargetElement(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTargetElement<Impl: IXamlLightStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTargetElement(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddTargetBrush<Impl: IXamlLightStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddTargetBrush(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <Brush as ::windows::core::Abi>::Abi as *const <Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTargetBrush<Impl: IXamlLightStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTargetBrush(&*(&lightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <Brush as ::windows::core::Abi>::Abi as *const <Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlLightStatics>, base.5, AddTargetElement::<Impl, OFFSET>, RemoveTargetElement::<Impl, OFFSET>, AddTargetBrush::<Impl, OFFSET>, RemoveTargetBrush::<Impl, OFFSET>)
    }
}
