pub trait ID2D1SimplifiedGeometrySink_Impl: Sized {
    fn SetFillMode(&self, fillmode: D2D1_FILL_MODE);
    fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT);
    fn BeginFigure(&self, startpoint: &D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN);
    fn AddLines(&self, points: *const D2D_POINT_2F, pointscount: u32);
    fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32);
    fn EndFigure(&self, figureend: D2D1_FIGURE_END);
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ID2D1SimplifiedGeometrySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>() -> ID2D1SimplifiedGeometrySink_Vtbl {
        unsafe extern "system" fn SetFillMode<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFillMode(::core::mem::transmute_copy(&fillmode))
        }
        unsafe extern "system" fn SetSegmentFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSegmentFlags(::core::mem::transmute_copy(&vertexflags))
        }
        unsafe extern "system" fn BeginFigure<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginFigure(::core::mem::transmute(&startpoint), ::core::mem::transmute_copy(&figurebegin))
        }
        unsafe extern "system" fn AddLines<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *const D2D_POINT_2F, pointscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLines(::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointscount))
        }
        unsafe extern "system" fn AddBeziers<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddBeziers(::core::mem::transmute_copy(&beziers), ::core::mem::transmute_copy(&bezierscount))
        }
        unsafe extern "system" fn EndFigure<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndFigure(::core::mem::transmute_copy(&figureend))
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetFillMode: SetFillMode::<Identity, Impl, OFFSET>,
            SetSegmentFlags: SetSegmentFlags::<Identity, Impl, OFFSET>,
            BeginFigure: BeginFigure::<Identity, Impl, OFFSET>,
            AddLines: AddLines::<Identity, Impl, OFFSET>,
            AddBeziers: AddBeziers::<Identity, Impl, OFFSET>,
            EndFigure: EndFigure::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SimplifiedGeometrySink as ::windows::core::Interface>::IID
    }
}
