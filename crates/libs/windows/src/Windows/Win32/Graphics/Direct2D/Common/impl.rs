pub trait ID2D1SimplifiedGeometrySinkImpl: Sized {
    fn SetFillMode();
    fn SetSegmentFlags();
    fn BeginFigure();
    fn AddLines();
    fn AddBeziers();
    fn EndFigure();
    fn Close();
}
impl ::windows::core::RuntimeName for ID2D1SimplifiedGeometrySink {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.Common.ID2D1SimplifiedGeometrySink";
}
impl ID2D1SimplifiedGeometrySinkVtbl {
    pub const fn new<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SimplifiedGeometrySinkVtbl {
        unsafe extern "system" fn SetFillMode<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFillMode(fillmode).into()
        }
        unsafe extern "system" fn SetSegmentFlags<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSegmentFlags(vertexflags).into()
        }
        unsafe extern "system" fn BeginFigure<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BeginFigure(&*(&startpoint as *const <D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), figurebegin).into()
        }
        unsafe extern "system" fn AddLines<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *const D2D_POINT_2F, pointscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddLines(&*(&points as *const <D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), pointscount).into()
        }
        unsafe extern "system" fn AddBeziers<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddBeziers(&*(&beziers as *const <D2D1_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType), bezierscount).into()
        }
        unsafe extern "system" fn EndFigure<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EndFigure(figureend).into()
        }
        unsafe extern "system" fn Close<Impl: ID2D1SimplifiedGeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SimplifiedGeometrySink>, base.5, SetFillMode::<Impl, OFFSET>, SetSegmentFlags::<Impl, OFFSET>, BeginFigure::<Impl, OFFSET>, AddLines::<Impl, OFFSET>, AddBeziers::<Impl, OFFSET>, EndFigure::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
