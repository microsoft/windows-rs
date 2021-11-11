#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AdvancedColorInfo(pub ::windows::runtime::IInspectable);
impl AdvancedColorInfo {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CurrentAdvancedColorKind(&self) -> ::windows::runtime::Result<AdvancedColorKind> {
        let this = self;
        unsafe {
            let mut result__: AdvancedColorKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedColorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RedPrimary(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn GreenPrimary(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn BluePrimary(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn WhitePoint(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn MaxLuminanceInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn MinLuminanceInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SdrWhiteLevelInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdvancedColorInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.AdvancedColorInfo;{8797dcfb-b229-4081-ae9a-2cc85e34ad6a})");
}
unsafe impl ::windows::runtime::Interface for AdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8797dcfb_b229_4081_ae9a_2cc85e34ad6a);
}
impl ::windows::runtime::RuntimeName for AdvancedColorInfo {
    const NAME: &'static str = "Windows.Graphics.Display.AdvancedColorInfo";
}
impl ::core::convert::From<AdvancedColorInfo> for ::windows::runtime::IUnknown {
    fn from(value: AdvancedColorInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AdvancedColorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AdvancedColorInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdvancedColorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdvancedColorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AdvancedColorInfo> for ::windows::runtime::IInspectable {
    fn from(value: AdvancedColorInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AdvancedColorInfo> for ::windows::runtime::IInspectable {
    fn from(value: &AdvancedColorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdvancedColorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdvancedColorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AdvancedColorInfo {}
unsafe impl ::core::marker::Sync for AdvancedColorInfo {}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: AdvancedColorKind = AdvancedColorKind(0i32);
    pub const WideColorGamut: AdvancedColorKind = AdvancedColorKind(1i32);
    pub const HighDynamicRange: AdvancedColorKind = AdvancedColorKind(2i32);
}
impl ::core::convert::From<i32> for AdvancedColorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdvancedColorKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AdvancedColorKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.AdvancedColorKind;i4)");
}
impl ::windows::runtime::DefaultType for AdvancedColorKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BrightnessOverride(pub ::windows::runtime::IInspectable);
impl BrightnessOverride {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsOverrideActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn BrightnessLevel(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SetBrightnessLevel(&self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), brightnesslevel, options).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), scenario, options).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), scenario, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn StartOverride(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn StopOverride(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn IsSupportedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveIsSupportedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn IsOverrideActiveChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveIsOverrideActiveChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn BrightnessLevelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveBrightnessLevelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetDefaultForSystem() -> ::windows::runtime::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrightnessOverride>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrightnessOverride>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn SaveForSystemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, BrightnessOverride>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IBrightnessOverrideStatics<R, F: FnOnce(&IBrightnessOverrideStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BrightnessOverride, IBrightnessOverrideStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BrightnessOverride {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.BrightnessOverride;{96c9621a-c143-4392-bedd-4a7e9574c8fd})");
}
unsafe impl ::windows::runtime::Interface for BrightnessOverride {
    type Vtable = IBrightnessOverride_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96c9621a_c143_4392_bedd_4a7e9574c8fd);
}
impl ::windows::runtime::RuntimeName for BrightnessOverride {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverride";
}
impl ::core::convert::From<BrightnessOverride> for ::windows::runtime::IUnknown {
    fn from(value: BrightnessOverride) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BrightnessOverride> for ::windows::runtime::IUnknown {
    fn from(value: &BrightnessOverride) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BrightnessOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BrightnessOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BrightnessOverride> for ::windows::runtime::IInspectable {
    fn from(value: BrightnessOverride) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BrightnessOverride> for ::windows::runtime::IInspectable {
    fn from(value: &BrightnessOverride) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BrightnessOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BrightnessOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BrightnessOverride {}
unsafe impl ::core::marker::Sync for BrightnessOverride {}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BrightnessOverrideSettings(pub ::windows::runtime::IInspectable);
impl BrightnessOverrideSettings {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn DesiredLevel(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn DesiredNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CreateFromLevel(level: f64) -> ::windows::runtime::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CreateFromNits(nits: f32) -> ::windows::runtime::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), nits, &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CreateFromDisplayBrightnessOverrideScenario(overridescenario: DisplayBrightnessOverrideScenario) -> ::windows::runtime::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), overridescenario, &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    pub fn IBrightnessOverrideSettingsStatics<R, F: FnOnce(&IBrightnessOverrideSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BrightnessOverrideSettings, IBrightnessOverrideSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BrightnessOverrideSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.BrightnessOverrideSettings;{d112ab2a-7604-4dba-bcf8-4b6f49502cb0})");
}
unsafe impl ::windows::runtime::Interface for BrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd112ab2a_7604_4dba_bcf8_4b6f49502cb0);
}
impl ::windows::runtime::RuntimeName for BrightnessOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverrideSettings";
}
impl ::core::convert::From<BrightnessOverrideSettings> for ::windows::runtime::IUnknown {
    fn from(value: BrightnessOverrideSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BrightnessOverrideSettings> for ::windows::runtime::IUnknown {
    fn from(value: &BrightnessOverrideSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BrightnessOverrideSettings> for ::windows::runtime::IInspectable {
    fn from(value: BrightnessOverrideSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BrightnessOverrideSettings> for ::windows::runtime::IInspectable {
    fn from(value: &BrightnessOverrideSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BrightnessOverrideSettings {}
unsafe impl ::core::marker::Sync for BrightnessOverrideSettings {}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ColorOverrideSettings(pub ::windows::runtime::IInspectable);
impl ColorOverrideSettings {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn DesiredDisplayColorOverrideScenario(&self) -> ::windows::runtime::Result<DisplayColorOverrideScenario> {
        let this = self;
        unsafe {
            let mut result__: DisplayColorOverrideScenario = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayColorOverrideScenario>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CreateFromDisplayColorOverrideScenario(overridescenario: DisplayColorOverrideScenario) -> ::windows::runtime::Result<ColorOverrideSettings> {
        Self::IColorOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), overridescenario, &mut result__).from_abi::<ColorOverrideSettings>(result__)
        })
    }
    pub fn IColorOverrideSettingsStatics<R, F: FnOnce(&IColorOverrideSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ColorOverrideSettings, IColorOverrideSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ColorOverrideSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.ColorOverrideSettings;{fbefa134-4a81-4c4d-a5b6-7d1b5c4bd00b})");
}
unsafe impl ::windows::runtime::Interface for ColorOverrideSettings {
    type Vtable = IColorOverrideSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfbefa134_4a81_4c4d_a5b6_7d1b5c4bd00b);
}
impl ::windows::runtime::RuntimeName for ColorOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.ColorOverrideSettings";
}
impl ::core::convert::From<ColorOverrideSettings> for ::windows::runtime::IUnknown {
    fn from(value: ColorOverrideSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ColorOverrideSettings> for ::windows::runtime::IUnknown {
    fn from(value: &ColorOverrideSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ColorOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ColorOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ColorOverrideSettings> for ::windows::runtime::IInspectable {
    fn from(value: ColorOverrideSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ColorOverrideSettings> for ::windows::runtime::IInspectable {
    fn from(value: &ColorOverrideSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ColorOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ColorOverrideSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ColorOverrideSettings {}
unsafe impl ::core::marker::Sync for ColorOverrideSettings {}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: DisplayBrightnessOverrideOptions = DisplayBrightnessOverrideOptions(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: DisplayBrightnessOverrideOptions = DisplayBrightnessOverrideOptions(1u32);
}
impl ::core::convert::From<u32> for DisplayBrightnessOverrideOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayBrightnessOverrideOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayBrightnessOverrideOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideOptions;u4)");
}
impl ::windows::runtime::DefaultType for DisplayBrightnessOverrideOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayBrightnessOverrideOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayBrightnessOverrideOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: DisplayBrightnessOverrideScenario = DisplayBrightnessOverrideScenario(0i32);
    pub const BarcodeReadingBrightness: DisplayBrightnessOverrideScenario = DisplayBrightnessOverrideScenario(1i32);
    pub const FullBrightness: DisplayBrightnessOverrideScenario = DisplayBrightnessOverrideScenario(2i32);
}
impl ::core::convert::From<i32> for DisplayBrightnessOverrideScenario {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayBrightnessOverrideScenario {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayBrightnessOverrideScenario {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideScenario;i4)");
}
impl ::windows::runtime::DefaultType for DisplayBrightnessOverrideScenario {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(0i32);
    pub const IdleBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(1i32);
    pub const BarcodeReadingBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(2i32);
    pub const FullBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(3i32);
}
impl ::core::convert::From<i32> for DisplayBrightnessScenario {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayBrightnessScenario {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayBrightnessScenario {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessScenario;i4)");
}
impl ::windows::runtime::DefaultType for DisplayBrightnessScenario {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: DisplayColorOverrideScenario = DisplayColorOverrideScenario(0i32);
}
impl ::core::convert::From<i32> for DisplayColorOverrideScenario {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayColorOverrideScenario {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayColorOverrideScenario {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayColorOverrideScenario;i4)");
}
impl ::windows::runtime::DefaultType for DisplayColorOverrideScenario {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayEnhancementOverride(pub ::windows::runtime::IInspectable);
impl DisplayEnhancementOverride {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn ColorOverrideSettings(&self) -> ::windows::runtime::Result<ColorOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ColorOverrideSettings>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SetColorOverrideSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ColorOverrideSettings>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn BrightnessOverrideSettings(&self) -> ::windows::runtime::Result<BrightnessOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SetBrightnessOverrideSettings<'a, Param0: ::windows::runtime::IntoParam<'a, BrightnessOverrideSettings>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CanOverride(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsOverrideActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetCurrentDisplayEnhancementOverrideCapabilities(&self) -> ::windows::runtime::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayEnhancementOverrideCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn RequestOverride(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn StopOverride(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn CanOverrideChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveCanOverrideChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn IsOverrideActiveChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveIsOverrideActiveChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn DisplayEnhancementOverrideCapabilitiesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveDisplayEnhancementOverrideCapabilitiesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<DisplayEnhancementOverride> {
        Self::IDisplayEnhancementOverrideStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayEnhancementOverride>(result__)
        })
    }
    pub fn IDisplayEnhancementOverrideStatics<R, F: FnOnce(&IDisplayEnhancementOverrideStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DisplayEnhancementOverride, IDisplayEnhancementOverrideStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayEnhancementOverride {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverride;{429594cf-d97a-4b02-a428-5c4292f7f522})");
}
unsafe impl ::windows::runtime::Interface for DisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x429594cf_d97a_4b02_a428_5c4292f7f522);
}
impl ::windows::runtime::RuntimeName for DisplayEnhancementOverride {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverride";
}
impl ::core::convert::From<DisplayEnhancementOverride> for ::windows::runtime::IUnknown {
    fn from(value: DisplayEnhancementOverride) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayEnhancementOverride> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayEnhancementOverride) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayEnhancementOverride> for ::windows::runtime::IInspectable {
    fn from(value: DisplayEnhancementOverride) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayEnhancementOverride> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayEnhancementOverride) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverride {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverride {}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayEnhancementOverrideCapabilities(pub ::windows::runtime::IInspectable);
impl DisplayEnhancementOverrideCapabilities {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsBrightnessControlSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn IsBrightnessNitsControlSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation_Collections`*"]
    pub fn GetSupportedNitRanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<NitRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<NitRange>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayEnhancementOverrideCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities;{457060de-ee5a-47b7-9918-1e51e812ccc8})");
}
unsafe impl ::windows::runtime::Interface for DisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x457060de_ee5a_47b7_9918_1e51e812ccc8);
}
impl ::windows::runtime::RuntimeName for DisplayEnhancementOverrideCapabilities {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities";
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: DisplayEnhancementOverrideCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayEnhancementOverrideCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: DisplayEnhancementOverrideCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayEnhancementOverrideCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverrideCapabilities {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverrideCapabilities {}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub ::windows::runtime::IInspectable);
impl DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayEnhancementOverrideCapabilities>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs;{db61e664-15fa-49da-8b77-07dbd2af585d})");
}
unsafe impl ::windows::runtime::Interface for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb61e664_15fa_49da_8b77_07dbd2af585d);
}
impl ::windows::runtime::RuntimeName for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs";
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayInformation(pub ::windows::runtime::IInspectable);
impl DisplayInformation {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CurrentOrientation(&self) -> ::windows::runtime::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn NativeOrientation(&self) -> ::windows::runtime::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn OrientationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn ResolutionScale(&self) -> ::windows::runtime::Result<ResolutionScale> {
        let this = self;
        unsafe {
            let mut result__: ResolutionScale = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResolutionScale>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn LogicalDpi(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn RawDpiX(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn RawDpiY(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn DpiChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveDpiChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn StereoEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn StereoEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveStereoEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`, `Storage_Streams`*"]
    pub fn GetColorProfileAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn ColorProfileChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveColorProfileChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn RawPixelsPerViewPixel(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<DisplayInformation> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayInformation>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn AutoRotationPreferences() -> ::windows::runtime::Result<DisplayOrientations> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> ::windows::runtime::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn DisplayContentsInvalidated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveDisplayContentsInvalidated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn DiagonalSizeInInches(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn ScreenWidthInRawPixels(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn ScreenHeightInRawPixels(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn GetAdvancedColorInfo(&self) -> ::windows::runtime::Result<AdvancedColorInfo> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedColorInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn AdvancedColorInfoChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveAdvancedColorInfoChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IDisplayInformationStatics<R, F: FnOnce(&IDisplayInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DisplayInformation, IDisplayInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayInformation;{bed112ae-adc3-4dc9-ae65-851f4d7d4799})");
}
unsafe impl ::windows::runtime::Interface for DisplayInformation {
    type Vtable = IDisplayInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbed112ae_adc3_4dc9_ae65_851f4d7d4799);
}
impl ::windows::runtime::RuntimeName for DisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayInformation";
}
impl ::core::convert::From<DisplayInformation> for ::windows::runtime::IUnknown {
    fn from(value: DisplayInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayInformation> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DisplayInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayInformation> for ::windows::runtime::IInspectable {
    fn from(value: DisplayInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayInformation> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayInformation {}
unsafe impl ::core::marker::Sync for DisplayInformation {}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: DisplayOrientations = DisplayOrientations(0u32);
    pub const Landscape: DisplayOrientations = DisplayOrientations(1u32);
    pub const Portrait: DisplayOrientations = DisplayOrientations(2u32);
    pub const LandscapeFlipped: DisplayOrientations = DisplayOrientations(4u32);
    pub const PortraitFlipped: DisplayOrientations = DisplayOrientations(8u32);
}
impl ::core::convert::From<u32> for DisplayOrientations {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayOrientations {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayOrientations {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayOrientations;u4)");
}
impl ::windows::runtime::DefaultType for DisplayOrientations {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayOrientations {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayOrientations {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayOrientations {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayOrientations {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayOrientations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Graphics_Display`*"]
pub struct DisplayProperties {}
impl DisplayProperties {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn CurrentOrientation() -> ::windows::runtime::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn NativeOrientation() -> ::windows::runtime::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn AutoRotationPreferences() -> ::windows::runtime::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> ::windows::runtime::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn OrientationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn ResolutionScale() -> ::windows::runtime::Result<ResolutionScale> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: ResolutionScale = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResolutionScale>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn LogicalDpi() -> ::windows::runtime::Result<f32> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn LogicalDpiChanged<'a, Param0: ::windows::runtime::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveLogicalDpiChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn StereoEnabled() -> ::windows::runtime::Result<bool> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn StereoEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveStereoEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`, `Storage_Streams`*"]
    pub fn GetColorProfileAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn ColorProfileChanged<'a, Param0: ::windows::runtime::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveColorProfileChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn DisplayContentsInvalidated<'a, Param0: ::windows::runtime::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display`, `Foundation`*"]
    pub fn RemoveDisplayContentsInvalidated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IDisplayPropertiesStatics<R, F: FnOnce(&IDisplayPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DisplayProperties, IDisplayPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for DisplayProperties {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayProperties";
}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayPropertiesEventHandler(::windows::runtime::IUnknown);
impl DisplayPropertiesEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DisplayPropertiesEventHandler_box::<F> {
            vtable: &DisplayPropertiesEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayPropertiesEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({dbdd8b01-f1a1-46d1-9ee3-543bcc995980})");
}
unsafe impl ::windows::runtime::Interface for DisplayPropertiesEventHandler {
    type Vtable = DisplayPropertiesEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdbdd8b01_f1a1_46d1_9ee3_543bcc995980);
}
#[repr(C)]
#[doc(hidden)]
pub struct DisplayPropertiesEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DisplayPropertiesEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DisplayPropertiesEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static> DisplayPropertiesEventHandler_box<F> {
    const VTABLE: DisplayPropertiesEventHandler_abi = DisplayPropertiesEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DisplayPropertiesEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Graphics_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayServices(pub ::windows::runtime::IInspectable);
impl DisplayServices {
    #[doc = "*Required features: `Graphics_Display`*"]
    pub fn FindAll() -> ::windows::runtime::Result<::windows::runtime::Array<super::DisplayId>> {
        Self::IDisplayServicesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::Array<super::DisplayId> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::runtime::Array::<super::DisplayId>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        })
    }
    pub fn IDisplayServicesStatics<R, F: FnOnce(&IDisplayServicesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DisplayServices, IDisplayServicesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayServices {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayServices;{1b54f32b-890d-5747-bd26-fdbdeb0c8a71})");
}
unsafe impl ::windows::runtime::Interface for DisplayServices {
    type Vtable = IDisplayServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b54f32b_890d_5747_bd26_fdbdeb0c8a71);
}
impl ::windows::runtime::RuntimeName for DisplayServices {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayServices";
}
impl ::core::convert::From<DisplayServices> for ::windows::runtime::IUnknown {
    fn from(value: DisplayServices) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayServices> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayServices) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DisplayServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayServices> for ::windows::runtime::IInspectable {
    fn from(value: DisplayServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayServices> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayServices {}
unsafe impl ::core::marker::Sync for DisplayServices {}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: HdrMetadataFormat = HdrMetadataFormat(0i32);
    pub const Hdr10Plus: HdrMetadataFormat = HdrMetadataFormat(1i32);
}
impl ::core::convert::From<i32> for HdrMetadataFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HdrMetadataFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdrMetadataFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.HdrMetadataFormat;i4)");
}
impl ::windows::runtime::DefaultType for HdrMetadataFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedColorInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8797dcfb_b229_4081_ae9a_2cc85e34ad6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedColorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdvancedColorKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: HdrMetadataFormat, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: AdvancedColorKind, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrightnessOverride(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBrightnessOverride {
    type Vtable = IBrightnessOverride_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96c9621a_c143_4392_bedd_4a7e9574c8fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverride_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scenario: DisplayBrightnessScenario, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd112ab2a_7604_4dba_bcf8_4b6f49502cb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettingsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBrightnessOverrideSettingsStatics {
    type Vtable = IBrightnessOverrideSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd487dc90_6f74_440b_b383_5fe96cf00b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nits: f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBrightnessOverrideStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBrightnessOverrideStatics {
    type Vtable = IBrightnessOverrideStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x03a7b9ed_e1f1_4a68_a11f_946ad8ce5393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorOverrideSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorOverrideSettings {
    type Vtable = IColorOverrideSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfbefa134_4a81_4c4d_a5b6_7d1b5c4bd00b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorOverrideSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayColorOverrideScenario) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorOverrideSettingsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorOverrideSettingsStatics {
    type Vtable = IColorOverrideSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb068e05f_c41f_4ac9_afab_827ab6248f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorOverrideSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overridescenario: DisplayColorOverrideScenario, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverride(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x429594cf_d97a_4b02_a428_5c4292f7f522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverride_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilities(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x457060de_ee5a_47b7_9918_1e51e812ccc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb61e664_15fa_49da_8b77_07dbd2af585d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_abi(
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
pub struct IDisplayEnhancementOverrideStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayEnhancementOverrideStatics {
    type Vtable = IDisplayEnhancementOverrideStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcf5b7ec1_9791_4453_b013_29b6f778e519);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideStatics_abi(
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
pub struct IDisplayInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayInformation {
    type Vtable = IDisplayInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbed112ae_adc3_4dc9_ae65_851f4d7d4799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayOrientations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayOrientations) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ResolutionScale) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayInformation2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayInformation2 {
    type Vtable = IDisplayInformation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4dcd0021_fad1_4b8e_8edf_775887b8bf19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayInformation3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayInformation3 {
    type Vtable = IDisplayInformation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb15011d_0f09_4466_8ff3_11de9a3c929a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayInformation4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayInformation4 {
    type Vtable = IDisplayInformation4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc972ce2f_1242_46be_b536_e1aafe9e7acf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayInformation5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayInformation5 {
    type Vtable = IDisplayInformation5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3a5442dc_2cde_4a8d_80d1_21dc5adcc1aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayInformationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayInformationStatics {
    type Vtable = IDisplayInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc6a02a6c_d452_44dc_ba07_96f3c6adf9d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayOrientations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DisplayOrientations) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayPropertiesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayPropertiesStatics {
    type Vtable = IDisplayPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6937ed8d_30ea_4ded_8271_4553ff02f68a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayOrientations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayOrientations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayOrientations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DisplayOrientations) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ResolutionScale) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayServices(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayServices {
    type Vtable = IDisplayServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b54f32b_890d_5747_bd26_fdbdeb0c8a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayServicesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayServicesStatics {
    type Vtable = IDisplayServicesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc2096bf_730a_5560_b461_91c13d692e0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayServicesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_Display`*"]
pub struct NitRange {
    pub MinNits: f32,
    pub MaxNits: f32,
    pub StepSizeNits: f32,
}
impl NitRange {}
impl ::core::default::Default for NitRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NitRange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NitRange").field("MinNits", &self.MinNits).field("MaxNits", &self.MaxNits).field("StepSizeNits", &self.StepSizeNits).finish()
    }
}
impl ::core::cmp::PartialEq for NitRange {
    fn eq(&self, other: &Self) -> bool {
        self.MinNits == other.MinNits && self.MaxNits == other.MaxNits && self.StepSizeNits == other.StepSizeNits
    }
}
impl ::core::cmp::Eq for NitRange {}
unsafe impl ::windows::runtime::Abi for NitRange {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NitRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.NitRange;f4;f4;f4)");
}
impl ::windows::runtime::DefaultType for NitRange {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: ResolutionScale = ResolutionScale(0i32);
    pub const Scale100Percent: ResolutionScale = ResolutionScale(100i32);
    pub const Scale120Percent: ResolutionScale = ResolutionScale(120i32);
    pub const Scale125Percent: ResolutionScale = ResolutionScale(125i32);
    pub const Scale140Percent: ResolutionScale = ResolutionScale(140i32);
    pub const Scale150Percent: ResolutionScale = ResolutionScale(150i32);
    pub const Scale160Percent: ResolutionScale = ResolutionScale(160i32);
    pub const Scale175Percent: ResolutionScale = ResolutionScale(175i32);
    pub const Scale180Percent: ResolutionScale = ResolutionScale(180i32);
    pub const Scale200Percent: ResolutionScale = ResolutionScale(200i32);
    pub const Scale225Percent: ResolutionScale = ResolutionScale(225i32);
    pub const Scale250Percent: ResolutionScale = ResolutionScale(250i32);
    pub const Scale300Percent: ResolutionScale = ResolutionScale(300i32);
    pub const Scale350Percent: ResolutionScale = ResolutionScale(350i32);
    pub const Scale400Percent: ResolutionScale = ResolutionScale(400i32);
    pub const Scale450Percent: ResolutionScale = ResolutionScale(450i32);
    pub const Scale500Percent: ResolutionScale = ResolutionScale(500i32);
}
impl ::core::convert::From<i32> for ResolutionScale {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ResolutionScale {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ResolutionScale {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.ResolutionScale;i4)");
}
impl ::windows::runtime::DefaultType for ResolutionScale {
    type DefaultType = Self;
}
