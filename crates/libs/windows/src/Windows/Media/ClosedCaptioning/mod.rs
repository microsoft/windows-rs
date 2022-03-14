#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: Self = Self(0i32);
    pub const White: Self = Self(1i32);
    pub const Black: Self = Self(2i32);
    pub const Red: Self = Self(3i32);
    pub const Green: Self = Self(4i32);
    pub const Blue: Self = Self(5i32);
    pub const Yellow: Self = Self(6i32);
    pub const Magenta: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
}
impl ::core::marker::Copy for ClosedCaptionColor {}
impl ::core::clone::Clone for ClosedCaptionColor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionColor {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionColor {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionColor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionColor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Raised: Self = Self(2i32);
    pub const Depressed: Self = Self(3i32);
    pub const Uniform: Self = Self(4i32);
    pub const DropShadow: Self = Self(5i32);
}
impl ::core::marker::Copy for ClosedCaptionEdgeEffect {}
impl ::core::clone::Clone for ClosedCaptionEdgeEffect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionEdgeEffect {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionEdgeEffect {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionEdgeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionEdgeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: Self = Self(0i32);
    pub const OneHundredPercent: Self = Self(1i32);
    pub const SeventyFivePercent: Self = Self(2i32);
    pub const TwentyFivePercent: Self = Self(3i32);
    pub const ZeroPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionOpacity {}
impl ::core::clone::Clone for ClosedCaptionOpacity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionOpacity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionOpacity {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionOpacity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionOpacity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
pub struct ClosedCaptionProperties {}
impl ClosedCaptionProperties {
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn FontColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ComputedFontColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputedFontColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn FontOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontOpacity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn FontSize() -> ::windows::core::Result<ClosedCaptionSize> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionSize>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn FontStyle() -> ::windows::core::Result<ClosedCaptionStyle> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionStyle>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn FontEffect() -> ::windows::core::Result<ClosedCaptionEdgeEffect> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionEdgeEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontEffect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionEdgeEffect>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn BackgroundColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ComputedBackgroundColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputedBackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn BackgroundOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundOpacity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn RegionColor() -> ::windows::core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegionColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ComputedRegionColor() -> ::windows::core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputedRegionColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
    pub fn RegionOpacity() -> ::windows::core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegionOpacity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IClosedCaptionPropertiesStatics<R, F: FnOnce(&IClosedCaptionPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: Self = Self(0i32);
    pub const FiftyPercent: Self = Self(1i32);
    pub const OneHundredPercent: Self = Self(2i32);
    pub const OneHundredFiftyPercent: Self = Self(3i32);
    pub const TwoHundredPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionSize {}
impl ::core::clone::Clone for ClosedCaptionSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: Self = Self(0i32);
    pub const MonospacedWithSerifs: Self = Self(1i32);
    pub const ProportionalWithSerifs: Self = Self(2i32);
    pub const MonospacedWithoutSerifs: Self = Self(3i32);
    pub const ProportionalWithoutSerifs: Self = Self(4i32);
    pub const Casual: Self = Self(5i32);
    pub const Cursive: Self = Self(6i32);
    pub const SmallCapitals: Self = Self(7i32);
}
impl ::core::marker::Copy for ClosedCaptionStyle {}
impl ::core::clone::Clone for ClosedCaptionStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ClosedCaptionStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClosedCaptionPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClosedCaptionPropertiesStatics {
    type Vtable = IClosedCaptionPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10aa1f84_cc30_4141_b503_5272289e0c20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedFontColor: usize,
    pub FontOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionSize) -> ::windows::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionStyle) -> ::windows::core::HRESULT,
    pub FontEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionEdgeEffect) -> ::windows::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedBackgroundColor: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
    pub RegionColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedRegionColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedRegionColor: usize,
    pub RegionOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
