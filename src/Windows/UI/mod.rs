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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl Color {}
impl ::core::default::Default for Color {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Color {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Color").field("A", &self.A).field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::core::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A && self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for Color {}
unsafe impl ::windows::runtime::Abi for Color {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Color {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
impl ::windows::runtime::DefaultType for Color {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorHelper(pub ::windows::runtime::IInspectable);
impl ColorHelper {
    #[doc = "*Required features: `UI`*"]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows::runtime::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), a, r, g, b, &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn ToDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, Color>>(color: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), color.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
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
impl ::core::convert::From<ColorHelper> for ::windows::runtime::IUnknown {
    fn from(value: ColorHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows::runtime::IUnknown {
    fn from(value: &ColorHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorHelper> for ::windows::runtime::IInspectable {
    fn from(value: ColorHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows::runtime::IInspectable {
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
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Colors(pub ::windows::runtime::IInspectable);
impl Colors {
    #[doc = "*Required features: `UI`*"]
    pub fn AliceBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn AntiqueWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Aqua() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Aquamarine() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Azure() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Beige() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Bisque() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Black() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BlanchedAlmond() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Blue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BlueViolet() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Brown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BurlyWood() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn CadetBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Chartreuse() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Chocolate() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Coral() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn CornflowerBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Cornsilk() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Crimson() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Cyan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkCyan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGoldenrod() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkKhaki() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkMagenta() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOliveGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOrange() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOrchid() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSalmon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSlateBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSlateGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkTurquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkViolet() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DeepPink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DeepSkyBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DimGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DodgerBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Firebrick() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn FloralWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn ForestGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Fuchsia() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gainsboro() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn GhostWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gold() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Goldenrod() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Green() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn GreenYellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Honeydew() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn HotPink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn IndianRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Indigo() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Ivory() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Khaki() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Lavender() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).65)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LavenderBlush() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LawnGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LemonChiffon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightCoral() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightCyan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).71)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGoldenrodYellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightPink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSalmon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSkyBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSlateGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSteelBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).80)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightYellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Lime() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LimeGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Linen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).84)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Magenta() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Maroon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).86)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumAquamarine() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).88)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumOrchid() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).89)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumPurple() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).90)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSlateBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).92)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSpringGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumTurquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).94)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumVioletRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).95)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MidnightBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).96)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MintCream() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MistyRose() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).98)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Moccasin() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn NavajoWhite() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).100)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Navy() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).101)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OldLace() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).102)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Olive() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OliveDrab() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).104)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Orange() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OrangeRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).106)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Orchid() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).107)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleGoldenrod() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleTurquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).110)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleVioletRed() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PapayaWhip() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).112)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PeachPuff() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).113)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Peru() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).114)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Pink() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Plum() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).116)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PowderBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).117)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Purple() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Red() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).119)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn RosyBrown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn RoyalBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).121)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SaddleBrown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).122)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Salmon() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).123)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SandyBrown() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SeaGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).125)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SeaShell() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Sienna() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).127)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Silver() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).128)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SkyBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).129)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SlateBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).130)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SlateGray() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).131)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Snow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).132)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SpringGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).133)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SteelBlue() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).134)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Tan() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).135)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Teal() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).136)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Thistle() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).137)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Tomato() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).138)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Transparent() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).139)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Turquoise() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).140)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Violet() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).141)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Wheat() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).142)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn White() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).143)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn WhiteSmoke() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).144)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Yellow() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).145)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn YellowGreen() -> ::windows::runtime::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).146)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
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
impl ::core::convert::From<Colors> for ::windows::runtime::IUnknown {
    fn from(value: Colors) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Colors> for ::windows::runtime::IUnknown {
    fn from(value: &Colors) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Colors> for ::windows::runtime::IInspectable {
    fn from(value: Colors) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Colors> for ::windows::runtime::IInspectable {
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
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorHelper(pub ::windows::runtime::IInspectable);
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
#[doc(hidden)]
pub struct IColorHelperStatics(pub ::windows::runtime::IInspectable);
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
#[doc(hidden)]
pub struct IColorHelperStatics2(pub ::windows::runtime::IInspectable);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: Color, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColors(pub ::windows::runtime::IInspectable);
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
#[doc(hidden)]
pub struct IColorsStatics(pub ::windows::runtime::IInspectable);
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
#[doc(hidden)]
pub struct IUIContentRoot(pub ::windows::runtime::IInspectable);
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
#[doc(hidden)]
pub struct IUIContext(pub ::windows::runtime::IInspectable);
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UIContentRoot(pub ::windows::runtime::IInspectable);
impl UIContentRoot {
    #[doc = "*Required features: `UI`*"]
    pub fn UIContext(&self) -> ::windows::runtime::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UIContext>(result__)
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
impl ::core::convert::From<UIContentRoot> for ::windows::runtime::IUnknown {
    fn from(value: UIContentRoot) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UIContentRoot> for ::windows::runtime::IUnknown {
    fn from(value: &UIContentRoot) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UIContentRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UIContentRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UIContentRoot> for ::windows::runtime::IInspectable {
    fn from(value: UIContentRoot) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UIContentRoot> for ::windows::runtime::IInspectable {
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
unsafe impl ::core::marker::Send for UIContentRoot {}
unsafe impl ::core::marker::Sync for UIContentRoot {}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UIContext(pub ::windows::runtime::IInspectable);
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
impl ::core::convert::From<UIContext> for ::windows::runtime::IUnknown {
    fn from(value: UIContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UIContext> for ::windows::runtime::IUnknown {
    fn from(value: &UIContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UIContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UIContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UIContext> for ::windows::runtime::IInspectable {
    fn from(value: UIContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UIContext> for ::windows::runtime::IInspectable {
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
unsafe impl ::core::marker::Send for UIContext {}
unsafe impl ::core::marker::Sync for UIContext {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct WindowId {
    pub Value: u64,
}
impl WindowId {}
impl ::core::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WindowId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WindowId").field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WindowId {}
unsafe impl ::windows::runtime::Abi for WindowId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WindowId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.WindowId;u8)");
}
impl ::windows::runtime::DefaultType for WindowId {
    type DefaultType = Self;
}
