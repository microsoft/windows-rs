pub trait IGeoshape_Impl: Sized {
    fn GeoshapeType(&mut self) -> ::windows::core::Result<GeoshapeType>;
    fn SpatialReferenceId(&mut self) -> ::windows::core::Result<u32>;
    fn AltitudeReferenceSystem(&mut self) -> ::windows::core::Result<AltitudeReferenceSystem>;
}
impl ::windows::core::RuntimeName for IGeoshape {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoshape";
}
impl IGeoshape_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoshape_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeoshape_Vtbl {
        unsafe extern "system" fn GeoshapeType<Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeoshapeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialReferenceId<Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpatialReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltitudeReferenceSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoshape, BASE_OFFSET>(),
            GeoshapeType: GeoshapeType::<Impl, IMPL_OFFSET>,
            SpatialReferenceId: SpatialReferenceId::<Impl, IMPL_OFFSET>,
            AltitudeReferenceSystem: AltitudeReferenceSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoshape as ::windows::core::Interface>::IID
    }
}
