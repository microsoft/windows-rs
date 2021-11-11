#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_ClosedCaptioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: ClosedCaptionColor = ClosedCaptionColor(0i32);
    pub const White: ClosedCaptionColor = ClosedCaptionColor(1i32);
    pub const Black: ClosedCaptionColor = ClosedCaptionColor(2i32);
    pub const Red: ClosedCaptionColor = ClosedCaptionColor(3i32);
    pub const Green: ClosedCaptionColor = ClosedCaptionColor(4i32);
    pub const Blue: ClosedCaptionColor = ClosedCaptionColor(5i32);
    pub const Yellow: ClosedCaptionColor = ClosedCaptionColor(6i32);
    pub const Magenta: ClosedCaptionColor = ClosedCaptionColor(7i32);
    pub const Cyan: ClosedCaptionColor = ClosedCaptionColor(8i32);
}
impl ::core::convert::From<i32> for ClosedCaptionColor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionColor {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
}
impl ::windows::core::DefaultType for ClosedCaptionColor {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_ClosedCaptioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(0i32);
    pub const None: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(1i32);
    pub const Raised: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(2i32);
    pub const Depressed: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(3i32);
    pub const Uniform: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(4i32);
    pub const DropShadow: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(5i32);
}
impl ::core::convert::From<i32> for ClosedCaptionEdgeEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionEdgeEffect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
}
impl ::windows::core::DefaultType for ClosedCaptionEdgeEffect {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_ClosedCaptioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: ClosedCaptionOpacity = ClosedCaptionOpacity(0i32);
    pub const OneHundredPercent: ClosedCaptionOpacity = ClosedCaptionOpacity(1i32);
    pub const SeventyFivePercent: ClosedCaptionOpacity = ClosedCaptionOpacity(2i32);
    pub const TwentyFivePercent: ClosedCaptionOpacity = ClosedCaptionOpacity(3i32);
    pub const ZeroPercent: ClosedCaptionOpacity = ClosedCaptionOpacity(4i32);
}
impl ::core::convert::From<i32> for ClosedCaptionOpacity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionOpacity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
}
impl ::windows::core::DefaultType for ClosedCaptionOpacity {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_ClosedCaptioning`*"]
pub struct ClosedCaptionProperties {}
impl ClosedCaptionProperties {
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn FontColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Media_ClosedCaptioning`, `UI`*"]
    pub fn ComputedFontColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn FontOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn FontSize() -> ::windows::core::Result<ClosedCaptionSize> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionSize>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn FontStyle() -> ::windows::core::Result<ClosedCaptionStyle> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionStyle>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn FontEffect() -> ::windows::core::Result<ClosedCaptionEdgeEffect> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionEdgeEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionEdgeEffect>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn BackgroundColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Media_ClosedCaptioning`, `UI`*"]
    pub fn ComputedBackgroundColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn BackgroundOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn RegionColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Media_ClosedCaptioning`, `UI`*"]
    pub fn ComputedRegionColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `Media_ClosedCaptioning`*"]
    pub fn RegionOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn IClosedCaptionPropertiesStatics<R, F: FnOnce(&IClosedCaptionPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[doc = "*Required features: `Media_ClosedCaptioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: ClosedCaptionSize = ClosedCaptionSize(0i32);
    pub const FiftyPercent: ClosedCaptionSize = ClosedCaptionSize(1i32);
    pub const OneHundredPercent: ClosedCaptionSize = ClosedCaptionSize(2i32);
    pub const OneHundredFiftyPercent: ClosedCaptionSize = ClosedCaptionSize(3i32);
    pub const TwoHundredPercent: ClosedCaptionSize = ClosedCaptionSize(4i32);
}
impl ::core::convert::From<i32> for ClosedCaptionSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
}
impl ::windows::core::DefaultType for ClosedCaptionSize {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_ClosedCaptioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: ClosedCaptionStyle = ClosedCaptionStyle(0i32);
    pub const MonospacedWithSerifs: ClosedCaptionStyle = ClosedCaptionStyle(1i32);
    pub const ProportionalWithSerifs: ClosedCaptionStyle = ClosedCaptionStyle(2i32);
    pub const MonospacedWithoutSerifs: ClosedCaptionStyle = ClosedCaptionStyle(3i32);
    pub const ProportionalWithoutSerifs: ClosedCaptionStyle = ClosedCaptionStyle(4i32);
    pub const Casual: ClosedCaptionStyle = ClosedCaptionStyle(5i32);
    pub const Cursive: ClosedCaptionStyle = ClosedCaptionStyle(6i32);
    pub const SmallCapitals: ClosedCaptionStyle = ClosedCaptionStyle(7i32);
}
impl ::core::convert::From<i32> for ClosedCaptionStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
}
impl ::windows::core::DefaultType for ClosedCaptionStyle {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClosedCaptionPropertiesStatics {
    type Vtable = IClosedCaptionPropertiesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10aa1f84_cc30_4141_b503_5272289e0c20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionSize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionEdgeEffect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
);
