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
        b"enum(Windows.UI.Composition.AnimationIterationBehavior;i4)",
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
windows_core::imp::required_hierarchy!(
    CompositionAnimation,
    ICompositionAnimationBase,
    CompositionObject
);
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionAnimation";
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
        b"enum(Windows.UI.Composition.CompositionBatchTypes;u4)",
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
        b"enum(Windows.UI.Composition.CompositionBorderMode;i4)",
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionBrush";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionColorBrush";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionContainerShape";
}
unsafe impl Send for CompositionContainerShape {}
unsafe impl Sync for CompositionContainerShape {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionDrawingSurface(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionDrawingSurface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CompositionDrawingSurface,
    ICompositionSurface,
    CompositionObject
);
impl windows_core::RuntimeType for CompositionDrawingSurface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionDrawingSurface>();
}
unsafe impl windows_core::Interface for CompositionDrawingSurface {
    type Vtable = <ICompositionDrawingSurface as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionDrawingSurface as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionDrawingSurface {
    type Target = ICompositionDrawingSurface;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionDrawingSurface {
    const NAME: &'static str = "Windows.UI.Composition.CompositionDrawingSurface";
}
unsafe impl Send for CompositionDrawingSurface {}
unsafe impl Sync for CompositionDrawingSurface {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionEasingFunction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionEasingFunction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionEasingFunction, CompositionObject);
impl windows_core::RuntimeType for CompositionEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionEasingFunction>();
}
unsafe impl windows_core::Interface for CompositionEasingFunction {
    type Vtable = <ICompositionEasingFunction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionEasingFunction as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionEasingFunction {
    type Target = ICompositionEasingFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.CompositionEasingFunction";
}
unsafe impl Send for CompositionEasingFunction {}
unsafe impl Sync for CompositionEasingFunction {}
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionEllipseGeometry";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionGeometry";
}
unsafe impl Send for CompositionGeometry {}
unsafe impl Sync for CompositionGeometry {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionGraphicsDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionGraphicsDevice,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionGraphicsDevice, CompositionObject);
impl windows_core::RuntimeType for CompositionGraphicsDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionGraphicsDevice>();
}
unsafe impl windows_core::Interface for CompositionGraphicsDevice {
    type Vtable = <ICompositionGraphicsDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionGraphicsDevice as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionGraphicsDevice {
    type Target = ICompositionGraphicsDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionGraphicsDevice {
    const NAME: &'static str = "Windows.UI.Composition.CompositionGraphicsDevice";
}
unsafe impl Send for CompositionGraphicsDevice {}
unsafe impl Sync for CompositionGraphicsDevice {}
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionNineGridBrush";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionObject";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionScopedBatch";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionShape";
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
    windows_collections::IVector<CompositionShape>
);
windows_core::imp::required_hierarchy!(CompositionShapeCollection, CompositionObject);
impl windows_core::RuntimeType for CompositionShapeCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IVector<CompositionShape>,
    >();
}
unsafe impl windows_core::Interface for CompositionShapeCollection {
    type Vtable =
        <windows_collections::IVector<CompositionShape> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<CompositionShape> as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionShapeCollection {
    type Target = windows_collections::IVector<CompositionShape>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionShapeCollection {
    const NAME: &'static str = "Windows.UI.Composition.CompositionShapeCollection";
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
    const NAME: &'static str = "Windows.UI.Composition.CompositionSpriteShape";
}
unsafe impl Send for CompositionSpriteShape {}
unsafe impl Sync for CompositionSpriteShape {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionSurfaceBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionSurfaceBrush,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CompositionSurfaceBrush,
    CompositionBrush,
    CompositionObject
);
impl windows_core::RuntimeType for CompositionSurfaceBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionSurfaceBrush>();
}
unsafe impl windows_core::Interface for CompositionSurfaceBrush {
    type Vtable = <ICompositionSurfaceBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionSurfaceBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionSurfaceBrush {
    type Target = ICompositionSurfaceBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionSurfaceBrush {
    const NAME: &'static str = "Windows.UI.Composition.CompositionSurfaceBrush";
}
unsafe impl Send for CompositionSurfaceBrush {}
unsafe impl Sync for CompositionSurfaceBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionTarget, CompositionObject);
impl windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionTarget>();
}
unsafe impl windows_core::Interface for CompositionTarget {
    type Vtable = <ICompositionTarget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionTarget as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionTarget {
    type Target = ICompositionTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Windows.UI.Composition.CompositionTarget";
}
unsafe impl Send for CompositionTarget {}
unsafe impl Sync for CompositionTarget {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compositor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Compositor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Compositor {
    pub(crate) fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Compositor,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
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
    const NAME: &'static str = "Windows.UI.Composition.Compositor";
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
    const NAME: &'static str = "Windows.UI.Composition.ContainerVisual";
}
unsafe impl Send for ContainerVisual {}
unsafe impl Sync for ContainerVisual {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CubicBezierEasingFunction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CubicBezierEasingFunction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CubicBezierEasingFunction,
    CompositionEasingFunction,
    CompositionObject
);
impl windows_core::RuntimeType for CubicBezierEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICubicBezierEasingFunction>();
}
unsafe impl windows_core::Interface for CubicBezierEasingFunction {
    type Vtable = <ICubicBezierEasingFunction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICubicBezierEasingFunction as windows_core::Interface>::IID;
}
impl core::ops::Deref for CubicBezierEasingFunction {
    type Target = ICubicBezierEasingFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CubicBezierEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.CubicBezierEasingFunction";
}
unsafe impl Send for CubicBezierEasingFunction {}
unsafe impl Sync for CubicBezierEasingFunction {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopWindowTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DesktopWindowTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DesktopWindowTarget, CompositionTarget, CompositionObject);
impl windows_core::RuntimeType for DesktopWindowTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDesktopWindowTarget>();
}
unsafe impl windows_core::Interface for DesktopWindowTarget {
    type Vtable = <IDesktopWindowTarget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDesktopWindowTarget as windows_core::Interface>::IID;
}
impl core::ops::Deref for DesktopWindowTarget {
    type Target = IDesktopWindowTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
unsafe impl Send for DesktopWindowTarget {}
unsafe impl Sync for DesktopWindowTarget {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DirectXAlphaMode(pub i32);
impl DirectXAlphaMode {
    pub const Unspecified: Self = Self(0);
    pub const Premultiplied: Self = Self(1);
    pub const Straight: Self = Self(2);
    pub const Ignore: Self = Self(3);
}
impl windows_core::TypeKind for DirectXAlphaMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DirectXAlphaMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Graphics.DirectX.DirectXAlphaMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DirectXPixelFormat(pub i32);
impl DirectXPixelFormat {
    pub const Unknown: Self = Self(0);
    pub const R32G32B32A32Typeless: Self = Self(1);
    pub const R32G32B32A32Float: Self = Self(2);
    pub const R32G32B32A32UInt: Self = Self(3);
    pub const R32G32B32A32Int: Self = Self(4);
    pub const R32G32B32Typeless: Self = Self(5);
    pub const R32G32B32Float: Self = Self(6);
    pub const R32G32B32UInt: Self = Self(7);
    pub const R32G32B32Int: Self = Self(8);
    pub const R16G16B16A16Typeless: Self = Self(9);
    pub const R16G16B16A16Float: Self = Self(10);
    pub const R16G16B16A16UIntNormalized: Self = Self(11);
    pub const R16G16B16A16UInt: Self = Self(12);
    pub const R16G16B16A16IntNormalized: Self = Self(13);
    pub const R16G16B16A16Int: Self = Self(14);
    pub const R32G32Typeless: Self = Self(15);
    pub const R32G32Float: Self = Self(16);
    pub const R32G32UInt: Self = Self(17);
    pub const R32G32Int: Self = Self(18);
    pub const R32G8X24Typeless: Self = Self(19);
    pub const D32FloatS8X24UInt: Self = Self(20);
    pub const R32FloatX8X24Typeless: Self = Self(21);
    pub const X32TypelessG8X24UInt: Self = Self(22);
    pub const R10G10B10A2Typeless: Self = Self(23);
    pub const R10G10B10A2UIntNormalized: Self = Self(24);
    pub const R10G10B10A2UInt: Self = Self(25);
    pub const R11G11B10Float: Self = Self(26);
    pub const R8G8B8A8Typeless: Self = Self(27);
    pub const R8G8B8A8UIntNormalized: Self = Self(28);
    pub const R8G8B8A8UIntNormalizedSrgb: Self = Self(29);
    pub const R8G8B8A8UInt: Self = Self(30);
    pub const R8G8B8A8IntNormalized: Self = Self(31);
    pub const R8G8B8A8Int: Self = Self(32);
    pub const R16G16Typeless: Self = Self(33);
    pub const R16G16Float: Self = Self(34);
    pub const R16G16UIntNormalized: Self = Self(35);
    pub const R16G16UInt: Self = Self(36);
    pub const R16G16IntNormalized: Self = Self(37);
    pub const R16G16Int: Self = Self(38);
    pub const R32Typeless: Self = Self(39);
    pub const D32Float: Self = Self(40);
    pub const R32Float: Self = Self(41);
    pub const R32UInt: Self = Self(42);
    pub const R32Int: Self = Self(43);
    pub const R24G8Typeless: Self = Self(44);
    pub const D24UIntNormalizedS8UInt: Self = Self(45);
    pub const R24UIntNormalizedX8Typeless: Self = Self(46);
    pub const X24TypelessG8UInt: Self = Self(47);
    pub const R8G8Typeless: Self = Self(48);
    pub const R8G8UIntNormalized: Self = Self(49);
    pub const R8G8UInt: Self = Self(50);
    pub const R8G8IntNormalized: Self = Self(51);
    pub const R8G8Int: Self = Self(52);
    pub const R16Typeless: Self = Self(53);
    pub const R16Float: Self = Self(54);
    pub const D16UIntNormalized: Self = Self(55);
    pub const R16UIntNormalized: Self = Self(56);
    pub const R16UInt: Self = Self(57);
    pub const R16IntNormalized: Self = Self(58);
    pub const R16Int: Self = Self(59);
    pub const R8Typeless: Self = Self(60);
    pub const R8UIntNormalized: Self = Self(61);
    pub const R8UInt: Self = Self(62);
    pub const R8IntNormalized: Self = Self(63);
    pub const R8Int: Self = Self(64);
    pub const A8UIntNormalized: Self = Self(65);
    pub const R1UIntNormalized: Self = Self(66);
    pub const R9G9B9E5SharedExponent: Self = Self(67);
    pub const R8G8B8G8UIntNormalized: Self = Self(68);
    pub const G8R8G8B8UIntNormalized: Self = Self(69);
    pub const BC1Typeless: Self = Self(70);
    pub const BC1UIntNormalized: Self = Self(71);
    pub const BC1UIntNormalizedSrgb: Self = Self(72);
    pub const BC2Typeless: Self = Self(73);
    pub const BC2UIntNormalized: Self = Self(74);
    pub const BC2UIntNormalizedSrgb: Self = Self(75);
    pub const BC3Typeless: Self = Self(76);
    pub const BC3UIntNormalized: Self = Self(77);
    pub const BC3UIntNormalizedSrgb: Self = Self(78);
    pub const BC4Typeless: Self = Self(79);
    pub const BC4UIntNormalized: Self = Self(80);
    pub const BC4IntNormalized: Self = Self(81);
    pub const BC5Typeless: Self = Self(82);
    pub const BC5UIntNormalized: Self = Self(83);
    pub const BC5IntNormalized: Self = Self(84);
    pub const B5G6R5UIntNormalized: Self = Self(85);
    pub const B5G5R5A1UIntNormalized: Self = Self(86);
    pub const B8G8R8A8UIntNormalized: Self = Self(87);
    pub const B8G8R8X8UIntNormalized: Self = Self(88);
    pub const R10G10B10XRBiasA2UIntNormalized: Self = Self(89);
    pub const B8G8R8A8Typeless: Self = Self(90);
    pub const B8G8R8A8UIntNormalizedSrgb: Self = Self(91);
    pub const B8G8R8X8Typeless: Self = Self(92);
    pub const B8G8R8X8UIntNormalizedSrgb: Self = Self(93);
    pub const BC6HTypeless: Self = Self(94);
    pub const BC6H16UnsignedFloat: Self = Self(95);
    pub const BC6H16Float: Self = Self(96);
    pub const BC7Typeless: Self = Self(97);
    pub const BC7UIntNormalized: Self = Self(98);
    pub const BC7UIntNormalizedSrgb: Self = Self(99);
    pub const Ayuv: Self = Self(100);
    pub const Y410: Self = Self(101);
    pub const Y416: Self = Self(102);
    pub const NV12: Self = Self(103);
    pub const P010: Self = Self(104);
    pub const P016: Self = Self(105);
    pub const Opaque420: Self = Self(106);
    pub const Yuy2: Self = Self(107);
    pub const Y210: Self = Self(108);
    pub const Y216: Self = Self(109);
    pub const NV11: Self = Self(110);
    pub const AI44: Self = Self(111);
    pub const IA44: Self = Self(112);
    pub const P8: Self = Self(113);
    pub const A8P8: Self = Self(114);
    pub const B4G4R4A4UIntNormalized: Self = Self(115);
    pub const P208: Self = Self(130);
    pub const V208: Self = Self(131);
    pub const V408: Self = Self(132);
    pub const SamplerFeedbackMinMipOpaque: Self = Self(189);
    pub const SamplerFeedbackMipRegionUsedOpaque: Self = Self(190);
    pub const A4B4G4R4: Self = Self(191);
}
impl windows_core::TypeKind for DirectXPixelFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DirectXPixelFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueueController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueueController,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueueController>();
}
unsafe impl windows_core::Interface for DispatcherQueueController {
    type Vtable = <IDispatcherQueueController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueueController as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueueController {
    type Target = IDispatcherQueueController;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
unsafe impl Send for DispatcherQueueController {}
unsafe impl Sync for DispatcherQueueController {}
pub type HWND = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    ICompositionAnimation,
    ICompositionAnimation_Vtbl,
    0x464c4c2c_1caa_4061_9b40_e13fde1503ca
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
    ICompositionAnimation2,
    ICompositionAnimation2_Vtbl,
    0x369b603e_a80f_4948_93e3_ed23fb38c6cb
);
impl windows_core::RuntimeType for ICompositionAnimation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionAnimation2 {
    pub(crate) fn SetTarget(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTarget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionAnimation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SetBooleanParameter: usize,
    Target: usize,
    pub SetTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionAnimationBase,
    ICompositionAnimationBase_Vtbl,
    0x1c2c2999_e818_48d3_a6dd_d78c82f8ace9
);
impl windows_core::RuntimeType for ICompositionAnimationBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ICompositionAnimationBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for ICompositionAnimationBase {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationBase";
}
pub trait ICompositionAnimationBase_Impl: windows_core::IUnknownImpl {}
impl ICompositionAnimationBase_Vtbl {
    pub const fn new<Identity: ICompositionAnimationBase_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICompositionAnimationBase,
                OFFSET,
            >(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ICompositionAnimationBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionBrush,
    ICompositionBrush_Vtbl,
    0xab0d7608_30c0_40e9_b568_b60a6bd1fb46
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
    0x2b264c5e_bf35_4831_8642_cf70c20fff2f
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
    0x4f5e859b_2e5b_44a8_982c_aa0f69c16059
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
    ICompositionDrawingSurface,
    ICompositionDrawingSurface_Vtbl,
    0xa166c300_fad0_4d11_9e67_e433162ff49e
);
impl windows_core::RuntimeType for ICompositionDrawingSurface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionDrawingSurface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionDrawingSurfaceInterop,
    ICompositionDrawingSurfaceInterop_Vtbl,
    0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee
);
windows_core::imp::interface_hierarchy!(ICompositionDrawingSurfaceInterop, windows_core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    pub(crate) unsafe fn BeginDraw<T>(
        &self,
        updaterect: Option<*const RECT>,
        updateoffset: *mut POINT,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(
                windows_core::Interface::as_raw(self),
                updaterect.unwrap_or(core::mem::zeroed()) as _,
                &T::IID,
                &mut result__,
                updateoffset as _,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn EndDraw(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn Resize(&self, sizepixels: SIZE) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Resize)(
                windows_core::Interface::as_raw(self),
                sizepixels,
            )
        }
    }
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const RECT,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
        *mut POINT,
    ) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, SIZE) -> windows_core::HRESULT,
    Scroll: usize,
    ResumeDraw: usize,
    SuspendDraw: usize,
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop {}
windows_core::imp::define_interface!(
    ICompositionEasingFunction,
    ICompositionEasingFunction_Vtbl,
    0x5145e356_bf79_4ea8_8cc2_6b5b472e6c9a
);
impl windows_core::RuntimeType for ICompositionEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionEasingFunction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionEllipseGeometry,
    ICompositionEllipseGeometry_Vtbl,
    0x4801f884_f6ad_4b93_afa9_897b64e57b1f
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
    0xe985217c_6a17_4207_abd8_5fd3dd612a9d
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
    ICompositionGraphicsDevice,
    ICompositionGraphicsDevice_Vtbl,
    0xfb22c6e1_80a2_4667_9936_dbeaf6eefe95
);
impl windows_core::RuntimeType for ICompositionGraphicsDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionGraphicsDevice {
    pub(crate) fn CreateDrawingSurface(
        &self,
        sizepixels: Size,
        pixelformat: DirectXPixelFormat,
        alphamode: DirectXAlphaMode,
    ) -> windows_core::Result<CompositionDrawingSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDrawingSurface)(
                windows_core::Interface::as_raw(self),
                sizepixels,
                pixelformat,
                alphamode,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositionGraphicsDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDrawingSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Size,
        DirectXPixelFormat,
        DirectXAlphaMode,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionNineGridBrush,
    ICompositionNineGridBrush_Vtbl,
    0xf25154e4_bc8c_4be7_b80f_8685b83c0186
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
    0xbcb4ad45_7609_4550_934f_16002a68fded
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
    Dispatcher: usize,
    Properties: usize,
    pub StartAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionObject2,
    ICompositionObject2_Vtbl,
    0xef874ea1_5cff_4b68_9e30_a1519d08ba03
);
impl windows_core::RuntimeType for ICompositionObject2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionObject2 {
    pub(crate) fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ImplicitAnimationCollection>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetImplicitAnimations)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionObject2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Comment: usize,
    SetComment: usize,
    ImplicitAnimations: usize,
    pub SetImplicitAnimations: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionScopedBatch,
    ICompositionScopedBatch_Vtbl,
    0x0d00dad0_fb07_46fd_8c72_6280d1a3d1dd
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
    0xb47ce2f7_9a88_42c4_9e87_2e500ca8688c
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
    0x401b61bb_0007_4363_b1f3_6bcc003fb83e
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
    ICompositionSurface,
    ICompositionSurface_Vtbl,
    0x1527540d_42c7_47a6_a408_668f79a90dfb
);
impl windows_core::RuntimeType for ICompositionSurface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ICompositionSurface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for ICompositionSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurface";
}
pub trait ICompositionSurface_Impl: windows_core::IUnknownImpl {}
impl ICompositionSurface_Vtbl {
    pub const fn new<Identity: ICompositionSurface_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionSurface, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSurface as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ICompositionSurface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionSurfaceBrush,
    ICompositionSurfaceBrush_Vtbl,
    0xad016d79_1e4c_4c0d_9c29_83338c87c162
);
impl windows_core::RuntimeType for ICompositionSurfaceBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionSurfaceBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionTarget,
    ICompositionTarget_Vtbl,
    0xa1bea8ba_d726_4663_8129_6b5e7927ffa6
);
impl windows_core::RuntimeType for ICompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionTarget {
    pub(crate) fn Root(&self) -> windows_core::Result<Visual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Root)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Visual>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetRoot)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompositionTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Root: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositor,
    ICompositor_Vtbl,
    0xb403ca50_7f8c_4e83_985f_cc45060036d8
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
    pub(crate) fn CreateCubicBezierEasingFunction(
        &self,
        controlpoint1: windows_numerics::Vector2,
        controlpoint2: windows_numerics::Vector2,
    ) -> windows_core::Result<CubicBezierEasingFunction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCubicBezierEasingFunction)(
                windows_core::Interface::as_raw(self),
                controlpoint1,
                controlpoint2,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateLinearEasingFunction(&self) -> windows_core::Result<LinearEasingFunction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearEasingFunction)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn CreateScalarKeyFrameAnimation(
        &self,
    ) -> windows_core::Result<ScalarKeyFrameAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScalarKeyFrameAnimation)(
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
    pub(crate) fn CreateSurfaceBrushWithSurface<P0>(
        &self,
        surface: P0,
    ) -> windows_core::Result<CompositionSurfaceBrush>
    where
        P0: windows_core::Param<ICompositionSurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceBrushWithSurface)(
                windows_core::Interface::as_raw(self),
                surface.param().abi(),
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
    pub CreateCubicBezierEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
        windows_numerics::Vector2,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateEffectFactory: usize,
    CreateEffectFactoryWithProperties: usize,
    CreateExpressionAnimation: usize,
    CreateExpressionAnimationWithExpression: usize,
    CreateInsetClip: usize,
    CreateInsetClipWithInsets: usize,
    pub CreateLinearEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreatePropertySet: usize,
    CreateQuaternionKeyFrameAnimation: usize,
    pub CreateScalarKeyFrameAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub CreateSurfaceBrushWithSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateTargetForCurrentView: usize,
    CreateVector2KeyFrameAnimation: usize,
    pub CreateVector3KeyFrameAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositor2,
    ICompositor2_Vtbl,
    0x735081dc_5e24_45da_a38f_e32cc349a9a0
);
impl windows_core::RuntimeType for ICompositor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositor2 {
    pub(crate) fn CreateImplicitAnimationCollection(
        &self,
    ) -> windows_core::Result<ImplicitAnimationCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImplicitAnimationCollection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub CreateImplicitAnimationCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    0x48ea31ad_7fcd_4076_a79c_90cc4b852c9b
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
    ICompositorDesktopInterop,
    ICompositorDesktopInterop_Vtbl,
    0x29e691fa_4567_4dca_b319_d0f207eb6807
);
windows_core::imp::interface_hierarchy!(ICompositorDesktopInterop, windows_core::IUnknown);
impl ICompositorDesktopInterop {
    pub(crate) unsafe fn CreateDesktopWindowTarget(
        &self,
        hwndtarget: HWND,
        istopmost: bool,
    ) -> windows_core::Result<IDesktopWindowTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDesktopWindowTarget)(
                windows_core::Interface::as_raw(self),
                hwndtarget,
                istopmost.into(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositorDesktopInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    EnsureOnThread: usize,
}
impl windows_core::RuntimeName for ICompositorDesktopInterop {}
windows_core::imp::define_interface!(
    ICompositorInterop,
    ICompositorInterop_Vtbl,
    0x25297d5c_3ad4_4c9c_b5cf_e36a38512330
);
windows_core::imp::interface_hierarchy!(ICompositorInterop, windows_core::IUnknown);
impl ICompositorInterop {
    pub(crate) unsafe fn CreateGraphicsDevice<P0>(
        &self,
        renderingdevice: P0,
    ) -> windows_core::Result<ICompositionGraphicsDevice>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGraphicsDevice)(
                windows_core::Interface::as_raw(self),
                renderingdevice.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICompositorInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateCompositionSurfaceForHandle: usize,
    CreateCompositionSurfaceForSwapChain: usize,
    pub CreateGraphicsDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for ICompositorInterop {}
windows_core::imp::define_interface!(
    IContainerVisual,
    IContainerVisual_Vtbl,
    0x02f6bc74_ed20_4773_afe6_d49b4a93db32
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
    ICubicBezierEasingFunction,
    ICubicBezierEasingFunction_Vtbl,
    0x32350666_c1e8_44f9_96b8_c98acf0ae698
);
impl windows_core::RuntimeType for ICubicBezierEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICubicBezierEasingFunction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDesktopWindowTarget,
    IDesktopWindowTarget_Vtbl,
    0x6329d6ca_3366_490e_9db3_25312929ac51
);
impl windows_core::RuntimeType for IDesktopWindowTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDesktopWindowTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDispatcherQueueController,
    IDispatcherQueueController_Vtbl,
    0x22f34e66_50db_4e36_a98d_61c01b384d20
);
impl windows_core::RuntimeType for IDispatcherQueueController {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDispatcherQueueController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IImplicitAnimationCollection,
    IImplicitAnimationCollection_Vtbl,
    0x0598a3ff_0a92_4c9d_a427_b25519250dbf
);
impl windows_core::RuntimeType for IImplicitAnimationCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImplicitAnimationCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IKeyFrameAnimation,
    IKeyFrameAnimation_Vtbl,
    0x126e7f22_3ae9_4540_9a8a_deae8a4a4a84
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
    pub(crate) fn InsertExpressionKeyFrameWithEasingFunction<P2>(
        &self,
        normalizedprogresskey: f32,
        value: &str,
        easingfunction: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<CompositionEasingFunction>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertExpressionKeyFrameWithEasingFunction)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
                easingfunction.param().abi(),
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
    KeyFrameCount: usize,
    StopBehavior: usize,
    SetStopBehavior: usize,
    InsertExpressionKeyFrame: usize,
    pub InsertExpressionKeyFrameWithEasingFunction:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            f32,
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ILinearEasingFunction,
    ILinearEasingFunction_Vtbl,
    0x9400975a_c7a6_46b3_acf7_1a268a0a117d
);
impl windows_core::RuntimeType for ILinearEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILinearEasingFunction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IScalarKeyFrameAnimation,
    IScalarKeyFrameAnimation_Vtbl,
    0xae288fa9_252c_4b95_a725_bf85e38000a1
);
impl windows_core::RuntimeType for IScalarKeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IScalarKeyFrameAnimation {
    pub(crate) fn InsertKeyFrameWithEasingFunction<P2>(
        &self,
        normalizedprogresskey: f32,
        value: f32,
        easingfunction: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<CompositionEasingFunction>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrameWithEasingFunction)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
                easingfunction.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IScalarKeyFrameAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    InsertKeyFrame: usize,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IShapeVisual,
    IShapeVisual_Vtbl,
    0xf2bd13c3_ba7e_4b0f_9126_ffb7536b8176
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
    0x08e05581_1ad1_4f97_9757_402d76e4233b
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
windows_core::imp::define_interface!(
    IVector3KeyFrameAnimation,
    IVector3KeyFrameAnimation_Vtbl,
    0xc8039daa_a281_43c2_a73d_b68e3c533c40
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
    pub(crate) fn InsertKeyFrameWithEasingFunction<P2>(
        &self,
        normalizedprogresskey: f32,
        value: windows_numerics::Vector3,
        easingfunction: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<CompositionEasingFunction>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrameWithEasingFunction)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
                easingfunction.param().abi(),
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
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        windows_numerics::Vector3,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IVisual,
    IVisual_Vtbl,
    0x117e202d_a859_4c89_873b_c2aa566788e3
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
    pub(crate) fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
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
    pub Scale: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
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
    0x3052b611_56c3_4c3e_8bf3_f6e1ad473f06
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
    0x8b745505_fd3e_4a98_84a8_e949468c6bcb
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
pub struct ImplicitAnimationCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ImplicitAnimationCollection,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ImplicitAnimationCollection, CompositionObject);
impl windows_core::RuntimeType for ImplicitAnimationCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImplicitAnimationCollection>();
}
unsafe impl windows_core::Interface for ImplicitAnimationCollection {
    type Vtable = <IImplicitAnimationCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImplicitAnimationCollection as windows_core::Interface>::IID;
}
impl core::ops::Deref for ImplicitAnimationCollection {
    type Target = IImplicitAnimationCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ImplicitAnimationCollection {
    const NAME: &'static str = "Windows.UI.Composition.ImplicitAnimationCollection";
}
unsafe impl Send for ImplicitAnimationCollection {}
unsafe impl Sync for ImplicitAnimationCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    KeyFrameAnimation,
    ICompositionAnimationBase,
    CompositionAnimation,
    CompositionObject
);
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
    const NAME: &'static str = "Windows.UI.Composition.KeyFrameAnimation";
}
unsafe impl Send for KeyFrameAnimation {}
unsafe impl Sync for KeyFrameAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinearEasingFunction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    LinearEasingFunction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    LinearEasingFunction,
    CompositionEasingFunction,
    CompositionObject
);
impl windows_core::RuntimeType for LinearEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ILinearEasingFunction>();
}
unsafe impl windows_core::Interface for LinearEasingFunction {
    type Vtable = <ILinearEasingFunction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILinearEasingFunction as windows_core::Interface>::IID;
}
impl core::ops::Deref for LinearEasingFunction {
    type Target = ILinearEasingFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for LinearEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.LinearEasingFunction";
}
unsafe impl Send for LinearEasingFunction {}
unsafe impl Sync for LinearEasingFunction {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScalarKeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScalarKeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ScalarKeyFrameAnimation,
    ICompositionAnimationBase,
    KeyFrameAnimation,
    CompositionAnimation,
    CompositionObject
);
impl windows_core::RuntimeType for ScalarKeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScalarKeyFrameAnimation>();
}
unsafe impl windows_core::Interface for ScalarKeyFrameAnimation {
    type Vtable = <IScalarKeyFrameAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScalarKeyFrameAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for ScalarKeyFrameAnimation {
    type Target = IScalarKeyFrameAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ScalarKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ScalarKeyFrameAnimation";
}
unsafe impl Send for ScalarKeyFrameAnimation {}
unsafe impl Sync for ScalarKeyFrameAnimation {}
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
    const NAME: &'static str = "Windows.UI.Composition.ShapeVisual";
}
unsafe impl Send for ShapeVisual {}
unsafe impl Sync for ShapeVisual {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl windows_core::TypeKind for Size {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Size {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
}
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
    const NAME: &'static str = "Windows.UI.Composition.SpriteVisual";
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
    ICompositionAnimationBase,
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
    const NAME: &'static str = "Windows.UI.Composition.Vector3KeyFrameAnimation";
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
    const NAME: &'static str = "Windows.UI.Composition.Visual";
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
    const NAME: &'static str = "Windows.UI.Composition.VisualCollection";
}
unsafe impl Send for VisualCollection {}
unsafe impl Sync for VisualCollection {}
