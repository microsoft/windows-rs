#[cfg(feature = "Foundation")]
pub trait IPointerPointTransform_Impl: Sized {
    fn Inverse(&mut self) -> ::windows::core::Result<IPointerPointTransform>;
    fn TryTransform(&mut self, inpoint: &super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&mut self, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointTransform";
}
#[cfg(feature = "Foundation")]
impl IPointerPointTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerPointTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerPointTransform_Vtbl {
        unsafe extern "system" fn Inverse<Impl: IPointerPointTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<Impl: IPointerPointTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryTransform(&*(&inpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBounds<Impl: IPointerPointTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformBounds(&*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerPointTransform, BASE_OFFSET>(),
            Inverse: Inverse::<Impl, IMPL_OFFSET>,
            TryTransform: TryTransform::<Impl, IMPL_OFFSET>,
            TransformBounds: TransformBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerPointTransform as ::windows::core::Interface>::IID
    }
}
