pub trait ID2D1SimplifiedGeometrySinkImpl: Sized {
    fn SetFillMode();
    fn SetSegmentFlags();
    fn BeginFigure();
    fn AddLines();
    fn AddBeziers();
    fn EndFigure();
    fn Close();
}
impl ID2D1SimplifiedGeometrySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SimplifiedGeometrySinkVtbl {
        unsafe extern "system" fn SetFillMode<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSegmentFlags<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginFigure<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddLines<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *const D2D_POINT_2F, pointscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBeziers<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndFigure<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFillMode::<Impl, IMPL_OFFSET>, SetSegmentFlags::<Impl, IMPL_OFFSET>, BeginFigure::<Impl, IMPL_OFFSET>, AddLines::<Impl, IMPL_OFFSET>, AddBeziers::<Impl, IMPL_OFFSET>, EndFigure::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SimplifiedGeometrySink as ::windows::core::Interface>::IID
    }
}
