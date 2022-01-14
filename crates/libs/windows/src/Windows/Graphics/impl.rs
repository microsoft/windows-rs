pub trait IGeometrySource2D_Impl: Sized {}
impl ::windows::core::RuntimeName for IGeometrySource2D {
    const NAME: &'static str = "Windows.Graphics.IGeometrySource2D";
}
impl IGeometrySource2D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometrySource2D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometrySource2D_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeometrySource2D, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometrySource2D as ::windows::core::Interface>::IID
    }
}
