pub trait IGeometrySource2D_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IGeometrySource2D {
    const NAME: &'static str = "Windows.Graphics.IGeometrySource2D";
}
impl IGeometrySource2D_Vtbl {
    pub const fn new<Identity: IGeometrySource2D_Impl, const OFFSET: isize>() -> IGeometrySource2D_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IGeometrySource2D, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGeometrySource2D as windows_core::Interface>::IID
    }
}
