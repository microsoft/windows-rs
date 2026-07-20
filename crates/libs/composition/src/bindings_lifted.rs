#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnimationIterationBehavior(pub i32);
impl AnimationIterationBehavior {
    pub const Count: Self = Self(0);
    pub const Forever: Self = Self(1);
}
impl windows_core::TypeKind for AnimationIterationBehavior {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AnimationIterationBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.AnimationIterationBehavior;i4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl windows_core::TypeKind for Color {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Color {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionAnimation, CompositionObject);
impl windows_core::RuntimeType for CompositionAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionAnimation>();
}
unsafe impl windows_core::Interface for CompositionAnimation {
    type Vtable = <ICompositionAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionAnimation {
    type Target = ICompositionAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionAnimation";
}
unsafe impl Send for CompositionAnimation {}
unsafe impl Sync for CompositionAnimation {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompositionBatchTypes(pub u32);
impl CompositionBatchTypes {
    pub const None: Self = Self(0);
    pub const Animation: Self = Self(1);
    pub const Effect: Self = Self(2);
    pub const InfiniteAnimation: Self = Self(4);
    pub const AllAnimations: Self = Self(5);
}
impl windows_core::TypeKind for CompositionBatchTypes {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CompositionBatchTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.CompositionBatchTypes;u4)",
    );
}
impl CompositionBatchTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CompositionBatchTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CompositionBatchTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CompositionBatchTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for CompositionBatchTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for CompositionBatchTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompositionBorderMode(pub i32);
impl CompositionBorderMode {
    pub const Inherit: Self = Self(0);
    pub const Soft: Self = Self(1);
    pub const Hard: Self = Self(2);
}
impl windows_core::TypeKind for CompositionBorderMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CompositionBorderMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.CompositionBorderMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionBrush,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionBrush, CompositionObject);
impl windows_core::RuntimeType for CompositionBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionBrush>();
}
unsafe impl windows_core::Interface for CompositionBrush {
    type Vtable = <ICompositionBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionBrush {
    type Target = ICompositionBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionBrush {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionBrush";
}
unsafe impl Send for CompositionBrush {}
unsafe impl Sync for CompositionBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionColorBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionColorBrush,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionColorBrush, CompositionBrush, CompositionObject);
impl windows_core::RuntimeType for CompositionColorBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionColorBrush>();
}
unsafe impl windows_core::Interface for CompositionColorBrush {
    type Vtable = <ICompositionColorBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionColorBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionColorBrush {
    type Target = ICompositionColorBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionColorBrush {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionColorBrush";
}
unsafe impl Send for CompositionColorBrush {}
unsafe impl Sync for CompositionColorBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionContainerShape(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionContainerShape,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CompositionContainerShape,
    CompositionShape,
    CompositionObject
);
impl windows_core::RuntimeType for CompositionContainerShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionContainerShape>();
}
unsafe impl windows_core::Interface for CompositionContainerShape {
    type Vtable = <ICompositionContainerShape as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionContainerShape as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionContainerShape {
    type Target = ICompositionContainerShape;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionContainerShape {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionContainerShape";
}
unsafe impl Send for CompositionContainerShape {}
unsafe impl Sync for CompositionContainerShape {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionEllipseGeometry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionEllipseGeometry,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CompositionEllipseGeometry,
    CompositionGeometry,
    CompositionObject
);
impl windows_core::RuntimeType for CompositionEllipseGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionEllipseGeometry>();
}
unsafe impl windows_core::Interface for CompositionEllipseGeometry {
    type Vtable = <ICompositionEllipseGeometry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionEllipseGeometry as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionEllipseGeometry {
    type Target = ICompositionEllipseGeometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionEllipseGeometry {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionEllipseGeometry";
}
unsafe impl Send for CompositionEllipseGeometry {}
unsafe impl Sync for CompositionEllipseGeometry {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionGeometry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionGeometry,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionGeometry, CompositionObject);
impl windows_core::RuntimeType for CompositionGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionGeometry>();
}
unsafe impl windows_core::Interface for CompositionGeometry {
    type Vtable = <ICompositionGeometry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionGeometry as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionGeometry {
    type Target = ICompositionGeometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionGeometry {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionGeometry";
}
unsafe impl Send for CompositionGeometry {}
unsafe impl Sync for CompositionGeometry {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionNineGridBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionNineGridBrush,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CompositionNineGridBrush,
    CompositionBrush,
    CompositionObject
);
impl windows_core::RuntimeType for CompositionNineGridBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionNineGridBrush>();
}
unsafe impl windows_core::Interface for CompositionNineGridBrush {
    type Vtable = <ICompositionNineGridBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionNineGridBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionNineGridBrush {
    type Target = ICompositionNineGridBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionNineGridBrush {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionNineGridBrush";
}
unsafe impl Send for CompositionNineGridBrush {}
unsafe impl Sync for CompositionNineGridBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionObject>();
}
unsafe impl windows_core::Interface for CompositionObject {
    type Vtable = <ICompositionObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionObject {
    type Target = ICompositionObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionObject {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionObject";
}
unsafe impl Send for CompositionObject {}
unsafe impl Sync for CompositionObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionScopedBatch(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionScopedBatch,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionScopedBatch, CompositionObject);
impl windows_core::RuntimeType for CompositionScopedBatch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionScopedBatch>();
}
unsafe impl windows_core::Interface for CompositionScopedBatch {
    type Vtable = <ICompositionScopedBatch as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionScopedBatch as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionScopedBatch {
    type Target = ICompositionScopedBatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionScopedBatch {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionScopedBatch";
}
unsafe impl Send for CompositionScopedBatch {}
unsafe impl Sync for CompositionScopedBatch {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionShape(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionShape,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionShape, CompositionObject);
impl windows_core::RuntimeType for CompositionShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionShape>();
}
unsafe impl windows_core::Interface for CompositionShape {
    type Vtable = <ICompositionShape as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionShape as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionShape {
    type Target = ICompositionShape;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionShape {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionShape";
}
unsafe impl Send for CompositionShape {}
unsafe impl Sync for CompositionShape {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionShapeCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionShapeCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IVector<CompositionShape>
);
windows_core::imp::required_hierarchy!(CompositionShapeCollection, CompositionObject);
impl windows_core::RuntimeType for CompositionShapeCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVector<CompositionShape>>();
}
unsafe impl windows_core::Interface for CompositionShapeCollection {
    type Vtable = <IVector<CompositionShape> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVector<CompositionShape> as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionShapeCollection {
    type Target = IVector<CompositionShape>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionShapeCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionShapeCollection";
}
unsafe impl Send for CompositionShapeCollection {}
unsafe impl Sync for CompositionShapeCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionSpriteShape(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionSpriteShape,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionSpriteShape, CompositionShape, CompositionObject);
impl windows_core::RuntimeType for CompositionSpriteShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionSpriteShape>();
}
unsafe impl windows_core::Interface for CompositionSpriteShape {
    type Vtable = <ICompositionSpriteShape as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionSpriteShape as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionSpriteShape {
    type Target = ICompositionSpriteShape;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionSpriteShape {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionSpriteShape";
}
unsafe impl Send for CompositionSpriteShape {}
unsafe impl Sync for CompositionSpriteShape {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compositor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Compositor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for Compositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositor>();
}
unsafe impl windows_core::Interface for Compositor {
    type Vtable = <ICompositor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositor as windows_core::Interface>::IID;
}
impl core::ops::Deref for Compositor {
    type Target = ICompositor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Compositor {
    const NAME: &'static str = "Microsoft.UI.Composition.Compositor";
}
unsafe impl Send for Compositor {}
unsafe impl Sync for Compositor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContainerVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ContainerVisual, Visual, CompositionObject);
impl windows_core::RuntimeType for ContainerVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContainerVisual>();
}
unsafe impl windows_core::Interface for ContainerVisual {
    type Vtable = <IContainerVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContainerVisual as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContainerVisual {
    type Target = IContainerVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContainerVisual {
    const NAME: &'static str = "Microsoft.UI.Composition.ContainerVisual";
}
unsafe impl Send for ContainerVisual {}
unsafe impl Sync for ContainerVisual {}
windows_core::imp::define_interface!(
    ICompositionAnimation,
    ICompositionAnimation_Vtbl,
    0xa829ccc8_6fde_5b90_ad37_efd307e1b631
);
impl windows_core::RuntimeType for ICompositionAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionBrush,
    ICompositionBrush_Vtbl,
    0x483924e7_99a5_5377_968b_dec6d40bbccd
);
impl windows_core::RuntimeType for ICompositionBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionColorBrush,
    ICompositionColorBrush_Vtbl,
    0x3f8ffb69_3e71_55a7_8e79_f27a214c56ae
);
impl windows_core::RuntimeType for ICompositionColorBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionColorBrush {
    pub(crate) fn Color(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Color)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetColor(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionColorBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Color:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionContainerShape,
    ICompositionContainerShape_Vtbl,
    0x064aabd5_2dab_52d3_824b_c72456540f29
);
impl windows_core::RuntimeType for ICompositionContainerShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionContainerShape {
    pub(crate) fn Shapes(&self) -> windows_core::Result<CompositionShapeCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Shapes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositionContainerShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Shapes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionEllipseGeometry,
    ICompositionEllipseGeometry_Vtbl,
    0xf2a21042_7a57_58c1_8b47_8bc8b21d3aa0
);
impl windows_core::RuntimeType for ICompositionEllipseGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionEllipseGeometry {
    pub(crate) fn SetRadius(&self, value: windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRadius)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionEllipseGeometry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Center: usize,
    SetCenter: usize,
    Radius: usize,
    pub SetRadius: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionGeometry,
    ICompositionGeometry_Vtbl,
    0x4e40bdb2_450b_5a81_9e9b_149417980cc4
);
impl windows_core::RuntimeType for ICompositionGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionGeometry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionNineGridBrush,
    ICompositionNineGridBrush_Vtbl,
    0xc77a3d21_c7ee_517a_98f4_ad9a7202bc86
);
impl windows_core::RuntimeType for ICompositionNineGridBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionNineGridBrush {
    pub(crate) fn SetIsCenterHollow(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsCenterHollow)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CompositionBrush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetInsetsWithValues(
        &self,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInsetsWithValues)(
                windows_core::Interface::as_raw(self),
                left,
                top,
                right,
                bottom,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionNineGridBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BottomInset: usize,
    SetBottomInset: usize,
    BottomInsetScale: usize,
    SetBottomInsetScale: usize,
    IsCenterHollow: usize,
    pub SetIsCenterHollow:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    LeftInset: usize,
    SetLeftInset: usize,
    LeftInsetScale: usize,
    SetLeftInsetScale: usize,
    RightInset: usize,
    SetRightInset: usize,
    RightInsetScale: usize,
    SetRightInsetScale: usize,
    Source: usize,
    pub SetSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TopInset: usize,
    SetTopInset: usize,
    TopInsetScale: usize,
    SetTopInsetScale: usize,
    SetInsets: usize,
    pub SetInsetsWithValues: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        f32,
        f32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionObject,
    ICompositionObject_Vtbl,
    0x0e583d49_fb5e_5481_a426_d3c41e059a5a
);
impl windows_core::RuntimeType for ICompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionObject {
    pub(crate) fn Compositor(&self) -> windows_core::Result<Compositor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compositor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn StartAnimation<P1>(
        &self,
        propertyname: &str,
        animation: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<CompositionAnimation>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).StartAnimation)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(propertyname)),
                animation.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Compositor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Properties: usize,
    pub StartAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionScopedBatch,
    ICompositionScopedBatch_Vtbl,
    0xd31ca572_99ce_5969_b042_6c2d330a3859
);
impl windows_core::RuntimeType for ICompositionScopedBatch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionScopedBatch {
    pub(crate) fn End(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)).ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionScopedBatch_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsActive: usize,
    IsEnded: usize,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionShape,
    ICompositionShape_Vtbl,
    0xed75d4d8_437f_5640_9720_faae35ce5895
);
impl windows_core::RuntimeType for ICompositionShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionShape {
    pub(crate) fn SetOffset(&self, value: windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffset)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CenterPoint: usize,
    SetCenterPoint: usize,
    Offset: usize,
    pub SetOffset: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionSpriteShape,
    ICompositionSpriteShape_Vtbl,
    0x982138f2_5781_509a_ba5d_112bcb0b98ef
);
impl windows_core::RuntimeType for ICompositionSpriteShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionSpriteShape {
    pub(crate) fn SetFillBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CompositionBrush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFillBrush)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionSpriteShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FillBrush: usize,
    pub SetFillBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositor,
    ICompositor_Vtbl,
    0x95213c13_c4cb_57de_b267_d21ab901ae38
);
impl windows_core::RuntimeType for ICompositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositor {
    pub(crate) fn CreateColorBrushWithColor(
        &self,
        color: Color,
    ) -> windows_core::Result<CompositionColorBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorBrushWithColor)(
                windows_core::Interface::as_raw(self),
                color,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateContainerVisual(&self) -> windows_core::Result<ContainerVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContainerVisual)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateScopedBatch(
        &self,
        batchtype: CompositionBatchTypes,
    ) -> windows_core::Result<CompositionScopedBatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScopedBatch)(
                windows_core::Interface::as_raw(self),
                batchtype,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateSpriteVisual(&self) -> windows_core::Result<SpriteVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpriteVisual)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateVector3KeyFrameAnimation(
        &self,
    ) -> windows_core::Result<Vector3KeyFrameAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVector3KeyFrameAnimation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateColorKeyFrameAnimation: usize,
    CreateColorBrush: usize,
    pub CreateColorBrushWithColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Color,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateContainerVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateCubicBezierEasingFunction: usize,
    CreateEffectFactory: usize,
    CreateEffectFactoryWithProperties: usize,
    CreateExpressionAnimation: usize,
    CreateExpressionAnimationWithExpression: usize,
    CreateInsetClip: usize,
    CreateInsetClipWithInsets: usize,
    CreateLinearEasingFunction: usize,
    CreatePropertySet: usize,
    CreateQuaternionKeyFrameAnimation: usize,
    CreateScalarKeyFrameAnimation: usize,
    pub CreateScopedBatch: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CompositionBatchTypes,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSpriteVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateSurfaceBrush: usize,
    CreateSurfaceBrushWithSurface: usize,
    CreateVector2KeyFrameAnimation: usize,
    pub CreateVector3KeyFrameAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositor2,
    ICompositor2_Vtbl,
    0xa9ffedad_3982_576d_a38a_c888ff605819
);
impl windows_core::RuntimeType for ICompositor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositor2 {
    pub(crate) fn CreateNineGridBrush(&self) -> windows_core::Result<CompositionNineGridBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNineGridBrush)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateAmbientLight: usize,
    CreateAnimationGroup: usize,
    CreateBackdropBrush: usize,
    CreateDistantLight: usize,
    CreateDropShadow: usize,
    CreateImplicitAnimationCollection: usize,
    CreateLayerVisual: usize,
    CreateMaskBrush: usize,
    pub CreateNineGridBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositor5,
    ICompositor5_Vtbl,
    0xbb52d644_a030_5c19_b883_577ded739ae7
);
impl windows_core::RuntimeType for ICompositor5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositor5 {
    pub(crate) fn CreateContainerShape(&self) -> windows_core::Result<CompositionContainerShape> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContainerShape)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateEllipseGeometry(&self) -> windows_core::Result<CompositionEllipseGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipseGeometry)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateShapeVisual(&self) -> windows_core::Result<ShapeVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateShapeVisual)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateSpriteShapeWithGeometry<P0>(
        &self,
        geometry: P0,
    ) -> windows_core::Result<CompositionSpriteShape>
    where
        P0: windows_core::Param<CompositionGeometry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpriteShapeWithGeometry)(
                windows_core::Interface::as_raw(self),
                geometry.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositor5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Comment: usize,
    SetComment: usize,
    GlobalPlaybackRate: usize,
    SetGlobalPlaybackRate: usize,
    CreateBounceScalarAnimation: usize,
    CreateBounceVector2Animation: usize,
    CreateBounceVector3Animation: usize,
    pub CreateContainerShape: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEllipseGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateLineGeometry: usize,
    CreatePathGeometry: usize,
    CreatePathGeometryWithPath: usize,
    CreatePathKeyFrameAnimation: usize,
    CreateRectangleGeometry: usize,
    CreateRoundedRectangleGeometry: usize,
    pub CreateShapeVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateSpriteShape: usize,
    pub CreateSpriteShapeWithGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContainerVisual,
    IContainerVisual_Vtbl,
    0xc70dbce1_2c2f_5d8e_91a4_aae1121e6186
);
impl windows_core::RuntimeType for IContainerVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContainerVisual {
    pub(crate) fn Children(&self) -> windows_core::Result<VisualCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Children)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IContainerVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Children: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IKeyFrameAnimation,
    IKeyFrameAnimation_Vtbl,
    0x5a8f57f0_f059_5b47_b308_c4c80fc71248
);
impl windows_core::RuntimeType for IKeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IKeyFrameAnimation {
    pub(crate) fn SetDelayTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDelayTime)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetDuration(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDuration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIterationBehavior(
        &self,
        value: AnimationIterationBehavior,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIterationBehavior)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIterationCount(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIterationCount)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IKeyFrameAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DelayTime: usize,
    pub SetDelayTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    Duration: usize,
    pub SetDuration: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    IterationBehavior: usize,
    pub SetIterationBehavior: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AnimationIterationBehavior,
    ) -> windows_core::HRESULT,
    IterationCount: usize,
    pub SetIterationCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IShapeVisual,
    IShapeVisual_Vtbl,
    0xa911c80b_a5a5_5aca_b8ff_c43f08f06143
);
impl windows_core::RuntimeType for IShapeVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IShapeVisual {
    pub(crate) fn Shapes(&self) -> windows_core::Result<CompositionShapeCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Shapes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IShapeVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Shapes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISpriteVisual,
    ISpriteVisual_Vtbl,
    0x7e964632_45e4_5761_806d_5b4022c14f26
);
impl windows_core::RuntimeType for ISpriteVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISpriteVisual {
    pub(crate) fn SetBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CompositionBrush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBrush)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISpriteVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Brush: usize,
    pub SetBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVector<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVector<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_collections::IIterable<T>> for IVector<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub(crate) fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Append)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
#[repr(C)]
pub struct IVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    GetAt: usize,
    Size: usize,
    GetView: usize,
    IndexOf: usize,
    SetAt: usize,
    InsertAt: usize,
    RemoveAt: usize,
    pub Append: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(
    IVector3KeyFrameAnimation,
    IVector3KeyFrameAnimation_Vtbl,
    0xd7da980e_2dde_5dd1_a40c_d6868dd2449e
);
impl windows_core::RuntimeType for IVector3KeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IVector3KeyFrameAnimation {
    pub(crate) fn InsertKeyFrame(
        &self,
        normalizedprogresskey: f32,
        value: windows_numerics::Vector3,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrame)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IVector3KeyFrameAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InsertKeyFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IVisual,
    IVisual_Vtbl,
    0xc0eeab6c_c897_5ac6_a1c9_63abd5055b9b
);
impl windows_core::RuntimeType for IVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IVisual {
    pub(crate) fn SetAnchorPoint(
        &self,
        value: windows_numerics::Vector2,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAnchorPoint)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetBorderMode(&self, value: CompositionBorderMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetCenterPoint(
        &self,
        value: windows_numerics::Vector3,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCenterPoint)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn IsVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetIsVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Offset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Offset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetOffset(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffset)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Opacity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Opacity)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetOpacity(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpacity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Parent(&self) -> windows_core::Result<ContainerVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetScale)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Size(&self) -> windows_core::Result<windows_numerics::Vector2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetSize(&self, value: windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AnchorPoint: usize,
    pub SetAnchorPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
    BackfaceVisibility: usize,
    SetBackfaceVisibility: usize,
    BorderMode: usize,
    pub SetBorderMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CompositionBorderMode,
    ) -> windows_core::HRESULT,
    CenterPoint: usize,
    pub SetCenterPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    Clip: usize,
    SetClip: usize,
    CompositeMode: usize,
    SetCompositeMode: usize,
    pub IsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub Opacity:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    Orientation: usize,
    SetOrientation: usize,
    pub Parent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    RotationAngle: usize,
    SetRotationAngle: usize,
    RotationAngleInDegrees: usize,
    SetRotationAngleInDegrees: usize,
    RotationAxis: usize,
    SetRotationAxis: usize,
    Scale: usize,
    pub SetScale: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IVisual2,
    IVisual2_Vtbl,
    0x492a7995_0c5c_5993_a283_52e4da3050ee
);
impl windows_core::RuntimeType for IVisual2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IVisual2 {
    pub(crate) fn SetParentForTransform<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Visual>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetParentForTransform)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetRelativeOffsetAdjustment(
        &self,
        value: windows_numerics::Vector3,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRelativeOffsetAdjustment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetRelativeSizeAdjustment(
        &self,
        value: windows_numerics::Vector2,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRelativeSizeAdjustment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IVisual2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ParentForTransform: usize,
    pub SetParentForTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    RelativeOffsetAdjustment: usize,
    pub SetRelativeOffsetAdjustment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    RelativeSizeAdjustment: usize,
    pub SetRelativeSizeAdjustment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IVisualCollection,
    IVisualCollection_Vtbl,
    0xd002896d_67d8_5f69_ab70_581fa3bf370f
);
impl windows_core::RuntimeType for IVisualCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IVisualCollection {
    pub(crate) fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn InsertAtBottom<P0>(&self, newchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Visual>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertAtBottom)(
                windows_core::Interface::as_raw(self),
                newchild.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn InsertAtTop<P0>(&self, newchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Visual>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertAtTop)(
                windows_core::Interface::as_raw(self),
                newchild.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn Remove<P0>(&self, child: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Visual>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Remove)(
                windows_core::Interface::as_raw(self),
                child.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn RemoveAll(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IVisualCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    InsertAbove: usize,
    pub InsertAtBottom: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InsertAtTop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    InsertBelow: usize,
    pub Remove: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(KeyFrameAnimation, CompositionAnimation, CompositionObject);
impl windows_core::RuntimeType for KeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IKeyFrameAnimation>();
}
unsafe impl windows_core::Interface for KeyFrameAnimation {
    type Vtable = <IKeyFrameAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyFrameAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for KeyFrameAnimation {
    type Target = IKeyFrameAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for KeyFrameAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.KeyFrameAnimation";
}
unsafe impl Send for KeyFrameAnimation {}
unsafe impl Sync for KeyFrameAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShapeVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ShapeVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ShapeVisual, ContainerVisual, Visual, CompositionObject);
impl windows_core::RuntimeType for ShapeVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IShapeVisual>();
}
unsafe impl windows_core::Interface for ShapeVisual {
    type Vtable = <IShapeVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IShapeVisual as windows_core::Interface>::IID;
}
impl core::ops::Deref for ShapeVisual {
    type Target = IShapeVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ShapeVisual {
    const NAME: &'static str = "Microsoft.UI.Composition.ShapeVisual";
}
unsafe impl Send for ShapeVisual {}
unsafe impl Sync for ShapeVisual {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpriteVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SpriteVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SpriteVisual, ContainerVisual, Visual, CompositionObject);
impl windows_core::RuntimeType for SpriteVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISpriteVisual>();
}
unsafe impl windows_core::Interface for SpriteVisual {
    type Vtable = <ISpriteVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpriteVisual as windows_core::Interface>::IID;
}
impl core::ops::Deref for SpriteVisual {
    type Target = ISpriteVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SpriteVisual {
    const NAME: &'static str = "Microsoft.UI.Composition.SpriteVisual";
}
unsafe impl Send for SpriteVisual {}
unsafe impl Sync for SpriteVisual {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vector3KeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Vector3KeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Vector3KeyFrameAnimation,
    KeyFrameAnimation,
    CompositionAnimation,
    CompositionObject
);
impl windows_core::RuntimeType for Vector3KeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVector3KeyFrameAnimation>();
}
unsafe impl windows_core::Interface for Vector3KeyFrameAnimation {
    type Vtable = <IVector3KeyFrameAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVector3KeyFrameAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for Vector3KeyFrameAnimation {
    type Target = IVector3KeyFrameAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Vector3KeyFrameAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.Vector3KeyFrameAnimation";
}
unsafe impl Send for Vector3KeyFrameAnimation {}
unsafe impl Sync for Vector3KeyFrameAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Visual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Visual, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Visual, CompositionObject);
impl windows_core::RuntimeType for Visual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisual>();
}
unsafe impl windows_core::Interface for Visual {
    type Vtable = <IVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisual as windows_core::Interface>::IID;
}
impl core::ops::Deref for Visual {
    type Target = IVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Visual {
    const NAME: &'static str = "Microsoft.UI.Composition.Visual";
}
unsafe impl Send for Visual {}
unsafe impl Sync for Visual {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisualCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    VisualCollection,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(VisualCollection, CompositionObject);
impl windows_core::RuntimeType for VisualCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisualCollection>();
}
unsafe impl windows_core::Interface for VisualCollection {
    type Vtable = <IVisualCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisualCollection as windows_core::Interface>::IID;
}
impl core::ops::Deref for VisualCollection {
    type Target = IVisualCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for VisualCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.VisualCollection";
}
unsafe impl Send for VisualCollection {}
unsafe impl Sync for VisualCollection {}
