pub trait IGeometrySource2DImpl: Sized {}
impl ::windows::core::RuntimeName for IGeometrySource2D {
    const NAME: &'static str = "Windows.Graphics.IGeometrySource2D";
}
impl IGeometrySource2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometrySource2DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometrySource2DVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGeometrySource2D>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometrySource2D as ::windows::core::Interface>::IID
    }
}
