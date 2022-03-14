#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct AdvancedColorInfo(::windows::core::IUnknown);
impl AdvancedColorInfo {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CurrentAdvancedColorKind(&self) -> ::windows::core::Result<AdvancedColorKind> {
        let this = self;
        unsafe {
            let mut result__: AdvancedColorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentAdvancedColorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedColorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RedPrimary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GreenPrimary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluePrimary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WhitePoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxLuminanceInNits)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinLuminanceInNits)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxAverageFullFrameLuminanceInNits)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn SdrWhiteLevelInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SdrWhiteLevelInNits)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHdrMetadataFormatCurrentlySupported)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAdvancedColorKindAvailable)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvancedColorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvancedColorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedColorInfo {}
impl ::core::fmt::Debug for AdvancedColorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedColorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedColorInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.AdvancedColorInfo;{8797dcfb-b229-4081-ae9a-2cc85e34ad6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAdvancedColorInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdvancedColorInfo {
    const NAME: &'static str = "Windows.Graphics.Display.AdvancedColorInfo";
}
impl ::core::convert::From<AdvancedColorInfo> for ::windows::core::IUnknown {
    fn from(value: AdvancedColorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedColorInfo> for ::windows::core::IUnknown {
    fn from(value: &AdvancedColorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedColorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedColorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvancedColorInfo> for ::windows::core::IInspectable {
    fn from(value: AdvancedColorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedColorInfo> for ::windows::core::IInspectable {
    fn from(value: &AdvancedColorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedColorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedColorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvancedColorInfo {}
unsafe impl ::core::marker::Sync for AdvancedColorInfo {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
}
impl ::core::marker::Copy for AdvancedColorKind {}
impl ::core::clone::Clone for AdvancedColorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdvancedColorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdvancedColorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdvancedColorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedColorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedColorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.AdvancedColorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct BrightnessOverride(::windows::core::IUnknown);
impl BrightnessOverride {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsOverrideActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOverrideActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn BrightnessLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrightnessLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn SetBrightnessLevel(&self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBrightnessLevel)(::core::mem::transmute_copy(this), brightnesslevel, options).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBrightnessScenario)(::core::mem::transmute_copy(this), scenario, options).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLevelForScenario)(::core::mem::transmute_copy(this), scenario, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn StartOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartOverride)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn StopOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopOverride)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupportedChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsSupportedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsSupportedChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsOverrideActiveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOverrideActiveChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsOverrideActiveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsOverrideActiveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BrightnessLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrightnessLevelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBrightnessLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBrightnessLevelChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetDefaultForSystem() -> ::windows::core::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultForSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrightnessOverride>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrightnessOverride>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveForSystemAsync<'a, Param0: ::windows::core::IntoParam<'a, BrightnessOverride>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveForSystemAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBrightnessOverrideStatics<R, F: FnOnce(&IBrightnessOverrideStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BrightnessOverride, IBrightnessOverrideStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BrightnessOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BrightnessOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BrightnessOverride {}
impl ::core::fmt::Debug for BrightnessOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrightnessOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BrightnessOverride {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.BrightnessOverride;{96c9621a-c143-4392-bedd-4a7e9574c8fd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BrightnessOverride {
    type Vtable = IBrightnessOverride_Vtbl;
    const IID: ::windows::core::GUID = <IBrightnessOverride as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BrightnessOverride {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverride";
}
impl ::core::convert::From<BrightnessOverride> for ::windows::core::IUnknown {
    fn from(value: BrightnessOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverride> for ::windows::core::IUnknown {
    fn from(value: &BrightnessOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BrightnessOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BrightnessOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BrightnessOverride> for ::windows::core::IInspectable {
    fn from(value: BrightnessOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverride> for ::windows::core::IInspectable {
    fn from(value: &BrightnessOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BrightnessOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BrightnessOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BrightnessOverride {}
unsafe impl ::core::marker::Sync for BrightnessOverride {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct BrightnessOverrideSettings(::windows::core::IUnknown);
impl BrightnessOverrideSettings {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn DesiredLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn DesiredNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredNits)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CreateFromLevel(level: f64) -> ::windows::core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLevel)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CreateFromNits(nits: f32) -> ::windows::core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromNits)(::core::mem::transmute_copy(this), nits, &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CreateFromDisplayBrightnessOverrideScenario(overridescenario: DisplayBrightnessOverrideScenario) -> ::windows::core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromDisplayBrightnessOverrideScenario)(::core::mem::transmute_copy(this), overridescenario, &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBrightnessOverrideSettingsStatics<R, F: FnOnce(&IBrightnessOverrideSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BrightnessOverrideSettings, IBrightnessOverrideSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BrightnessOverrideSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BrightnessOverrideSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BrightnessOverrideSettings {}
impl ::core::fmt::Debug for BrightnessOverrideSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrightnessOverrideSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BrightnessOverrideSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.BrightnessOverrideSettings;{d112ab2a-7604-4dba-bcf8-4b6f49502cb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_Vtbl;
    const IID: ::windows::core::GUID = <IBrightnessOverrideSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BrightnessOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverrideSettings";
}
impl ::core::convert::From<BrightnessOverrideSettings> for ::windows::core::IUnknown {
    fn from(value: BrightnessOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverrideSettings> for ::windows::core::IUnknown {
    fn from(value: &BrightnessOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BrightnessOverrideSettings> for ::windows::core::IInspectable {
    fn from(value: BrightnessOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverrideSettings> for ::windows::core::IInspectable {
    fn from(value: &BrightnessOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BrightnessOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BrightnessOverrideSettings {}
unsafe impl ::core::marker::Sync for BrightnessOverrideSettings {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct ColorOverrideSettings(::windows::core::IUnknown);
impl ColorOverrideSettings {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn DesiredDisplayColorOverrideScenario(&self) -> ::windows::core::Result<DisplayColorOverrideScenario> {
        let this = self;
        unsafe {
            let mut result__: DisplayColorOverrideScenario = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredDisplayColorOverrideScenario)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayColorOverrideScenario>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CreateFromDisplayColorOverrideScenario(overridescenario: DisplayColorOverrideScenario) -> ::windows::core::Result<ColorOverrideSettings> {
        Self::IColorOverrideSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromDisplayColorOverrideScenario)(::core::mem::transmute_copy(this), overridescenario, &mut result__).from_abi::<ColorOverrideSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorOverrideSettingsStatics<R, F: FnOnce(&IColorOverrideSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ColorOverrideSettings, IColorOverrideSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorOverrideSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorOverrideSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorOverrideSettings {}
impl ::core::fmt::Debug for ColorOverrideSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorOverrideSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorOverrideSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.ColorOverrideSettings;{fbefa134-4a81-4c4d-a5b6-7d1b5c4bd00b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ColorOverrideSettings {
    type Vtable = IColorOverrideSettings_Vtbl;
    const IID: ::windows::core::GUID = <IColorOverrideSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ColorOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.ColorOverrideSettings";
}
impl ::core::convert::From<ColorOverrideSettings> for ::windows::core::IUnknown {
    fn from(value: ColorOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorOverrideSettings> for ::windows::core::IUnknown {
    fn from(value: &ColorOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ColorOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ColorOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorOverrideSettings> for ::windows::core::IInspectable {
    fn from(value: ColorOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorOverrideSettings> for ::windows::core::IInspectable {
    fn from(value: &ColorOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ColorOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ColorOverrideSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ColorOverrideSettings {}
unsafe impl ::core::marker::Sync for ColorOverrideSettings {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: Self = Self(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideOptions {}
impl ::core::clone::Clone for DisplayBrightnessOverrideOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBrightnessOverrideOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayBrightnessOverrideOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBrightnessOverrideOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBrightnessOverrideOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayBrightnessOverrideOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayBrightnessOverrideOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayBrightnessOverrideOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: Self = Self(0i32);
    pub const BarcodeReadingBrightness: Self = Self(1i32);
    pub const FullBrightness: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideScenario {}
impl ::core::clone::Clone for DisplayBrightnessOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBrightnessOverrideScenario {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayBrightnessOverrideScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBrightnessOverrideScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBrightnessOverrideScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayBrightnessOverrideScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: Self = Self(0i32);
    pub const IdleBrightness: Self = Self(1i32);
    pub const BarcodeReadingBrightness: Self = Self(2i32);
    pub const FullBrightness: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayBrightnessScenario {}
impl ::core::clone::Clone for DisplayBrightnessScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBrightnessScenario {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayBrightnessScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBrightnessScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBrightnessScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayBrightnessScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayColorOverrideScenario {}
impl ::core::clone::Clone for DisplayColorOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayColorOverrideScenario {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayColorOverrideScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayColorOverrideScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayColorOverrideScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayColorOverrideScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayColorOverrideScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayEnhancementOverride(::windows::core::IUnknown);
impl DisplayEnhancementOverride {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn ColorOverrideSettings(&self) -> ::windows::core::Result<ColorOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorOverrideSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ColorOverrideSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn SetColorOverrideSettings<'a, Param0: ::windows::core::IntoParam<'a, ColorOverrideSettings>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorOverrideSettings)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn BrightnessOverrideSettings(&self) -> ::windows::core::Result<BrightnessOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BrightnessOverrideSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BrightnessOverrideSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn SetBrightnessOverrideSettings<'a, Param0: ::windows::core::IntoParam<'a, BrightnessOverrideSettings>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBrightnessOverrideSettings)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CanOverride(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanOverride)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsOverrideActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOverrideActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetCurrentDisplayEnhancementOverrideCapabilities(&self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentDisplayEnhancementOverrideCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayEnhancementOverrideCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn RequestOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestOverride)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn StopOverride(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopOverride)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanOverrideChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanOverrideChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanOverrideChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanOverrideChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsOverrideActiveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOverrideActiveChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsOverrideActiveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsOverrideActiveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayEnhancementOverrideCapabilitiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayEnhancementOverrideCapabilitiesChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisplayEnhancementOverrideCapabilitiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDisplayEnhancementOverrideCapabilitiesChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<DisplayEnhancementOverride> {
        Self::IDisplayEnhancementOverrideStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayEnhancementOverride>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayEnhancementOverrideStatics<R, F: FnOnce(&IDisplayEnhancementOverrideStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayEnhancementOverride, IDisplayEnhancementOverrideStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayEnhancementOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayEnhancementOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayEnhancementOverride {}
impl ::core::fmt::Debug for DisplayEnhancementOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayEnhancementOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayEnhancementOverride {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverride;{429594cf-d97a-4b02-a428-5c4292f7f522})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayEnhancementOverride as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayEnhancementOverride {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverride";
}
impl ::core::convert::From<DisplayEnhancementOverride> for ::windows::core::IUnknown {
    fn from(value: DisplayEnhancementOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverride> for ::windows::core::IUnknown {
    fn from(value: &DisplayEnhancementOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayEnhancementOverride> for ::windows::core::IInspectable {
    fn from(value: DisplayEnhancementOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverride> for ::windows::core::IInspectable {
    fn from(value: &DisplayEnhancementOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayEnhancementOverride {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverride {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverride {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilities(::windows::core::IUnknown);
impl DisplayEnhancementOverrideCapabilities {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsBrightnessControlSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBrightnessControlSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn IsBrightnessNitsControlSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBrightnessNitsControlSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedNitRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NitRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedNitRanges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<NitRange>>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayEnhancementOverrideCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayEnhancementOverrideCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayEnhancementOverrideCapabilities {}
impl ::core::fmt::Debug for DisplayEnhancementOverrideCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayEnhancementOverrideCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayEnhancementOverrideCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities;{457060de-ee5a-47b7-9918-1e51e812ccc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayEnhancementOverrideCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayEnhancementOverrideCapabilities {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities";
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilities> for ::windows::core::IUnknown {
    fn from(value: DisplayEnhancementOverrideCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilities> for ::windows::core::IUnknown {
    fn from(value: &DisplayEnhancementOverrideCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilities> for ::windows::core::IInspectable {
    fn from(value: DisplayEnhancementOverrideCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilities> for ::windows::core::IInspectable {
    fn from(value: &DisplayEnhancementOverrideCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverrideCapabilities {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverrideCapabilities {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(::windows::core::IUnknown);
impl DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayEnhancementOverrideCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
impl ::core::fmt::Debug for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayEnhancementOverrideCapabilitiesChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs;{db61e664-15fa-49da-8b77-07dbd2af585d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayEnhancementOverrideCapabilitiesChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs";
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayInformation(::windows::core::IUnknown);
impl DisplayInformation {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn CurrentOrientation(&self) -> ::windows::core::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentOrientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn NativeOrientation(&self) -> ::windows::core::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NativeOrientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OrientationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OrientationChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOrientationChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn ResolutionScale(&self) -> ::windows::core::Result<ResolutionScale> {
        let this = self;
        unsafe {
            let mut result__: ResolutionScale = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResolutionScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResolutionScale>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn LogicalDpi(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalDpi)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn RawDpiX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawDpiX)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn RawDpiY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawDpiY)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DpiChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DpiChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDpiChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDpiChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn StereoEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StereoEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoEnabledChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStereoEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStereoEnabledChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetColorProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetColorProfileAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ColorProfileChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorProfileChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveColorProfileChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveColorProfileChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn RawPixelsPerViewPixel(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawPixelsPerViewPixel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DiagonalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiagonalSizeInInches)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn ScreenWidthInRawPixels(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenWidthInRawPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn ScreenHeightInRawPixels(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenHeightInRawPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetAdvancedColorInfo(&self) -> ::windows::core::Result<AdvancedColorInfo> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAdvancedColorInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedColorInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AdvancedColorInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdvancedColorInfoChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdvancedColorInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdvancedColorInfoChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<DisplayInformation> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayInformation>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn AutoRotationPreferences() -> ::windows::core::Result<DisplayOrientations> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoRotationPreferences)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> ::windows::core::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetAutoRotationPreferences)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayContentsInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayContentsInvalidated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisplayContentsInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveDisplayContentsInvalidated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IDisplayInformationStatics<R, F: FnOnce(&IDisplayInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayInformation, IDisplayInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayInformation {}
impl ::core::fmt::Debug for DisplayInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayInformation;{bed112ae-adc3-4dc9-ae65-851f4d7d4799})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayInformation";
}
impl ::core::convert::From<DisplayInformation> for ::windows::core::IUnknown {
    fn from(value: DisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayInformation> for ::windows::core::IUnknown {
    fn from(value: &DisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayInformation> for ::windows::core::IInspectable {
    fn from(value: DisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayInformation> for ::windows::core::IInspectable {
    fn from(value: &DisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayInformation {}
unsafe impl ::core::marker::Sync for DisplayInformation {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: Self = Self(0u32);
    pub const Landscape: Self = Self(1u32);
    pub const Portrait: Self = Self(2u32);
    pub const LandscapeFlipped: Self = Self(4u32);
    pub const PortraitFlipped: Self = Self(8u32);
}
impl ::core::marker::Copy for DisplayOrientations {}
impl ::core::clone::Clone for DisplayOrientations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayOrientations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayOrientations {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayOrientations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayOrientations").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayOrientations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayOrientations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayOrientations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayOrientations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayOrientations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayOrientations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayOrientations;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct DisplayProperties {}
#[cfg(feature = "deprecated")]
impl DisplayProperties {
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentOrientation() -> ::windows::core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentOrientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NativeOrientation() -> ::windows::core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NativeOrientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AutoRotationPreferences() -> ::windows::core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: DisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoRotationPreferences)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> ::windows::core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetAutoRotationPreferences)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn OrientationChanged<'a, Param0: ::windows::core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OrientationChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveOrientationChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ResolutionScale() -> ::windows::core::Result<ResolutionScale> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: ResolutionScale = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResolutionScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResolutionScale>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LogicalDpi() -> ::windows::core::Result<f32> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalDpi)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn LogicalDpiChanged<'a, Param0: ::windows::core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalDpiChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveLogicalDpiChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveLogicalDpiChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn StereoEnabled() -> ::windows::core::Result<bool> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StereoEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoEnabledChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStereoEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveStereoEnabledChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn GetColorProfileAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetColorProfileAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ColorProfileChanged<'a, Param0: ::windows::core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorProfileChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveColorProfileChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveColorProfileChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DisplayContentsInvalidated<'a, Param0: ::windows::core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayContentsInvalidated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveDisplayContentsInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveDisplayContentsInvalidated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IDisplayPropertiesStatics<R, F: FnOnce(&IDisplayPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayProperties, IDisplayPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for DisplayProperties {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayProperties";
}
#[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct DisplayPropertiesEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl DisplayPropertiesEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DisplayPropertiesEventHandlerBox::<F> { vtable: &DisplayPropertiesEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct DisplayPropertiesEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DisplayPropertiesEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DisplayPropertiesEventHandlerBox<F> {
    const VTABLE: DisplayPropertiesEventHandler_Vtbl = DisplayPropertiesEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DisplayPropertiesEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for DisplayPropertiesEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for DisplayPropertiesEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for DisplayPropertiesEventHandler {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for DisplayPropertiesEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPropertiesEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for DisplayPropertiesEventHandler {
    type Vtable = DisplayPropertiesEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbdd8b01_f1a1_46d1_9ee3_543bcc995980);
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for DisplayPropertiesEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dbdd8b01-f1a1-46d1-9ee3-543bcc995980}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct DisplayPropertiesEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayServices(::windows::core::IUnknown);
impl DisplayServices {
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    pub fn FindAll() -> ::windows::core::Result<::windows::core::Array<super::DisplayId>> {
        Self::IDisplayServicesStatics(|this| unsafe {
            let mut result__: ::windows::core::Array<super::DisplayId> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAll)(::core::mem::transmute_copy(this), ::windows::core::Array::<super::DisplayId>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayServicesStatics<R, F: FnOnce(&IDisplayServicesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayServices, IDisplayServicesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayServices {}
impl ::core::fmt::Debug for DisplayServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayServices;{1b54f32b-890d-5747-bd26-fdbdeb0c8a71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayServices {
    type Vtable = IDisplayServices_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayServices as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayServices {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayServices";
}
impl ::core::convert::From<DisplayServices> for ::windows::core::IUnknown {
    fn from(value: DisplayServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayServices> for ::windows::core::IUnknown {
    fn from(value: &DisplayServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayServices> for ::windows::core::IInspectable {
    fn from(value: DisplayServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayServices> for ::windows::core::IInspectable {
    fn from(value: &DisplayServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayServices {}
unsafe impl ::core::marker::Sync for DisplayServices {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
}
impl ::core::marker::Copy for HdrMetadataFormat {}
impl ::core::clone::Clone for HdrMetadataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdrMetadataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdrMetadataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdrMetadataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdrMetadataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdrMetadataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.HdrMetadataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedColorInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8797dcfb_b229_4081_ae9a_2cc85e34ad6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedColorInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentAdvancedColorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdvancedColorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RedPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedPrimary: usize,
    #[cfg(feature = "Foundation")]
    pub GreenPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GreenPrimary: usize,
    #[cfg(feature = "Foundation")]
    pub BluePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BluePrimary: usize,
    #[cfg(feature = "Foundation")]
    pub WhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhitePoint: usize,
    pub MaxLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SdrWhiteLevelInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub IsHdrMetadataFormatCurrentlySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: HdrMetadataFormat, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAdvancedColorKindAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: AdvancedColorKind, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverride(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBrightnessOverride {
    type Vtable = IBrightnessOverride_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96c9621a_c143_4392_bedd_4a7e9574c8fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverride_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::HRESULT,
    pub SetBrightnessScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::HRESULT,
    pub GetLevelForScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, result__: *mut f64) -> ::windows::core::HRESULT,
    pub StartOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BrightnessLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BrightnessLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBrightnessLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBrightnessLevelChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverrideSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd112ab2a_7604_4dba_bcf8_4b6f49502cb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesiredLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub DesiredNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverrideSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBrightnessOverrideSettingsStatics {
    type Vtable = IBrightnessOverrideSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd487dc90_6f74_440b_b383_5fe96cf00b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFromNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nits: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFromDisplayBrightnessOverrideScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverrideStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBrightnessOverrideStatics {
    type Vtable = IBrightnessOverrideStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03a7b9ed_e1f1_4a68_a11f_946ad8ce5393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefaultForSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveForSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveForSystemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorOverrideSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorOverrideSettings {
    type Vtable = IColorOverrideSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbefa134_4a81_4c4d_a5b6_7d1b5c4bd00b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorOverrideSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesiredDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayColorOverrideScenario) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorOverrideSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorOverrideSettingsStatics {
    type Vtable = IColorOverrideSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb068e05f_c41f_4ac9_afab_827ab6248f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorOverrideSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overridescenario: DisplayColorOverrideScenario, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverride(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x429594cf_d97a_4b02_a428_5c4292f7f522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverride_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ColorOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetColorOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BrightnessOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBrightnessOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CanOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetCurrentDisplayEnhancementOverrideCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RequestOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CanOverrideChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanOverrideChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanOverrideChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanOverrideChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayEnhancementOverrideCapabilitiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayEnhancementOverrideCapabilitiesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x457060de_ee5a_47b7_9918_1e51e812ccc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBrightnessControlSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBrightnessNitsControlSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedNitRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedNitRanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb61e664_15fa_49da_8b77_07dbd2af585d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayEnhancementOverrideStatics {
    type Vtable = IDisplayEnhancementOverrideStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf5b7ec1_9791_4453_b013_29b6f778e519);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed112ae_adc3_4dc9_ae65_851f4d7d4799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT,
    pub NativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OrientationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOrientationChanged: usize,
    pub ResolutionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows::core::HRESULT,
    pub LogicalDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub RawDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DpiChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDpiChanged: usize,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StereoEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStereoEnabledChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetColorProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetColorProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ColorProfileChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveColorProfileChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayInformation2 {
    type Vtable = IDisplayInformation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dcd0021_fad1_4b8e_8edf_775887b8bf19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RawPixelsPerViewPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayInformation3 {
    type Vtable = IDisplayInformation3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb15011d_0f09_4466_8ff3_11de9a3c929a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DiagonalSizeInInches: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiagonalSizeInInches: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayInformation4 {
    type Vtable = IDisplayInformation4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc972ce2f_1242_46be_b536_e1aafe9e7acf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ScreenWidthInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ScreenHeightInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayInformation5 {
    type Vtable = IDisplayInformation5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a5442dc_2cde_4a8d_80d1_21dc5adcc1aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetAdvancedColorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AdvancedColorInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvancedColorInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvancedColorInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvancedColorInfoChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayInformationStatics {
    type Vtable = IDisplayInformationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a02a6c_d452_44dc_ba07_96f3c6adf9d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT,
    pub SetAutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayContentsInvalidated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayContentsInvalidated: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDisplayPropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IDisplayPropertiesStatics {
    type Vtable = IDisplayPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6937ed8d_30ea_4ded_8271_4553ff02f68a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub NativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NativeOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub AutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoRotationPreferences: usize,
    #[cfg(feature = "deprecated")]
    pub SetAutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAutoRotationPreferences: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    OrientationChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveOrientationChanged: usize,
    #[cfg(feature = "deprecated")]
    pub ResolutionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResolutionScale: usize,
    #[cfg(feature = "deprecated")]
    pub LogicalDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogicalDpi: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LogicalDpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LogicalDpiChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLogicalDpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLogicalDpiChanged: usize,
    #[cfg(feature = "deprecated")]
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StereoEnabled: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StereoEnabledChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStereoEnabledChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub GetColorProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    GetColorProfileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ColorProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveColorProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DisplayContentsInvalidated: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveDisplayContentsInvalidated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayServices(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayServices {
    type Vtable = IDisplayServices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b54f32b_890d_5747_bd26_fdbdeb0c8a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayServices_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayServicesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayServicesStatics {
    type Vtable = IDisplayServicesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2096bf_730a_5560_b461_91c13d692e0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayServicesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Display\"`*"]
pub struct NitRange {
    pub MinNits: f32,
    pub MaxNits: f32,
    pub StepSizeNits: f32,
}
impl ::core::marker::Copy for NitRange {}
impl ::core::clone::Clone for NitRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NitRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NitRange").field("MinNits", &self.MinNits).field("MaxNits", &self.MaxNits).field("StepSizeNits", &self.StepSizeNits).finish()
    }
}
unsafe impl ::windows::core::Abi for NitRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NitRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.NitRange;f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for NitRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NitRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for NitRange {}
impl ::core::default::Default for NitRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: Self = Self(0i32);
    pub const Scale100Percent: Self = Self(100i32);
    pub const Scale120Percent: Self = Self(120i32);
    pub const Scale125Percent: Self = Self(125i32);
    pub const Scale140Percent: Self = Self(140i32);
    pub const Scale150Percent: Self = Self(150i32);
    pub const Scale160Percent: Self = Self(160i32);
    pub const Scale175Percent: Self = Self(175i32);
    pub const Scale180Percent: Self = Self(180i32);
    pub const Scale200Percent: Self = Self(200i32);
    pub const Scale225Percent: Self = Self(225i32);
    pub const Scale250Percent: Self = Self(250i32);
    pub const Scale300Percent: Self = Self(300i32);
    pub const Scale350Percent: Self = Self(350i32);
    pub const Scale400Percent: Self = Self(400i32);
    pub const Scale450Percent: Self = Self(450i32);
    pub const Scale500Percent: Self = Self(500i32);
}
impl ::core::marker::Copy for ResolutionScale {}
impl ::core::clone::Clone for ResolutionScale {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResolutionScale {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ResolutionScale {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResolutionScale {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResolutionScale").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResolutionScale {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.ResolutionScale;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
