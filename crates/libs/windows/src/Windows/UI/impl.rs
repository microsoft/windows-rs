#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorHelper {
    const NAME: &'static str = "Windows.UI.IColorHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IColorHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorHelperVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorHelper as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorHelperStaticsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorHelperStatics, BASE_OFFSET>(), FromArgb: FromArgb::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorHelperStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorHelperStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorHelperStatics2Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorHelperStatics2, BASE_OFFSET>(), ToDisplayName: ToDisplayName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorHelperStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColors, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColors as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorsStatics, BASE_OFFSET>(),
            AliceBlue: AliceBlue::<Impl, IMPL_OFFSET>,
            AntiqueWhite: AntiqueWhite::<Impl, IMPL_OFFSET>,
            Aqua: Aqua::<Impl, IMPL_OFFSET>,
            Aquamarine: Aquamarine::<Impl, IMPL_OFFSET>,
            Azure: Azure::<Impl, IMPL_OFFSET>,
            Beige: Beige::<Impl, IMPL_OFFSET>,
            Bisque: Bisque::<Impl, IMPL_OFFSET>,
            Black: Black::<Impl, IMPL_OFFSET>,
            BlanchedAlmond: BlanchedAlmond::<Impl, IMPL_OFFSET>,
            Blue: Blue::<Impl, IMPL_OFFSET>,
            BlueViolet: BlueViolet::<Impl, IMPL_OFFSET>,
            Brown: Brown::<Impl, IMPL_OFFSET>,
            BurlyWood: BurlyWood::<Impl, IMPL_OFFSET>,
            CadetBlue: CadetBlue::<Impl, IMPL_OFFSET>,
            Chartreuse: Chartreuse::<Impl, IMPL_OFFSET>,
            Chocolate: Chocolate::<Impl, IMPL_OFFSET>,
            Coral: Coral::<Impl, IMPL_OFFSET>,
            CornflowerBlue: CornflowerBlue::<Impl, IMPL_OFFSET>,
            Cornsilk: Cornsilk::<Impl, IMPL_OFFSET>,
            Crimson: Crimson::<Impl, IMPL_OFFSET>,
            Cyan: Cyan::<Impl, IMPL_OFFSET>,
            DarkBlue: DarkBlue::<Impl, IMPL_OFFSET>,
            DarkCyan: DarkCyan::<Impl, IMPL_OFFSET>,
            DarkGoldenrod: DarkGoldenrod::<Impl, IMPL_OFFSET>,
            DarkGray: DarkGray::<Impl, IMPL_OFFSET>,
            DarkGreen: DarkGreen::<Impl, IMPL_OFFSET>,
            DarkKhaki: DarkKhaki::<Impl, IMPL_OFFSET>,
            DarkMagenta: DarkMagenta::<Impl, IMPL_OFFSET>,
            DarkOliveGreen: DarkOliveGreen::<Impl, IMPL_OFFSET>,
            DarkOrange: DarkOrange::<Impl, IMPL_OFFSET>,
            DarkOrchid: DarkOrchid::<Impl, IMPL_OFFSET>,
            DarkRed: DarkRed::<Impl, IMPL_OFFSET>,
            DarkSalmon: DarkSalmon::<Impl, IMPL_OFFSET>,
            DarkSeaGreen: DarkSeaGreen::<Impl, IMPL_OFFSET>,
            DarkSlateBlue: DarkSlateBlue::<Impl, IMPL_OFFSET>,
            DarkSlateGray: DarkSlateGray::<Impl, IMPL_OFFSET>,
            DarkTurquoise: DarkTurquoise::<Impl, IMPL_OFFSET>,
            DarkViolet: DarkViolet::<Impl, IMPL_OFFSET>,
            DeepPink: DeepPink::<Impl, IMPL_OFFSET>,
            DeepSkyBlue: DeepSkyBlue::<Impl, IMPL_OFFSET>,
            DimGray: DimGray::<Impl, IMPL_OFFSET>,
            DodgerBlue: DodgerBlue::<Impl, IMPL_OFFSET>,
            Firebrick: Firebrick::<Impl, IMPL_OFFSET>,
            FloralWhite: FloralWhite::<Impl, IMPL_OFFSET>,
            ForestGreen: ForestGreen::<Impl, IMPL_OFFSET>,
            Fuchsia: Fuchsia::<Impl, IMPL_OFFSET>,
            Gainsboro: Gainsboro::<Impl, IMPL_OFFSET>,
            GhostWhite: GhostWhite::<Impl, IMPL_OFFSET>,
            Gold: Gold::<Impl, IMPL_OFFSET>,
            Goldenrod: Goldenrod::<Impl, IMPL_OFFSET>,
            Gray: Gray::<Impl, IMPL_OFFSET>,
            Green: Green::<Impl, IMPL_OFFSET>,
            GreenYellow: GreenYellow::<Impl, IMPL_OFFSET>,
            Honeydew: Honeydew::<Impl, IMPL_OFFSET>,
            HotPink: HotPink::<Impl, IMPL_OFFSET>,
            IndianRed: IndianRed::<Impl, IMPL_OFFSET>,
            Indigo: Indigo::<Impl, IMPL_OFFSET>,
            Ivory: Ivory::<Impl, IMPL_OFFSET>,
            Khaki: Khaki::<Impl, IMPL_OFFSET>,
            Lavender: Lavender::<Impl, IMPL_OFFSET>,
            LavenderBlush: LavenderBlush::<Impl, IMPL_OFFSET>,
            LawnGreen: LawnGreen::<Impl, IMPL_OFFSET>,
            LemonChiffon: LemonChiffon::<Impl, IMPL_OFFSET>,
            LightBlue: LightBlue::<Impl, IMPL_OFFSET>,
            LightCoral: LightCoral::<Impl, IMPL_OFFSET>,
            LightCyan: LightCyan::<Impl, IMPL_OFFSET>,
            LightGoldenrodYellow: LightGoldenrodYellow::<Impl, IMPL_OFFSET>,
            LightGreen: LightGreen::<Impl, IMPL_OFFSET>,
            LightGray: LightGray::<Impl, IMPL_OFFSET>,
            LightPink: LightPink::<Impl, IMPL_OFFSET>,
            LightSalmon: LightSalmon::<Impl, IMPL_OFFSET>,
            LightSeaGreen: LightSeaGreen::<Impl, IMPL_OFFSET>,
            LightSkyBlue: LightSkyBlue::<Impl, IMPL_OFFSET>,
            LightSlateGray: LightSlateGray::<Impl, IMPL_OFFSET>,
            LightSteelBlue: LightSteelBlue::<Impl, IMPL_OFFSET>,
            LightYellow: LightYellow::<Impl, IMPL_OFFSET>,
            Lime: Lime::<Impl, IMPL_OFFSET>,
            LimeGreen: LimeGreen::<Impl, IMPL_OFFSET>,
            Linen: Linen::<Impl, IMPL_OFFSET>,
            Magenta: Magenta::<Impl, IMPL_OFFSET>,
            Maroon: Maroon::<Impl, IMPL_OFFSET>,
            MediumAquamarine: MediumAquamarine::<Impl, IMPL_OFFSET>,
            MediumBlue: MediumBlue::<Impl, IMPL_OFFSET>,
            MediumOrchid: MediumOrchid::<Impl, IMPL_OFFSET>,
            MediumPurple: MediumPurple::<Impl, IMPL_OFFSET>,
            MediumSeaGreen: MediumSeaGreen::<Impl, IMPL_OFFSET>,
            MediumSlateBlue: MediumSlateBlue::<Impl, IMPL_OFFSET>,
            MediumSpringGreen: MediumSpringGreen::<Impl, IMPL_OFFSET>,
            MediumTurquoise: MediumTurquoise::<Impl, IMPL_OFFSET>,
            MediumVioletRed: MediumVioletRed::<Impl, IMPL_OFFSET>,
            MidnightBlue: MidnightBlue::<Impl, IMPL_OFFSET>,
            MintCream: MintCream::<Impl, IMPL_OFFSET>,
            MistyRose: MistyRose::<Impl, IMPL_OFFSET>,
            Moccasin: Moccasin::<Impl, IMPL_OFFSET>,
            NavajoWhite: NavajoWhite::<Impl, IMPL_OFFSET>,
            Navy: Navy::<Impl, IMPL_OFFSET>,
            OldLace: OldLace::<Impl, IMPL_OFFSET>,
            Olive: Olive::<Impl, IMPL_OFFSET>,
            OliveDrab: OliveDrab::<Impl, IMPL_OFFSET>,
            Orange: Orange::<Impl, IMPL_OFFSET>,
            OrangeRed: OrangeRed::<Impl, IMPL_OFFSET>,
            Orchid: Orchid::<Impl, IMPL_OFFSET>,
            PaleGoldenrod: PaleGoldenrod::<Impl, IMPL_OFFSET>,
            PaleGreen: PaleGreen::<Impl, IMPL_OFFSET>,
            PaleTurquoise: PaleTurquoise::<Impl, IMPL_OFFSET>,
            PaleVioletRed: PaleVioletRed::<Impl, IMPL_OFFSET>,
            PapayaWhip: PapayaWhip::<Impl, IMPL_OFFSET>,
            PeachPuff: PeachPuff::<Impl, IMPL_OFFSET>,
            Peru: Peru::<Impl, IMPL_OFFSET>,
            Pink: Pink::<Impl, IMPL_OFFSET>,
            Plum: Plum::<Impl, IMPL_OFFSET>,
            PowderBlue: PowderBlue::<Impl, IMPL_OFFSET>,
            Purple: Purple::<Impl, IMPL_OFFSET>,
            Red: Red::<Impl, IMPL_OFFSET>,
            RosyBrown: RosyBrown::<Impl, IMPL_OFFSET>,
            RoyalBlue: RoyalBlue::<Impl, IMPL_OFFSET>,
            SaddleBrown: SaddleBrown::<Impl, IMPL_OFFSET>,
            Salmon: Salmon::<Impl, IMPL_OFFSET>,
            SandyBrown: SandyBrown::<Impl, IMPL_OFFSET>,
            SeaGreen: SeaGreen::<Impl, IMPL_OFFSET>,
            SeaShell: SeaShell::<Impl, IMPL_OFFSET>,
            Sienna: Sienna::<Impl, IMPL_OFFSET>,
            Silver: Silver::<Impl, IMPL_OFFSET>,
            SkyBlue: SkyBlue::<Impl, IMPL_OFFSET>,
            SlateBlue: SlateBlue::<Impl, IMPL_OFFSET>,
            SlateGray: SlateGray::<Impl, IMPL_OFFSET>,
            Snow: Snow::<Impl, IMPL_OFFSET>,
            SpringGreen: SpringGreen::<Impl, IMPL_OFFSET>,
            SteelBlue: SteelBlue::<Impl, IMPL_OFFSET>,
            Tan: Tan::<Impl, IMPL_OFFSET>,
            Teal: Teal::<Impl, IMPL_OFFSET>,
            Thistle: Thistle::<Impl, IMPL_OFFSET>,
            Tomato: Tomato::<Impl, IMPL_OFFSET>,
            Transparent: Transparent::<Impl, IMPL_OFFSET>,
            Turquoise: Turquoise::<Impl, IMPL_OFFSET>,
            Violet: Violet::<Impl, IMPL_OFFSET>,
            Wheat: Wheat::<Impl, IMPL_OFFSET>,
            White: White::<Impl, IMPL_OFFSET>,
            WhiteSmoke: WhiteSmoke::<Impl, IMPL_OFFSET>,
            Yellow: Yellow::<Impl, IMPL_OFFSET>,
            YellowGreen: YellowGreen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorsStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContentRootImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIContentRootVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUIContentRoot, BASE_OFFSET>(), UIContext: UIContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIContentRoot as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIContextVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUIContext, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIContext as ::windows::core::Interface>::IID
    }
}
