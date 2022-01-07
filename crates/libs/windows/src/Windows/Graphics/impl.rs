pub trait IGeometrySource2DImpl: Sized {}
impl ::windows::core::RuntimeName for IGeometrySource2D {
    const NAME: &'static str = "Windows.Graphics.IGeometrySource2D";
}
impl IGeometrySource2DVtbl {
    pub const fn new<Impl: IGeometrySource2DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeometrySource2DVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeometrySource2D>, base.5)
    }
}
