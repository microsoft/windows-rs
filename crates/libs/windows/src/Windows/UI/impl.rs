#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperStaticsImpl: Sized {
    fn FromArgb(&self, a: u8, r: u8, g: u8, b: u8) -> ::windows::core::Result<Color>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorHelperStatics2Impl: Sized {
    fn ToDisplayName(&self, color: &Color) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorsImpl: Sized {}
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
pub trait IUIContentRootImpl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<UIContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIContextImpl: Sized {}
