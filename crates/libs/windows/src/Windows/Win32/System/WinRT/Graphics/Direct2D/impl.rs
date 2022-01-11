#[cfg(feature = "Win32_Graphics_Direct2D")]
pub trait IGeometrySource2DInteropImpl: Sized {
    fn GetGeometry();
    fn TryGetGeometryUsingFactory();
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl IGeometrySource2DInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometrySource2DInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeometrySource2DInteropVtbl {
        unsafe extern "system" fn GetGeometry<Impl: IGeometrySource2DInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Impl: IGeometrySource2DInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGeometry::<Impl, IMPL_OFFSET>, TryGetGeometryUsingFactory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeometrySource2DInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects", feature = "Win32_Foundation"))]
pub trait IGraphicsEffectD2D1InteropImpl: Sized {
    fn GetEffectId();
    fn GetNamedPropertyMapping();
    fn GetPropertyCount();
    fn GetProperty();
    fn GetSource();
    fn GetSourceCount();
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects", feature = "Win32_Foundation"))]
impl IGraphicsEffectD2D1InteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectD2D1InteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsEffectD2D1InteropVtbl {
        unsafe extern "system" fn GetEffectId<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyCount<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSource<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, source: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceCount<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetEffectId::<Impl, IMPL_OFFSET>, GetNamedPropertyMapping::<Impl, IMPL_OFFSET>, GetPropertyCount::<Impl, IMPL_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>, GetSource::<Impl, IMPL_OFFSET>, GetSourceCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffectD2D1Interop as ::windows::core::Interface>::IID
    }
}
