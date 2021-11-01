#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Accessibility")]
pub mod Accessibility;
#[cfg(feature = "UI_ApplicationSettings")]
pub mod ApplicationSettings;
#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Core")]
pub mod Core;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Notifications")]
pub mod Notifications;
#[cfg(feature = "UI_Popups")]
pub mod Popups;
#[cfg(feature = "UI_Shell")]
pub mod Shell;
#[cfg(feature = "UI_StartScreen")]
pub mod StartScreen;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_UIAutomation")]
pub mod UIAutomation;
#[cfg(feature = "UI_ViewManagement")]
pub mod ViewManagement;
#[cfg(feature = "UI_WebUI")]
pub mod WebUI;
#[cfg(feature = "UI_WindowManagement")]
pub mod WindowManagement;
#[cfg(feature = "UI_Xaml")]
pub mod Xaml;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl Color {}
impl ::std::default::Default for Color {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Color {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Color").field("A", &self.A).field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::std::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A && self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::std::cmp::Eq for Color {}
unsafe impl ::windows::runtime::Abi for Color {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Color {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ColorHelper(::windows::runtime::IInspectable);
impl ColorHelper {
    #[doc = "*Required features: `UI`*"]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows::runtime::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), a, r, g, b, &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn ToDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, Color>>(color: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), color.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ColorHelper, IColorHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IColorHelperStatics2<R, F: FnOnce(&IColorHelperStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ColorHelper, IColorHelperStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ColorHelper;{193cfbe7-65c7-4540-ad08-6283ba76879a})");
}
unsafe impl ::windows::runtime::Interface for ColorHelper {
    type Vtable = IColorHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(423427047, 26055, 17728, [173, 8, 98, 131, 186, 118, 135, 154]);
}
impl ::windows::runtime::RuntimeName for ColorHelper {
    const NAME: &'static str = "Windows.UI.ColorHelper";
}
impl ::std::convert::From<ColorHelper> for ::windows::runtime::IUnknown {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ColorHelper> for ::windows::runtime::IUnknown {
    fn from(value: &ColorHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ColorHelper> for ::windows::runtime::IInspectable {
    fn from(value: ColorHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ColorHelper> for ::windows::runtime::IInspectable {
    fn from(value: &ColorHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ColorHelper {}
unsafe impl ::std::marker::Sync for ColorHelper {}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Colors(::windows::runtime::IInspectable);
impl Colors {
    #[doc = "*Required features: `UI`*"]
    pub fn AliceBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn AntiqueWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Aqua() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Aquamarine() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Azure() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Beige() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Bisque() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Black() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BlanchedAlmond() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Blue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BlueViolet() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Brown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BurlyWood() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn CadetBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Chartreuse() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Chocolate() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Coral() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn CornflowerBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Cornsilk() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Crimson() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Cyan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkCyan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGoldenrod() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkKhaki() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkMagenta() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOliveGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOrange() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOrchid() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSalmon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSlateBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSlateGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkTurquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkViolet() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DeepPink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DeepSkyBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DimGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DodgerBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Firebrick() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn FloralWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn ForestGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Fuchsia() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gainsboro() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn GhostWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gold() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Goldenrod() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Green() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn GreenYellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Honeydew() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn HotPink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn IndianRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Indigo() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Ivory() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Khaki() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Lavender() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).65)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LavenderBlush() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LawnGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LemonChiffon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightCoral() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightCyan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).71)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGoldenrodYellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightPink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSalmon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSkyBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSlateGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSteelBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).80)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightYellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Lime() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LimeGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Linen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).84)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Magenta() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Maroon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).86)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumAquamarine() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).88)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumOrchid() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).89)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumPurple() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).90)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSlateBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).92)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSpringGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumTurquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).94)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumVioletRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).95)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MidnightBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).96)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MintCream() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MistyRose() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).98)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Moccasin() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn NavajoWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).100)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Navy() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).101)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OldLace() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).102)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Olive() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OliveDrab() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).104)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Orange() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OrangeRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).106)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Orchid() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).107)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleGoldenrod() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleTurquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).110)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleVioletRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PapayaWhip() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).112)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PeachPuff() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).113)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Peru() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).114)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Pink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Plum() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).116)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PowderBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).117)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Purple() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Red() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).119)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn RosyBrown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn RoyalBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).121)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SaddleBrown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).122)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Salmon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).123)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SandyBrown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).125)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SeaShell() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Sienna() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).127)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Silver() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).128)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SkyBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).129)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SlateBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).130)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SlateGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).131)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Snow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).132)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SpringGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).133)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SteelBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).134)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Tan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).135)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Teal() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).136)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Thistle() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).137)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Tomato() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).138)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Transparent() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).139)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Turquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).140)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Violet() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).141)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Wheat() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).142)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn White() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).143)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn WhiteSmoke() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).144)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Yellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).145)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn YellowGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).146)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Colors, IColorsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Colors {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Colors;{9b8c9326-4ca6-4ce5-8994-9eff65cabdcc})");
}
unsafe impl ::windows::runtime::Interface for Colors {
    type Vtable = IColors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2609681190, 19622, 19685, [137, 148, 158, 255, 101, 202, 189, 204]);
}
impl ::windows::runtime::RuntimeName for Colors {
    const NAME: &'static str = "Windows.UI.Colors";
}
impl ::std::convert::From<Colors> for ::windows::runtime::IUnknown {
    fn from(value: Colors) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Colors> for ::windows::runtime::IUnknown {
    fn from(value: &Colors) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Colors> for ::windows::runtime::IInspectable {
    fn from(value: Colors) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Colors> for ::windows::runtime::IInspectable {
    fn from(value: &Colors) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Colors {}
unsafe impl ::std::marker::Sync for Colors {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IColorHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorHelper {
    type Vtable = IColorHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(423427047, 26055, 17728, [173, 8, 98, 131, 186, 118, 135, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IColorHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2231688170, 64362, 16708, [166, 194, 51, 73, 156, 146, 132, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IColorHelperStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorHelperStatics2 {
    type Vtable = IColorHelperStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(618245890, 28336, 19348, [133, 92, 252, 240, 129, 141, 154, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: Color, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IColors(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColors {
    type Vtable = IColors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2609681190, 19622, 19685, [137, 148, 158, 255, 101, 202, 189, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IColorsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorsStatics {
    type Vtable = IColorsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3488951812, 52390, 17940, [161, 126, 117, 73, 16, 200, 74, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Color) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUIContentRoot(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUIContentRoot {
    type Vtable = IUIContentRoot_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(503102150, 45931, 23737, [155, 197, 43, 122, 14, 221, 195, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContentRoot_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUIContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUIContext {
    type Vtable = IUIContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3143432909, 23512, 22992, [165, 158, 28, 23, 164, 214, 210, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UIContentRoot(::windows::runtime::IInspectable);
impl UIContentRoot {
    #[doc = "*Required features: `UI`*"]
    pub fn UIContext(&self) -> ::windows::runtime::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UIContext>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UIContentRoot {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIContentRoot;{1dfcbac6-b36b-5cb9-9bc5-2b7a0eddc378})");
}
unsafe impl ::windows::runtime::Interface for UIContentRoot {
    type Vtable = IUIContentRoot_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(503102150, 45931, 23737, [155, 197, 43, 122, 14, 221, 195, 120]);
}
impl ::windows::runtime::RuntimeName for UIContentRoot {
    const NAME: &'static str = "Windows.UI.UIContentRoot";
}
impl ::std::convert::From<UIContentRoot> for ::windows::runtime::IUnknown {
    fn from(value: UIContentRoot) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UIContentRoot> for ::windows::runtime::IUnknown {
    fn from(value: &UIContentRoot) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UIContentRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UIContentRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UIContentRoot> for ::windows::runtime::IInspectable {
    fn from(value: UIContentRoot) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UIContentRoot> for ::windows::runtime::IInspectable {
    fn from(value: &UIContentRoot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UIContentRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UIContentRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UIContentRoot {}
unsafe impl ::std::marker::Sync for UIContentRoot {}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UIContext(::windows::runtime::IInspectable);
impl UIContext {}
unsafe impl ::windows::runtime::RuntimeType for UIContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIContext;{bb5cfacd-5bd8-59d0-a59e-1c17a4d6d243})");
}
unsafe impl ::windows::runtime::Interface for UIContext {
    type Vtable = IUIContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3143432909, 23512, 22992, [165, 158, 28, 23, 164, 214, 210, 67]);
}
impl ::windows::runtime::RuntimeName for UIContext {
    const NAME: &'static str = "Windows.UI.UIContext";
}
impl ::std::convert::From<UIContext> for ::windows::runtime::IUnknown {
    fn from(value: UIContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UIContext> for ::windows::runtime::IUnknown {
    fn from(value: &UIContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UIContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UIContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UIContext> for ::windows::runtime::IInspectable {
    fn from(value: UIContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UIContext> for ::windows::runtime::IInspectable {
    fn from(value: &UIContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UIContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UIContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UIContext {}
unsafe impl ::std::marker::Sync for UIContext {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct WindowId {
    pub Value: u64,
}
impl WindowId {}
impl ::std::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WindowId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WindowId").field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for WindowId {}
unsafe impl ::windows::runtime::Abi for WindowId {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WindowId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.WindowId;u8)");
}
