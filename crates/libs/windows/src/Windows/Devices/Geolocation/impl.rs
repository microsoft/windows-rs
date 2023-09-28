pub trait IGeoshape_Impl: Sized {
    fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType>;
    fn SpatialReferenceId(&self) -> ::windows_core::Result<u32>;
    fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem>;
}
impl ::windows_core::RuntimeName for IGeoshape {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoshape";
}
impl IGeoshape_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: isize>() -> IGeoshape_Vtbl {
        unsafe extern "system" fn GeoshapeType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GeoshapeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialReferenceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpatialReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGeoshape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AltitudeReferenceSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IGeoshape, OFFSET>(),
            GeoshapeType: GeoshapeType::<Identity, Impl, OFFSET>,
            SpatialReferenceId: SpatialReferenceId::<Identity, Impl, OFFSET>,
            AltitudeReferenceSystem: AltitudeReferenceSystem::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IGeoshape as ::windows_core::ComInterface>::IID
    }
}
