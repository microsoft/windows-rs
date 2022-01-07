#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorHelper {
    const NAME: &'static str = "Windows.UI.IColorHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IColorHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorHelperImpl, const OFFSET: isize>() -> IColorHelperVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorHelper>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperStaticsImpl: Sized {
    fn FromArgb(&self, a: u8, r: u8, g: u8, b: u8) -> ::windows::core::Result<Color>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorHelperStatics {
    const NAME: &'static str = "Windows.UI.IColorHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorHelperStaticsImpl, const OFFSET: isize>() -> IColorHelperStaticsVtbl {
        unsafe extern "system" fn FromArgb<Impl: IColorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromArgb(a, r, g, b) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorHelperStatics>, ::windows::core::GetTrustLevel, FromArgb::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperStatics2Impl: Sized {
    fn ToDisplayName(&self, color: &Color) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorHelperStatics2 {
    const NAME: &'static str = "Windows.UI.IColorHelperStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IColorHelperStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorHelperStatics2Impl, const OFFSET: isize>() -> IColorHelperStatics2Vtbl {
        unsafe extern "system" fn ToDisplayName<Impl: IColorHelperStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: Color, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToDisplayName(&*(&color as *const <Color as ::windows::core::Abi>::Abi as *const <Color as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorHelperStatics2>, ::windows::core::GetTrustLevel, ToDisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColors {
    const NAME: &'static str = "Windows.UI.IColors";
}
#[cfg(feature = "implement_exclusive")]
impl IColorsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorsImpl, const OFFSET: isize>() -> IColorsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColors>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorsStaticsImpl: Sized {
    fn AliceBlue(&self) -> ::windows::core::Result<Color>;
    fn AntiqueWhite(&self) -> ::windows::core::Result<Color>;
    fn Aqua(&self) -> ::windows::core::Result<Color>;
    fn Aquamarine(&self) -> ::windows::core::Result<Color>;
    fn Azure(&self) -> ::windows::core::Result<Color>;
    fn Beige(&self) -> ::windows::core::Result<Color>;
    fn Bisque(&self) -> ::windows::core::Result<Color>;
    fn Black(&self) -> ::windows::core::Result<Color>;
    fn BlanchedAlmond(&self) -> ::windows::core::Result<Color>;
    fn Blue(&self) -> ::windows::core::Result<Color>;
    fn BlueViolet(&self) -> ::windows::core::Result<Color>;
    fn Brown(&self) -> ::windows::core::Result<Color>;
    fn BurlyWood(&self) -> ::windows::core::Result<Color>;
    fn CadetBlue(&self) -> ::windows::core::Result<Color>;
    fn Chartreuse(&self) -> ::windows::core::Result<Color>;
    fn Chocolate(&self) -> ::windows::core::Result<Color>;
    fn Coral(&self) -> ::windows::core::Result<Color>;
    fn CornflowerBlue(&self) -> ::windows::core::Result<Color>;
    fn Cornsilk(&self) -> ::windows::core::Result<Color>;
    fn Crimson(&self) -> ::windows::core::Result<Color>;
    fn Cyan(&self) -> ::windows::core::Result<Color>;
    fn DarkBlue(&self) -> ::windows::core::Result<Color>;
    fn DarkCyan(&self) -> ::windows::core::Result<Color>;
    fn DarkGoldenrod(&self) -> ::windows::core::Result<Color>;
    fn DarkGray(&self) -> ::windows::core::Result<Color>;
    fn DarkGreen(&self) -> ::windows::core::Result<Color>;
    fn DarkKhaki(&self) -> ::windows::core::Result<Color>;
    fn DarkMagenta(&self) -> ::windows::core::Result<Color>;
    fn DarkOliveGreen(&self) -> ::windows::core::Result<Color>;
    fn DarkOrange(&self) -> ::windows::core::Result<Color>;
    fn DarkOrchid(&self) -> ::windows::core::Result<Color>;
    fn DarkRed(&self) -> ::windows::core::Result<Color>;
    fn DarkSalmon(&self) -> ::windows::core::Result<Color>;
    fn DarkSeaGreen(&self) -> ::windows::core::Result<Color>;
    fn DarkSlateBlue(&self) -> ::windows::core::Result<Color>;
    fn DarkSlateGray(&self) -> ::windows::core::Result<Color>;
    fn DarkTurquoise(&self) -> ::windows::core::Result<Color>;
    fn DarkViolet(&self) -> ::windows::core::Result<Color>;
    fn DeepPink(&self) -> ::windows::core::Result<Color>;
    fn DeepSkyBlue(&self) -> ::windows::core::Result<Color>;
    fn DimGray(&self) -> ::windows::core::Result<Color>;
    fn DodgerBlue(&self) -> ::windows::core::Result<Color>;
    fn Firebrick(&self) -> ::windows::core::Result<Color>;
    fn FloralWhite(&self) -> ::windows::core::Result<Color>;
    fn ForestGreen(&self) -> ::windows::core::Result<Color>;
    fn Fuchsia(&self) -> ::windows::core::Result<Color>;
    fn Gainsboro(&self) -> ::windows::core::Result<Color>;
    fn GhostWhite(&self) -> ::windows::core::Result<Color>;
    fn Gold(&self) -> ::windows::core::Result<Color>;
    fn Goldenrod(&self) -> ::windows::core::Result<Color>;
    fn Gray(&self) -> ::windows::core::Result<Color>;
    fn Green(&self) -> ::windows::core::Result<Color>;
    fn GreenYellow(&self) -> ::windows::core::Result<Color>;
    fn Honeydew(&self) -> ::windows::core::Result<Color>;
    fn HotPink(&self) -> ::windows::core::Result<Color>;
    fn IndianRed(&self) -> ::windows::core::Result<Color>;
    fn Indigo(&self) -> ::windows::core::Result<Color>;
    fn Ivory(&self) -> ::windows::core::Result<Color>;
    fn Khaki(&self) -> ::windows::core::Result<Color>;
    fn Lavender(&self) -> ::windows::core::Result<Color>;
    fn LavenderBlush(&self) -> ::windows::core::Result<Color>;
    fn LawnGreen(&self) -> ::windows::core::Result<Color>;
    fn LemonChiffon(&self) -> ::windows::core::Result<Color>;
    fn LightBlue(&self) -> ::windows::core::Result<Color>;
    fn LightCoral(&self) -> ::windows::core::Result<Color>;
    fn LightCyan(&self) -> ::windows::core::Result<Color>;
    fn LightGoldenrodYellow(&self) -> ::windows::core::Result<Color>;
    fn LightGreen(&self) -> ::windows::core::Result<Color>;
    fn LightGray(&self) -> ::windows::core::Result<Color>;
    fn LightPink(&self) -> ::windows::core::Result<Color>;
    fn LightSalmon(&self) -> ::windows::core::Result<Color>;
    fn LightSeaGreen(&self) -> ::windows::core::Result<Color>;
    fn LightSkyBlue(&self) -> ::windows::core::Result<Color>;
    fn LightSlateGray(&self) -> ::windows::core::Result<Color>;
    fn LightSteelBlue(&self) -> ::windows::core::Result<Color>;
    fn LightYellow(&self) -> ::windows::core::Result<Color>;
    fn Lime(&self) -> ::windows::core::Result<Color>;
    fn LimeGreen(&self) -> ::windows::core::Result<Color>;
    fn Linen(&self) -> ::windows::core::Result<Color>;
    fn Magenta(&self) -> ::windows::core::Result<Color>;
    fn Maroon(&self) -> ::windows::core::Result<Color>;
    fn MediumAquamarine(&self) -> ::windows::core::Result<Color>;
    fn MediumBlue(&self) -> ::windows::core::Result<Color>;
    fn MediumOrchid(&self) -> ::windows::core::Result<Color>;
    fn MediumPurple(&self) -> ::windows::core::Result<Color>;
    fn MediumSeaGreen(&self) -> ::windows::core::Result<Color>;
    fn MediumSlateBlue(&self) -> ::windows::core::Result<Color>;
    fn MediumSpringGreen(&self) -> ::windows::core::Result<Color>;
    fn MediumTurquoise(&self) -> ::windows::core::Result<Color>;
    fn MediumVioletRed(&self) -> ::windows::core::Result<Color>;
    fn MidnightBlue(&self) -> ::windows::core::Result<Color>;
    fn MintCream(&self) -> ::windows::core::Result<Color>;
    fn MistyRose(&self) -> ::windows::core::Result<Color>;
    fn Moccasin(&self) -> ::windows::core::Result<Color>;
    fn NavajoWhite(&self) -> ::windows::core::Result<Color>;
    fn Navy(&self) -> ::windows::core::Result<Color>;
    fn OldLace(&self) -> ::windows::core::Result<Color>;
    fn Olive(&self) -> ::windows::core::Result<Color>;
    fn OliveDrab(&self) -> ::windows::core::Result<Color>;
    fn Orange(&self) -> ::windows::core::Result<Color>;
    fn OrangeRed(&self) -> ::windows::core::Result<Color>;
    fn Orchid(&self) -> ::windows::core::Result<Color>;
    fn PaleGoldenrod(&self) -> ::windows::core::Result<Color>;
    fn PaleGreen(&self) -> ::windows::core::Result<Color>;
    fn PaleTurquoise(&self) -> ::windows::core::Result<Color>;
    fn PaleVioletRed(&self) -> ::windows::core::Result<Color>;
    fn PapayaWhip(&self) -> ::windows::core::Result<Color>;
    fn PeachPuff(&self) -> ::windows::core::Result<Color>;
    fn Peru(&self) -> ::windows::core::Result<Color>;
    fn Pink(&self) -> ::windows::core::Result<Color>;
    fn Plum(&self) -> ::windows::core::Result<Color>;
    fn PowderBlue(&self) -> ::windows::core::Result<Color>;
    fn Purple(&self) -> ::windows::core::Result<Color>;
    fn Red(&self) -> ::windows::core::Result<Color>;
    fn RosyBrown(&self) -> ::windows::core::Result<Color>;
    fn RoyalBlue(&self) -> ::windows::core::Result<Color>;
    fn SaddleBrown(&self) -> ::windows::core::Result<Color>;
    fn Salmon(&self) -> ::windows::core::Result<Color>;
    fn SandyBrown(&self) -> ::windows::core::Result<Color>;
    fn SeaGreen(&self) -> ::windows::core::Result<Color>;
    fn SeaShell(&self) -> ::windows::core::Result<Color>;
    fn Sienna(&self) -> ::windows::core::Result<Color>;
    fn Silver(&self) -> ::windows::core::Result<Color>;
    fn SkyBlue(&self) -> ::windows::core::Result<Color>;
    fn SlateBlue(&self) -> ::windows::core::Result<Color>;
    fn SlateGray(&self) -> ::windows::core::Result<Color>;
    fn Snow(&self) -> ::windows::core::Result<Color>;
    fn SpringGreen(&self) -> ::windows::core::Result<Color>;
    fn SteelBlue(&self) -> ::windows::core::Result<Color>;
    fn Tan(&self) -> ::windows::core::Result<Color>;
    fn Teal(&self) -> ::windows::core::Result<Color>;
    fn Thistle(&self) -> ::windows::core::Result<Color>;
    fn Tomato(&self) -> ::windows::core::Result<Color>;
    fn Transparent(&self) -> ::windows::core::Result<Color>;
    fn Turquoise(&self) -> ::windows::core::Result<Color>;
    fn Violet(&self) -> ::windows::core::Result<Color>;
    fn Wheat(&self) -> ::windows::core::Result<Color>;
    fn White(&self) -> ::windows::core::Result<Color>;
    fn WhiteSmoke(&self) -> ::windows::core::Result<Color>;
    fn Yellow(&self) -> ::windows::core::Result<Color>;
    fn YellowGreen(&self) -> ::windows::core::Result<Color>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorsStatics {
    const NAME: &'static str = "Windows.UI.IColorsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorsStaticsImpl, const OFFSET: isize>() -> IColorsStaticsVtbl {
        unsafe extern "system" fn AliceBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AliceBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntiqueWhite<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntiqueWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aqua<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aqua() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aquamarine<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aquamarine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Azure<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Azure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Beige<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Beige() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bisque<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bisque() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Black<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Black() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlanchedAlmond<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlanchedAlmond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Blue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlueViolet<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlueViolet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Brown<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Brown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BurlyWood<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BurlyWood() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CadetBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CadetBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Chartreuse<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Chartreuse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Chocolate<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Chocolate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Coral<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Coral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CornflowerBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CornflowerBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cornsilk<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cornsilk() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Crimson<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Crimson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cyan<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cyan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkCyan<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkCyan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkGoldenrod<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkGoldenrod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkGray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkKhaki<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkKhaki() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkMagenta<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkMagenta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkOliveGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkOliveGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkOrange<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkOrange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkOrchid<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkOrchid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkRed<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkRed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSalmon<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkSalmon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSeaGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkSeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSlateBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkSlateBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSlateGray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkSlateGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkTurquoise<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkTurquoise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkViolet<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DarkViolet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeepPink<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeepPink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeepSkyBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeepSkyBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DimGray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DimGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DodgerBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DodgerBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Firebrick<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Firebrick() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FloralWhite<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FloralWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForestGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForestGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Fuchsia<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fuchsia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gainsboro<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gainsboro() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GhostWhite<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GhostWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gold<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Goldenrod<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Goldenrod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Green<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Green() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GreenYellow<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GreenYellow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Honeydew<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Honeydew() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HotPink<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HotPink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndianRed<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndianRed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Indigo<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Indigo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ivory<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ivory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Khaki<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Khaki() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lavender<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lavender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LavenderBlush<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LavenderBlush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LawnGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LawnGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LemonChiffon<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LemonChiffon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightCoral<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightCoral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightCyan<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightCyan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightGoldenrodYellow<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightGoldenrodYellow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightGray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightPink<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightPink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSalmon<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightSalmon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSeaGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightSeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSkyBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightSkyBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSlateGray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightSlateGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSteelBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightSteelBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightYellow<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightYellow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lime<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LimeGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LimeGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Linen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Linen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Magenta<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Magenta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maroon<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maroon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumAquamarine<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumAquamarine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumOrchid<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumOrchid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumPurple<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumPurple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumSeaGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumSeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumSlateBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumSlateBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumSpringGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumSpringGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumTurquoise<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumTurquoise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumVioletRed<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediumVioletRed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MidnightBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MidnightBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MintCream<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MintCream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MistyRose<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MistyRose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Moccasin<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Moccasin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavajoWhite<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavajoWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Navy<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldLace<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldLace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Olive<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Olive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OliveDrab<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OliveDrab() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orange<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrangeRed<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrangeRed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orchid<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orchid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleGoldenrod<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaleGoldenrod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaleGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleTurquoise<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaleTurquoise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleVioletRed<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaleVioletRed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PapayaWhip<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PapayaWhip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeachPuff<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeachPuff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peru<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peru() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pink<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plum<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Plum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowderBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowderBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purple<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Purple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Red<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Red() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RosyBrown<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RosyBrown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoyalBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoyalBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaddleBrown<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaddleBrown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Salmon<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Salmon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SandyBrown<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SandyBrown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeaGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeaShell<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeaShell() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sienna<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sienna() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silver<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Silver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkyBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkyBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SlateBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlateBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SlateGray<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlateGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Snow<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Snow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpringGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpringGreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SteelBlue<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SteelBlue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tan<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Teal<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Teal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thistle<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thistle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tomato<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tomato() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transparent<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transparent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Turquoise<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Turquoise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Violet<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Violet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wheat<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wheat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn White<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).White() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteSmoke<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhiteSmoke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Yellow<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Yellow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YellowGreen<Impl: IColorsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YellowGreen() {
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
            ::windows::core::GetRuntimeClassName::<IColorsStatics>,
            ::windows::core::GetTrustLevel,
            AliceBlue::<Impl, OFFSET>,
            AntiqueWhite::<Impl, OFFSET>,
            Aqua::<Impl, OFFSET>,
            Aquamarine::<Impl, OFFSET>,
            Azure::<Impl, OFFSET>,
            Beige::<Impl, OFFSET>,
            Bisque::<Impl, OFFSET>,
            Black::<Impl, OFFSET>,
            BlanchedAlmond::<Impl, OFFSET>,
            Blue::<Impl, OFFSET>,
            BlueViolet::<Impl, OFFSET>,
            Brown::<Impl, OFFSET>,
            BurlyWood::<Impl, OFFSET>,
            CadetBlue::<Impl, OFFSET>,
            Chartreuse::<Impl, OFFSET>,
            Chocolate::<Impl, OFFSET>,
            Coral::<Impl, OFFSET>,
            CornflowerBlue::<Impl, OFFSET>,
            Cornsilk::<Impl, OFFSET>,
            Crimson::<Impl, OFFSET>,
            Cyan::<Impl, OFFSET>,
            DarkBlue::<Impl, OFFSET>,
            DarkCyan::<Impl, OFFSET>,
            DarkGoldenrod::<Impl, OFFSET>,
            DarkGray::<Impl, OFFSET>,
            DarkGreen::<Impl, OFFSET>,
            DarkKhaki::<Impl, OFFSET>,
            DarkMagenta::<Impl, OFFSET>,
            DarkOliveGreen::<Impl, OFFSET>,
            DarkOrange::<Impl, OFFSET>,
            DarkOrchid::<Impl, OFFSET>,
            DarkRed::<Impl, OFFSET>,
            DarkSalmon::<Impl, OFFSET>,
            DarkSeaGreen::<Impl, OFFSET>,
            DarkSlateBlue::<Impl, OFFSET>,
            DarkSlateGray::<Impl, OFFSET>,
            DarkTurquoise::<Impl, OFFSET>,
            DarkViolet::<Impl, OFFSET>,
            DeepPink::<Impl, OFFSET>,
            DeepSkyBlue::<Impl, OFFSET>,
            DimGray::<Impl, OFFSET>,
            DodgerBlue::<Impl, OFFSET>,
            Firebrick::<Impl, OFFSET>,
            FloralWhite::<Impl, OFFSET>,
            ForestGreen::<Impl, OFFSET>,
            Fuchsia::<Impl, OFFSET>,
            Gainsboro::<Impl, OFFSET>,
            GhostWhite::<Impl, OFFSET>,
            Gold::<Impl, OFFSET>,
            Goldenrod::<Impl, OFFSET>,
            Gray::<Impl, OFFSET>,
            Green::<Impl, OFFSET>,
            GreenYellow::<Impl, OFFSET>,
            Honeydew::<Impl, OFFSET>,
            HotPink::<Impl, OFFSET>,
            IndianRed::<Impl, OFFSET>,
            Indigo::<Impl, OFFSET>,
            Ivory::<Impl, OFFSET>,
            Khaki::<Impl, OFFSET>,
            Lavender::<Impl, OFFSET>,
            LavenderBlush::<Impl, OFFSET>,
            LawnGreen::<Impl, OFFSET>,
            LemonChiffon::<Impl, OFFSET>,
            LightBlue::<Impl, OFFSET>,
            LightCoral::<Impl, OFFSET>,
            LightCyan::<Impl, OFFSET>,
            LightGoldenrodYellow::<Impl, OFFSET>,
            LightGreen::<Impl, OFFSET>,
            LightGray::<Impl, OFFSET>,
            LightPink::<Impl, OFFSET>,
            LightSalmon::<Impl, OFFSET>,
            LightSeaGreen::<Impl, OFFSET>,
            LightSkyBlue::<Impl, OFFSET>,
            LightSlateGray::<Impl, OFFSET>,
            LightSteelBlue::<Impl, OFFSET>,
            LightYellow::<Impl, OFFSET>,
            Lime::<Impl, OFFSET>,
            LimeGreen::<Impl, OFFSET>,
            Linen::<Impl, OFFSET>,
            Magenta::<Impl, OFFSET>,
            Maroon::<Impl, OFFSET>,
            MediumAquamarine::<Impl, OFFSET>,
            MediumBlue::<Impl, OFFSET>,
            MediumOrchid::<Impl, OFFSET>,
            MediumPurple::<Impl, OFFSET>,
            MediumSeaGreen::<Impl, OFFSET>,
            MediumSlateBlue::<Impl, OFFSET>,
            MediumSpringGreen::<Impl, OFFSET>,
            MediumTurquoise::<Impl, OFFSET>,
            MediumVioletRed::<Impl, OFFSET>,
            MidnightBlue::<Impl, OFFSET>,
            MintCream::<Impl, OFFSET>,
            MistyRose::<Impl, OFFSET>,
            Moccasin::<Impl, OFFSET>,
            NavajoWhite::<Impl, OFFSET>,
            Navy::<Impl, OFFSET>,
            OldLace::<Impl, OFFSET>,
            Olive::<Impl, OFFSET>,
            OliveDrab::<Impl, OFFSET>,
            Orange::<Impl, OFFSET>,
            OrangeRed::<Impl, OFFSET>,
            Orchid::<Impl, OFFSET>,
            PaleGoldenrod::<Impl, OFFSET>,
            PaleGreen::<Impl, OFFSET>,
            PaleTurquoise::<Impl, OFFSET>,
            PaleVioletRed::<Impl, OFFSET>,
            PapayaWhip::<Impl, OFFSET>,
            PeachPuff::<Impl, OFFSET>,
            Peru::<Impl, OFFSET>,
            Pink::<Impl, OFFSET>,
            Plum::<Impl, OFFSET>,
            PowderBlue::<Impl, OFFSET>,
            Purple::<Impl, OFFSET>,
            Red::<Impl, OFFSET>,
            RosyBrown::<Impl, OFFSET>,
            RoyalBlue::<Impl, OFFSET>,
            SaddleBrown::<Impl, OFFSET>,
            Salmon::<Impl, OFFSET>,
            SandyBrown::<Impl, OFFSET>,
            SeaGreen::<Impl, OFFSET>,
            SeaShell::<Impl, OFFSET>,
            Sienna::<Impl, OFFSET>,
            Silver::<Impl, OFFSET>,
            SkyBlue::<Impl, OFFSET>,
            SlateBlue::<Impl, OFFSET>,
            SlateGray::<Impl, OFFSET>,
            Snow::<Impl, OFFSET>,
            SpringGreen::<Impl, OFFSET>,
            SteelBlue::<Impl, OFFSET>,
            Tan::<Impl, OFFSET>,
            Teal::<Impl, OFFSET>,
            Thistle::<Impl, OFFSET>,
            Tomato::<Impl, OFFSET>,
            Transparent::<Impl, OFFSET>,
            Turquoise::<Impl, OFFSET>,
            Violet::<Impl, OFFSET>,
            Wheat::<Impl, OFFSET>,
            White::<Impl, OFFSET>,
            WhiteSmoke::<Impl, OFFSET>,
            Yellow::<Impl, OFFSET>,
            YellowGreen::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIContentRootImpl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<UIContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIContentRoot {
    const NAME: &'static str = "Windows.UI.IUIContentRoot";
}
#[cfg(feature = "implement_exclusive")]
impl IUIContentRootVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContentRootImpl, const OFFSET: isize>() -> IUIContentRootVtbl {
        unsafe extern "system" fn UIContext<Impl: IUIContentRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIContentRoot>, ::windows::core::GetTrustLevel, UIContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIContextImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIContext {
    const NAME: &'static str = "Windows.UI.IUIContext";
}
#[cfg(feature = "implement_exclusive")]
impl IUIContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextImpl, const OFFSET: isize>() -> IUIContextVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIContext>, ::windows::core::GetTrustLevel)
    }
}
