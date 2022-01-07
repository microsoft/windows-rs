pub trait IGeometrySource2DImpl: Sized {}
impl ::windows::core::RuntimeName for IGeometrySource2D {
    const NAME: &'static str = "Windows.Graphics.IGeometrySource2D";
}
impl IGeometrySource2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometrySource2DImpl, const OFFSET: isize>() -> IGeometrySource2DVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGeometrySource2D>, ::windows::core::GetTrustLevel)
    }
}
