#[cfg(feature = "Win32_Graphics_Direct2D")]
pub trait IGeometrySource2DInterop_Impl: Sized {
    fn GetGeometry(&mut self) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
    fn TryGetGeometryUsingFactory(&mut self, factory: &::core::option::Option<super::super::super::super::Graphics::Direct2D::ID2D1Factory>) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl IGeometrySource2DInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometrySource2DInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometrySource2DInterop_Vtbl {
        unsafe extern "system" fn GetGeometry<Impl: IGeometrySource2DInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Impl: IGeometrySource2DInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetGeometryUsingFactory(::core::mem::transmute(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGeometry: GetGeometry::<Impl, IMPL_OFFSET>,
            TryGetGeometryUsingFactory: TryGetGeometryUsingFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometrySource2DInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects", feature = "Win32_Foundation"))]
pub trait IGraphicsEffectD2D1Interop_Impl: Sized {
    fn GetEffectId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNamedPropertyMapping(&mut self, name: super::super::super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::Result<()>;
    fn GetPropertyCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetProperty(&mut self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Foundation::IPropertyValue>;
    fn GetSource(&mut self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn GetSourceCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects", feature = "Win32_Foundation"))]
impl IGraphicsEffectD2D1Interop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectD2D1Interop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsEffectD2D1Interop_Vtbl {
        unsafe extern "system" fn GetEffectId<Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectId() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNamedPropertyMapping(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&mapping)).into()
        }
        unsafe extern "system" fn GetPropertyCount<Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, source: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *source = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceCount<Impl: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEffectId: GetEffectId::<Impl, IMPL_OFFSET>,
            GetNamedPropertyMapping: GetNamedPropertyMapping::<Impl, IMPL_OFFSET>,
            GetPropertyCount: GetPropertyCount::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetSource: GetSource::<Impl, IMPL_OFFSET>,
            GetSourceCount: GetSourceCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffectD2D1Interop as ::windows::core::Interface>::IID
    }
}
