#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl ::core::marker::Copy for Color {}
impl ::core::clone::Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Color").field("A", &self.A).field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
unsafe impl ::windows::core::Abi for Color {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Color {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Color>()) == 0 }
    }
}
impl ::core::cmp::Eq for Color {}
impl ::core::default::Default for Color {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct ColorHelper(::windows::core::IUnknown);
impl ColorHelper {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows::core::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromArgb)(::core::mem::transmute_copy(this), a, r, g, b, &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn ToDisplayName<'a, Param0: ::windows::core::IntoParam<'a, Color>>(color: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToDisplayName)(::core::mem::transmute_copy(this), color.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorHelper, IColorHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics2<R, F: FnOnce(&IColorHelperStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorHelper, IColorHelperStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorHelper {}
impl ::core::fmt::Debug for ColorHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ColorHelper;{193cfbe7-65c7-4540-ad08-6283ba76879a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: ::windows::core::GUID = <IColorHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Windows.UI.ColorHelper";
}
impl ::core::convert::From<ColorHelper> for ::windows::core::IUnknown {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows::core::IUnknown {
    fn from(value: &ColorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorHelper> for ::windows::core::IInspectable {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows::core::IInspectable {
    fn from(value: &ColorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct Colors(::windows::core::IUnknown);
impl Colors {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn AliceBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AliceBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn AntiqueWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AntiqueWhite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Aqua() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Aqua)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Aquamarine() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Aquamarine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Azure() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Azure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Beige() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Beige)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Bisque() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bisque)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Black() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Black)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BlanchedAlmond() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BlanchedAlmond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Blue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Blue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BlueViolet() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BlueViolet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Brown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Brown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BurlyWood() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BurlyWood)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn CadetBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CadetBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Chartreuse() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Chartreuse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Chocolate() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Chocolate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Coral() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Coral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn CornflowerBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CornflowerBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Cornsilk() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cornsilk)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Crimson() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Crimson)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Cyan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cyan)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkCyan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkCyan)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGoldenrod() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkGoldenrod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkGray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkKhaki() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkKhaki)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkMagenta() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkMagenta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOliveGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkOliveGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOrange() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkOrange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOrchid() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkOrchid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkRed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSalmon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkSalmon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkSeaGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSlateBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkSlateBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSlateGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkSlateGray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkTurquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkTurquoise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkViolet() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DarkViolet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DeepPink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeepPink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DeepSkyBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeepSkyBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DimGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DimGray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DodgerBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DodgerBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Firebrick() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Firebrick)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn FloralWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FloralWhite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn ForestGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForestGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Fuchsia() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Fuchsia)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gainsboro() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gainsboro)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn GhostWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GhostWhite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gold() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Goldenrod() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Goldenrod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Green() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Green)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn GreenYellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GreenYellow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Honeydew() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Honeydew)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn HotPink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HotPink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn IndianRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndianRed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Indigo() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Indigo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Ivory() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ivory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Khaki() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Khaki)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Lavender() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lavender)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LavenderBlush() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LavenderBlush)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LawnGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LawnGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LemonChiffon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LemonChiffon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightCoral() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightCoral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightCyan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightCyan)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGoldenrodYellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightGoldenrodYellow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightGray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightPink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightPink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSalmon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightSalmon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightSeaGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSkyBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightSkyBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSlateGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightSlateGray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSteelBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightSteelBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightYellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LightYellow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Lime() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LimeGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LimeGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Linen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Linen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Magenta() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Magenta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Maroon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Maroon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumAquamarine() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumAquamarine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumOrchid() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumOrchid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumPurple() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumPurple)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumSeaGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSlateBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumSlateBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSpringGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumSpringGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumTurquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumTurquoise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumVioletRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediumVioletRed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MidnightBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MidnightBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MintCream() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MintCream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MistyRose() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MistyRose)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Moccasin() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Moccasin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn NavajoWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavajoWhite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Navy() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Navy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OldLace() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldLace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Olive() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Olive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OliveDrab() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OliveDrab)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Orange() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OrangeRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OrangeRed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Orchid() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orchid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleGoldenrod() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PaleGoldenrod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PaleGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleTurquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PaleTurquoise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleVioletRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PaleVioletRed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PapayaWhip() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PapayaWhip)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PeachPuff() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PeachPuff)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Peru() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Peru)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Pink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Plum() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Plum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PowderBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowderBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Purple() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Purple)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Red() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Red)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn RosyBrown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RosyBrown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn RoyalBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoyalBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SaddleBrown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaddleBrown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Salmon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Salmon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SandyBrown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SandyBrown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SeaGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SeaShell() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SeaShell)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Sienna() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Sienna)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Silver() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Silver)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SkyBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SkyBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SlateBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlateBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SlateGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlateGray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Snow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Snow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SpringGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpringGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SteelBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SteelBlue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Tan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tan)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Teal() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Teal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Thistle() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thistle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Tomato() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tomato)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Transparent() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transparent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Turquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Turquoise)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Violet() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Violet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Wheat() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Wheat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn White() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).White)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn WhiteSmoke() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WhiteSmoke)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Yellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Yellow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn YellowGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).YellowGreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Colors, IColorsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Colors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Colors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Colors {}
impl ::core::fmt::Debug for Colors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Colors").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Colors {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Colors;{9b8c9326-4ca6-4ce5-8994-9eff65cabdcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Colors {
    type Vtable = IColors_Vtbl;
    const IID: ::windows::core::GUID = <IColors as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Colors {
    const NAME: &'static str = "Windows.UI.Colors";
}
impl ::core::convert::From<Colors> for ::windows::core::IUnknown {
    fn from(value: Colors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Colors> for ::windows::core::IUnknown {
    fn from(value: &Colors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Colors> for ::windows::core::IInspectable {
    fn from(value: Colors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Colors> for ::windows::core::IInspectable {
    fn from(value: &Colors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Colors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x193cfbe7_65c7_4540_ad08_6283ba76879a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8504dbea_fb6a_4144_a6c2_33499c9284f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromArgb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelperStatics2 {
    type Vtable = IColorHelperStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24d9af02_6eb0_4b94_855c_fcf0818d9a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ToDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: Color, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColors(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColors {
    type Vtable = IColors_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b8c9326_4ca6_4ce5_8994_9eff65cabdcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorsStatics {
    type Vtable = IColorsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcff52e04_cca6_4614_a17e_754910c84a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AliceBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Aqua: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Azure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Beige: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Bisque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Black: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Blue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Brown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Coral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Crimson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Cyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DimGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Gold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Gray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Green: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub HotPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Indigo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Ivory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Khaki: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Lavender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Lime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Linen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Magenta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Maroon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MintCream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Navy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub OldLace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Olive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Orange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Orchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Peru: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Pink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Plum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Purple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Red: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Salmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Sienna: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Silver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Snow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Tan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Teal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Thistle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Tomato: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Transparent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Violet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Wheat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub White: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub Yellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIContentRoot(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dfcbac6_b36b_5cb9_9bc5_2b7a0eddc378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContentRoot_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUIContext {
    type Vtable = IUIContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5cfacd_5bd8_59d0_a59e_1c17a4d6d243);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContext_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct UIContentRoot(::windows::core::IUnknown);
impl UIContentRoot {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn UIContext(&self) -> ::windows::core::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UIContext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UIContext>(result__)
        }
    }
}
impl ::core::clone::Clone for UIContentRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UIContentRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UIContentRoot {}
impl ::core::fmt::Debug for UIContentRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIContentRoot").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIContentRoot {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIContentRoot;{1dfcbac6-b36b-5cb9-9bc5-2b7a0eddc378})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
    const IID: ::windows::core::GUID = <IUIContentRoot as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UIContentRoot {
    const NAME: &'static str = "Windows.UI.UIContentRoot";
}
impl ::core::convert::From<UIContentRoot> for ::windows::core::IUnknown {
    fn from(value: UIContentRoot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContentRoot> for ::windows::core::IUnknown {
    fn from(value: &UIContentRoot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UIContentRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UIContentRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UIContentRoot> for ::windows::core::IInspectable {
    fn from(value: UIContentRoot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContentRoot> for ::windows::core::IInspectable {
    fn from(value: &UIContentRoot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UIContentRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UIContentRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIContentRoot {}
unsafe impl ::core::marker::Sync for UIContentRoot {}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct UIContext(::windows::core::IUnknown);
impl UIContext {}
impl ::core::clone::Clone for UIContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UIContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UIContext {}
impl ::core::fmt::Debug for UIContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIContext;{bb5cfacd-5bd8-59d0-a59e-1c17a4d6d243})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UIContext {
    type Vtable = IUIContext_Vtbl;
    const IID: ::windows::core::GUID = <IUIContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UIContext {
    const NAME: &'static str = "Windows.UI.UIContext";
}
impl ::core::convert::From<UIContext> for ::windows::core::IUnknown {
    fn from(value: UIContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContext> for ::windows::core::IUnknown {
    fn from(value: &UIContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UIContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UIContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UIContext> for ::windows::core::IInspectable {
    fn from(value: UIContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContext> for ::windows::core::IInspectable {
    fn from(value: &UIContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UIContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UIContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIContext {}
unsafe impl ::core::marker::Sync for UIContext {}
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WindowId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WindowId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for WindowId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WindowId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.WindowId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WindowId>()) == 0 }
    }
}
impl ::core::cmp::Eq for WindowId {}
impl ::core::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
