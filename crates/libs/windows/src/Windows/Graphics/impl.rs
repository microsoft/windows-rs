pub trait IGeometrySource2D_Impl: Sized {}
impl ::windows::core::RuntimeName for IGeometrySource2D {
    const NAME: &'static str = "Windows.Graphics.IGeometrySource2D";
}
impl IGeometrySource2D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGeometrySource2D_Impl, const OFFSET: isize>() -> IGeometrySource2D_Vtbl {
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IGeometrySource2D, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometrySource2D as ::windows::core::Interface>::IID
    }
}
