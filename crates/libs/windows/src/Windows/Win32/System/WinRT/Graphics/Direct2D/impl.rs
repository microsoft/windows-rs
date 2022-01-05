pub trait IGeometrySource2DInteropImpl: Sized {
    fn GetGeometry();
    fn TryGetGeometryUsingFactory();
}
pub trait IGraphicsEffectD2D1InteropImpl: Sized {
    fn GetEffectId();
    fn GetNamedPropertyMapping();
    fn GetPropertyCount();
    fn GetProperty();
    fn GetSource();
    fn GetSourceCount();
}
