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
#[doc = "*Required features: 'UI'*"]
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
}
impl ::windows::core::DefaultType for Color {
    type DefaultType = Self;
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
#[doc = "*Required features: 'UI'*"]
#[repr(transparent)]
pub struct ColorHelper(::windows::core::IUnknown);
impl ColorHelper {
    #[doc = "*Required features: 'UI'*"]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows::core::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), a, r, g, b, &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn ToDisplayName<'a, Param0: ::windows::core::IntoParam<'a, Color>>(color: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), color.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
}
unsafe impl ::windows::core::Interface for ColorHelper {
    type Vtable = IColorHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x193cfbe7_65c7_4540_ad08_6283ba76879a);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ColorHelper {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ColorHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[doc = "*Required features: 'UI'*"]
#[repr(transparent)]
pub struct Colors(::windows::core::IUnknown);
impl Colors {
    #[doc = "*Required features: 'UI'*"]
    pub fn AliceBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn AntiqueWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Aqua() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Aquamarine() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Azure() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Beige() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Bisque() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Black() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn BlanchedAlmond() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Blue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn BlueViolet() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Brown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn BurlyWood() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn CadetBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Chartreuse() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Chocolate() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Coral() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn CornflowerBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Cornsilk() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Crimson() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Cyan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkCyan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkGoldenrod() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkKhaki() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkMagenta() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkOliveGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkOrange() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkOrchid() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkSalmon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkSeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkSlateBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkSlateGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkTurquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DarkViolet() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DeepPink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DeepSkyBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DimGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn DodgerBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Firebrick() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn FloralWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn ForestGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Fuchsia() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Gainsboro() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn GhostWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Gold() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Goldenrod() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Gray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Green() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn GreenYellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Honeydew() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn HotPink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn IndianRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Indigo() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).62)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Ivory() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).63)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Khaki() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Lavender() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).65)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LavenderBlush() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).66)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LawnGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).67)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LemonChiffon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).68)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).69)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightCoral() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).70)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightCyan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).71)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightGoldenrodYellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).72)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).73)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).74)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightPink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).75)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightSalmon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).76)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightSeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).77)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightSkyBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).78)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightSlateGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).79)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightSteelBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).80)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LightYellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).81)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Lime() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).82)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn LimeGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).83)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Linen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).84)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Magenta() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).85)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Maroon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).86)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumAquamarine() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).87)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).88)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumOrchid() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).89)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumPurple() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).90)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumSeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).91)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumSlateBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).92)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumSpringGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).93)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumTurquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).94)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MediumVioletRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).95)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MidnightBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).96)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MintCream() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).97)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn MistyRose() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).98)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Moccasin() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).99)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn NavajoWhite() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).100)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Navy() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).101)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn OldLace() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).102)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Olive() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).103)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn OliveDrab() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).104)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Orange() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).105)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn OrangeRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).106)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Orchid() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).107)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PaleGoldenrod() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).108)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PaleGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).109)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PaleTurquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).110)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PaleVioletRed() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).111)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PapayaWhip() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).112)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PeachPuff() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).113)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Peru() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).114)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Pink() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).115)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Plum() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).116)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn PowderBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).117)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Purple() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).118)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Red() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).119)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn RosyBrown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).120)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn RoyalBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).121)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SaddleBrown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).122)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Salmon() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).123)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SandyBrown() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).124)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SeaGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).125)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SeaShell() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).126)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Sienna() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).127)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Silver() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).128)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SkyBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).129)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SlateBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).130)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SlateGray() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).131)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Snow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).132)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SpringGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).133)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn SteelBlue() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).134)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Tan() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).135)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Teal() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).136)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Thistle() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).137)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Tomato() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).138)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Transparent() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).139)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Turquoise() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).140)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Violet() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).141)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Wheat() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).142)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn White() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).143)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn WhiteSmoke() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).144)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn Yellow() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).145)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: 'UI'*"]
    pub fn YellowGreen() -> ::windows::core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).146)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Color>(result__)
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
}
unsafe impl ::windows::core::Interface for Colors {
    type Vtable = IColorsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b8c9326_4ca6_4ce5_8994_9eff65cabdcc);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Colors {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Colors {
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
    type Vtable = IColorHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x193cfbe7_65c7_4540_ad08_6283ba76879a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8504dbea_fb6a_4144_a6c2_33499c9284f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelperStatics2 {
    type Vtable = IColorHelperStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24d9af02_6eb0_4b94_855c_fcf0818d9a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: Color, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColors(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColors {
    type Vtable = IColorsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b8c9326_4ca6_4ce5_8994_9eff65cabdcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorsStatics {
    type Vtable = IColorsStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcff52e04_cca6_4614_a17e_754910c84a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIContentRoot(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUIContentRoot {
    type Vtable = IUIContentRootVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dfcbac6_b36b_5cb9_9bc5_2b7a0eddc378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContentRootVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUIContext {
    type Vtable = IUIContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5cfacd_5bd8_59d0_a59e_1c17a4d6d243);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI'*"]
#[repr(transparent)]
pub struct UIContentRoot(::windows::core::IUnknown);
impl UIContentRoot {
    #[doc = "*Required features: 'UI'*"]
    pub fn UIContext(&self) -> ::windows::core::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UIContext>(result__)
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
}
unsafe impl ::windows::core::Interface for UIContentRoot {
    type Vtable = IUIContentRootVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dfcbac6_b36b_5cb9_9bc5_2b7a0eddc378);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UIContentRoot {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UIContentRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIContentRoot {}
unsafe impl ::core::marker::Sync for UIContentRoot {}
#[doc = "*Required features: 'UI'*"]
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
}
unsafe impl ::windows::core::Interface for UIContext {
    type Vtable = IUIContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5cfacd_5bd8_59d0_a59e_1c17a4d6d243);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UIContext {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UIContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIContext {}
unsafe impl ::core::marker::Sync for UIContext {}
#[repr(C)]
#[doc = "*Required features: 'UI'*"]
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
}
impl ::windows::core::DefaultType for WindowId {
    type DefaultType = Self;
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
