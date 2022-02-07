pub trait IGeoshape_Impl: Sized {
    fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType>;
    fn SpatialReferenceId(&self) -> ::windows::core::Result<u32>;
    fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem>;
}
impl ::windows::core::RuntimeName for IGeoshape {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoshape";
}
impl IGeoshape_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoshape_Impl, const OFFSET: isize>() -> IGeoshape_Vtbl {
        unsafe extern "system" fn GeoshapeType<Identity: ::windows::core::IUnknownImpl, Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GeoshapeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialReferenceId<Identity: ::windows::core::IUnknownImpl, Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpatialReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Identity: ::windows::core::IUnknownImpl, Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoshape, OFFSET>(),
            GeoshapeType: GeoshapeType::<Identity, Impl, OFFSET>,
            SpatialReferenceId: SpatialReferenceId::<Identity, Impl, OFFSET>,
            AltitudeReferenceSystem: AltitudeReferenceSystem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoshape as ::windows::core::Interface>::IID
    }
}
