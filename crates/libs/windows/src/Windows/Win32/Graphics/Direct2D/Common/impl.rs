pub trait ID2D1SimplifiedGeometrySink_Impl: Sized + windows_core::IUnknownImpl {
    fn SetFillMode(&self, fillmode: D2D1_FILL_MODE);
    fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT);
    fn BeginFigure(&self, startpoint: &D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN);
    fn AddLines(&self, points: *const D2D_POINT_2F, pointscount: u32);
    fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32);
    fn EndFigure(&self, figureend: D2D1_FIGURE_END);
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1SimplifiedGeometrySink {}
impl ID2D1SimplifiedGeometrySink_Vtbl {
    pub const fn new<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>() -> ID2D1SimplifiedGeometrySink_Vtbl {
        unsafe extern "system" fn SetFillMode<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::SetFillMode(this, core::mem::transmute_copy(&fillmode))
        }
        unsafe extern "system" fn SetSegmentFlags<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::SetSegmentFlags(this, core::mem::transmute_copy(&vertexflags))
        }
        unsafe extern "system" fn BeginFigure<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::BeginFigure(this, core::mem::transmute(&startpoint), core::mem::transmute_copy(&figurebegin))
        }
        unsafe extern "system" fn AddLines<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const D2D_POINT_2F, pointscount: u32) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::AddLines(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount))
        }
        unsafe extern "system" fn AddBeziers<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::AddBeziers(this, core::mem::transmute_copy(&beziers), core::mem::transmute_copy(&bezierscount))
        }
        unsafe extern "system" fn EndFigure<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::EndFigure(this, core::mem::transmute_copy(&figureend))
        }
        unsafe extern "system" fn Close<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SimplifiedGeometrySink_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFillMode: SetFillMode::<Identity, OFFSET>,
            SetSegmentFlags: SetSegmentFlags::<Identity, OFFSET>,
            BeginFigure: BeginFigure::<Identity, OFFSET>,
            AddLines: AddLines::<Identity, OFFSET>,
            AddBeziers: AddBeziers::<Identity, OFFSET>,
            EndFigure: EndFigure::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
