#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IClosedCaptionPropertiesStaticsImpl: Sized {
    fn FontColor(&mut self) -> ::windows::core::Result<ClosedCaptionColor>;
    fn ComputedFontColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn FontOpacity(&mut self) -> ::windows::core::Result<ClosedCaptionOpacity>;
    fn FontSize(&mut self) -> ::windows::core::Result<ClosedCaptionSize>;
    fn FontStyle(&mut self) -> ::windows::core::Result<ClosedCaptionStyle>;
    fn FontEffect(&mut self) -> ::windows::core::Result<ClosedCaptionEdgeEffect>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<ClosedCaptionColor>;
    fn ComputedBackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn BackgroundOpacity(&mut self) -> ::windows::core::Result<ClosedCaptionOpacity>;
    fn RegionColor(&mut self) -> ::windows::core::Result<ClosedCaptionColor>;
    fn ComputedRegionColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn RegionOpacity(&mut self) -> ::windows::core::Result<ClosedCaptionOpacity>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClosedCaptionPropertiesStatics {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.IClosedCaptionPropertiesStatics";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IClosedCaptionPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClosedCaptionPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClosedCaptionPropertiesStaticsVtbl {
        unsafe extern "system" fn FontColor<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputedFontColor<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputedFontColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontOpacity<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontSize<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStyle<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontEffect<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionEdgeEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundColor<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ComputedBackgroundColor<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputedBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundOpacity<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegionColor<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegionColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputedRegionColor<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputedRegionColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegionOpacity<Impl: IClosedCaptionPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegionOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClosedCaptionPropertiesStatics, BASE_OFFSET>(),
            FontColor: FontColor::<Impl, IMPL_OFFSET>,
            ComputedFontColor: ComputedFontColor::<Impl, IMPL_OFFSET>,
            FontOpacity: FontOpacity::<Impl, IMPL_OFFSET>,
            FontSize: FontSize::<Impl, IMPL_OFFSET>,
            FontStyle: FontStyle::<Impl, IMPL_OFFSET>,
            FontEffect: FontEffect::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            ComputedBackgroundColor: ComputedBackgroundColor::<Impl, IMPL_OFFSET>,
            BackgroundOpacity: BackgroundOpacity::<Impl, IMPL_OFFSET>,
            RegionColor: RegionColor::<Impl, IMPL_OFFSET>,
            ComputedRegionColor: ComputedRegionColor::<Impl, IMPL_OFFSET>,
            RegionOpacity: RegionOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClosedCaptionPropertiesStatics as ::windows::core::Interface>::IID
    }
}
