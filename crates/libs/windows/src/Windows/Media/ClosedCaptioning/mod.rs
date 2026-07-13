#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: Self = Self(0);
    pub const White: Self = Self(1);
    pub const Black: Self = Self(2);
    pub const Red: Self = Self(3);
    pub const Green: Self = Self(4);
    pub const Blue: Self = Self(5);
    pub const Yellow: Self = Self(6);
    pub const Magenta: Self = Self(7);
    pub const Cyan: Self = Self(8);
}
impl windows_core::TypeKind for ClosedCaptionColor {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.ClosedCaptionColor");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: Self = Self(0);
    pub const None: Self = Self(1);
    pub const Raised: Self = Self(2);
    pub const Depressed: Self = Self(3);
    pub const Uniform: Self = Self(4);
    pub const DropShadow: Self = Self(5);
}
impl windows_core::TypeKind for ClosedCaptionEdgeEffect {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: Self = Self(0);
    pub const OneHundredPercent: Self = Self(1);
    pub const SeventyFivePercent: Self = Self(2);
    pub const TwentyFivePercent: Self = Self(3);
    pub const ZeroPercent: Self = Self(4);
}
impl windows_core::TypeKind for ClosedCaptionOpacity {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.ClosedCaptionOpacity");
}
pub struct ClosedCaptionProperties;
impl ClosedCaptionProperties {
    pub fn FontColor() -> windows_core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedFontColor() -> windows_core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComputedFontColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FontOpacity() -> windows_core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontOpacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FontSize() -> windows_core::Result<ClosedCaptionSize> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FontStyle() -> windows_core::Result<ClosedCaptionStyle> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FontEffect() -> windows_core::Result<ClosedCaptionEdgeEffect> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontEffect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BackgroundColor() -> windows_core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedBackgroundColor() -> windows_core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComputedBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BackgroundOpacity() -> windows_core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundOpacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RegionColor() -> windows_core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegionColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedRegionColor() -> windows_core::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComputedRegionColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RegionOpacity() -> windows_core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegionOpacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PropertiesChanged<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        Self::IClosedCaptionPropertiesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PropertiesChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePropertiesChanged))
        })
    }
    fn IClosedCaptionPropertiesStatics<R, F: FnOnce(&IClosedCaptionPropertiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IClosedCaptionPropertiesStatics2<R, F: FnOnce(&IClosedCaptionPropertiesStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: Self = Self(0);
    pub const FiftyPercent: Self = Self(1);
    pub const OneHundredPercent: Self = Self(2);
    pub const OneHundredFiftyPercent: Self = Self(3);
    pub const TwoHundredPercent: Self = Self(4);
}
impl windows_core::TypeKind for ClosedCaptionSize {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.ClosedCaptionSize");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: Self = Self(0);
    pub const MonospacedWithSerifs: Self = Self(1);
    pub const ProportionalWithSerifs: Self = Self(2);
    pub const MonospacedWithoutSerifs: Self = Self(3);
    pub const ProportionalWithoutSerifs: Self = Self(4);
    pub const Casual: Self = Self(5);
    pub const Cursive: Self = Self(6);
    pub const SmallCapitals: Self = Self(7);
}
impl windows_core::TypeKind for ClosedCaptionStyle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.ClosedCaptionStyle");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosedCaptionTheme(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClosedCaptionTheme, windows_core::IUnknown, windows_core::IInspectable);
impl ClosedCaptionTheme {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FontColor(&self) -> windows_core::Result<ClosedCaptionColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn ComputedFontColor(&self) -> windows_core::Result<super::super::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputedFontColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FontOpacity(&self) -> windows_core::Result<ClosedCaptionOpacity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<ClosedCaptionSize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FontStyle(&self) -> windows_core::Result<ClosedCaptionStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FontEffect(&self) -> windows_core::Result<ClosedCaptionEdgeEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<ClosedCaptionColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn ComputedBackgroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputedBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn BackgroundOpacity(&self) -> windows_core::Result<ClosedCaptionOpacity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackgroundOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn RegionColor(&self) -> windows_core::Result<ClosedCaptionColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegionColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn ComputedRegionColor(&self) -> windows_core::Result<super::super::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputedRegionColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn RegionOpacity(&self) -> windows_core::Result<ClosedCaptionOpacity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegionOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetAvailableThemes() -> windows_core::Result<windows_core::Array<Self>> {
        Self::IClosedCaptionThemeStatics(|this| unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableThemes)(windows_core::Interface::as_raw(this), windows_core::Array::<Self>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        })
    }
    pub fn GetSelectedTheme() -> windows_core::Result<Self> {
        Self::IClosedCaptionThemeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSelectedTheme)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TrySetSelectedTheme<P0>(value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        Self::IClosedCaptionThemeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetSelectedTheme)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn ThemesChanged<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        Self::IClosedCaptionThemeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ThemesChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveThemesChanged))
        })
    }
    pub fn SelectedThemeChanged<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        Self::IClosedCaptionThemeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).SelectedThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveSelectedThemeChanged))
        })
    }
    fn IClosedCaptionThemeStatics<R, F: FnOnce(&IClosedCaptionThemeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ClosedCaptionTheme, IClosedCaptionThemeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ClosedCaptionTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClosedCaptionTheme>();
}
unsafe impl windows_core::Interface for ClosedCaptionTheme {
    type Vtable = <IClosedCaptionTheme as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClosedCaptionTheme as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClosedCaptionTheme {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionTheme";
}
unsafe impl Send for ClosedCaptionTheme {}
unsafe impl Sync for ClosedCaptionTheme {}
windows_core::imp::define_interface!(IClosedCaptionPropertiesStatics, IClosedCaptionPropertiesStatics_Vtbl, 0x10aa1f84_cc30_4141_b503_5272289e0c20);
impl windows_core::RuntimeType for IClosedCaptionPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.IClosedCaptionPropertiesStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FontColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionColor) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedFontColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedFontColor: usize,
    pub FontOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionOpacity) -> windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionSize) -> windows_core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionStyle) -> windows_core::HRESULT,
    pub FontEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionEdgeEffect) -> windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionColor) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedBackgroundColor: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionOpacity) -> windows_core::HRESULT,
    pub RegionColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionColor) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedRegionColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedRegionColor: usize,
    pub RegionOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionOpacity) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClosedCaptionPropertiesStatics2, IClosedCaptionPropertiesStatics2_Vtbl, 0x9de26870_37de_4197_8845_9a48dc5ac317);
impl windows_core::RuntimeType for IClosedCaptionPropertiesStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.IClosedCaptionPropertiesStatics2");
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClosedCaptionTheme, IClosedCaptionTheme_Vtbl, 0xd3974055_b9b5_52a4_b655_30661f73d1c1);
impl windows_core::RuntimeType for IClosedCaptionTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.IClosedCaptionTheme");
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionTheme_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FontColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionColor) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedFontColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedFontColor: usize,
    pub FontOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionOpacity) -> windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionSize) -> windows_core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionStyle) -> windows_core::HRESULT,
    pub FontEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionEdgeEffect) -> windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionColor) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedBackgroundColor: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionOpacity) -> windows_core::HRESULT,
    pub RegionColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionColor) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedRegionColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedRegionColor: usize,
    pub RegionOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClosedCaptionOpacity) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClosedCaptionThemeStatics, IClosedCaptionThemeStatics_Vtbl, 0x0938ae0f_214e_5760_88c9_bb3f1b54f3c7);
impl windows_core::RuntimeType for IClosedCaptionThemeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ClosedCaptioning.IClosedCaptionThemeStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionThemeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAvailableThemes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelectedTheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySetSelectedTheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ThemesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveThemesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SelectedThemeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSelectedThemeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
