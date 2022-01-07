pub trait IGeometrySource2DInteropImpl: Sized {
    fn GetGeometry();
    fn TryGetGeometryUsingFactory();
}
impl ::windows::core::RuntimeName for IGeometrySource2DInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Graphics.Direct2D.IGeometrySource2DInterop";
}
impl IGeometrySource2DInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeometrySource2DInteropImpl, const OFFSET: isize>() -> IGeometrySource2DInteropVtbl {
        unsafe extern "system" fn GetGeometry<Impl: IGeometrySource2DInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometry(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Impl: IGeometrySource2DInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetGeometryUsingFactory(&*(&factory as *const <super::super::super::super::Graphics::Direct2D::ID2D1Factory as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Direct2D::ID2D1Factory as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGeometrySource2DInterop>, ::windows::core::GetTrustLevel, GetGeometry::<Impl, OFFSET>, TryGetGeometryUsingFactory::<Impl, OFFSET>)
    }
}
pub trait IGraphicsEffectD2D1InteropImpl: Sized {
    fn GetEffectId();
    fn GetNamedPropertyMapping();
    fn GetPropertyCount();
    fn GetProperty();
    fn GetSource();
    fn GetSourceCount();
}
impl ::windows::core::RuntimeName for IGraphicsEffectD2D1Interop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Graphics.Direct2D.IGraphicsEffectD2D1Interop";
}
impl IGraphicsEffectD2D1InteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>() -> IGraphicsEffectD2D1InteropVtbl {
        unsafe extern "system" fn GetEffectId<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectId(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedPropertyMapping(&*(&name as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&mapping)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyCount<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(index, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, source: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource(index, ::core::mem::transmute_copy(&source)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceCount<Impl: IGraphicsEffectD2D1InteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsEffectD2D1Interop>, ::windows::core::GetTrustLevel, GetEffectId::<Impl, OFFSET>, GetNamedPropertyMapping::<Impl, OFFSET>, GetPropertyCount::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, GetSource::<Impl, OFFSET>, GetSourceCount::<Impl, OFFSET>)
    }
}
