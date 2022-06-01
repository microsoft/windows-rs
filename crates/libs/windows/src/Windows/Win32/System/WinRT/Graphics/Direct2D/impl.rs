#[cfg(feature = "Win32_Graphics_Direct2D")]
pub trait IGeometrySource2DInterop_Impl: Sized {
    fn GetGeometry(&self) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
    fn TryGetGeometryUsingFactory(&self, factory: &::core::option::Option<super::super::super::super::Graphics::Direct2D::ID2D1Factory>) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl ::windows::core::RuntimeName for IGeometrySource2DInterop {}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl IGeometrySource2DInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGeometrySource2DInterop_Impl, const OFFSET: isize>() -> IGeometrySource2DInterop_Vtbl {
        unsafe extern "system" fn GetGeometry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGeometrySource2DInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGeometrySource2DInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryGetGeometryUsingFactory(::core::mem::transmute(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetGeometry: GetGeometry::<Identity, Impl, OFFSET>,
            TryGetGeometryUsingFactory: TryGetGeometryUsingFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometrySource2DInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
pub trait IGraphicsEffectD2D1Interop_Impl: Sized {
    fn GetEffectId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNamedPropertyMapping(&self, name: &::windows::core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::Result<()>;
    fn GetPropertyCount(&self) -> ::windows::core::Result<u32>;
    fn GetProperty(&self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Foundation::IPropertyValue>;
    fn GetSource(&self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn GetSourceCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
impl ::windows::core::RuntimeName for IGraphicsEffectD2D1Interop {}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
impl IGraphicsEffectD2D1Interop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>() -> IGraphicsEffectD2D1Interop_Vtbl {
        unsafe extern "system" fn GetEffectId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEffectId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNamedPropertyMapping(::core::mem::transmute(&name), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&mapping)).into()
        }
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, source: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSource(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(source, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetEffectId: GetEffectId::<Identity, Impl, OFFSET>,
            GetNamedPropertyMapping: GetNamedPropertyMapping::<Identity, Impl, OFFSET>,
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffectD2D1Interop as ::windows::core::Interface>::IID
    }
}
