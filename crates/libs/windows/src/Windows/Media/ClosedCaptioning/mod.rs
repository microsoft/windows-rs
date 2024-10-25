windows_core::imp::define_interface!(IClosedCaptionPropertiesStatics, IClosedCaptionPropertiesStatics_Vtbl, 0x10aa1f84_cc30_4141_b503_5272289e0c20);
impl windows_core::RuntimeType for IClosedCaptionPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
}
#[repr(C)]
pub struct IClosedCaptionPropertiesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
pub struct ClosedCaptionProperties;
impl ClosedCaptionProperties {}
impl windows_core::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
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
impl windows_core::TypeKind for ClosedCaptionColor {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Raised: Self = Self(2i32);
    pub const Depressed: Self = Self(3i32);
    pub const Uniform: Self = Self(4i32);
    pub const DropShadow: Self = Self(5i32);
}
impl windows_core::TypeKind for ClosedCaptionEdgeEffect {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: Self = Self(0i32);
    pub const OneHundredPercent: Self = Self(1i32);
    pub const SeventyFivePercent: Self = Self(2i32);
    pub const TwentyFivePercent: Self = Self(3i32);
    pub const ZeroPercent: Self = Self(4i32);
}
impl windows_core::TypeKind for ClosedCaptionOpacity {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: Self = Self(0i32);
    pub const FiftyPercent: Self = Self(1i32);
    pub const OneHundredPercent: Self = Self(2i32);
    pub const OneHundredFiftyPercent: Self = Self(3i32);
    pub const TwoHundredPercent: Self = Self(4i32);
}
impl windows_core::TypeKind for ClosedCaptionSize {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
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
impl windows_core::TypeKind for ClosedCaptionStyle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
}
