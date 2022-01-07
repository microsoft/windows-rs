#[cfg(feature = "implement_exclusive")]
pub trait IPalmRejectionDelayZonePreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview";
}
#[cfg(feature = "implement_exclusive")]
impl IPalmRejectionDelayZonePreviewVtbl {
    pub const fn new<Impl: IPalmRejectionDelayZonePreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPalmRejectionDelayZonePreviewVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPalmRejectionDelayZonePreview>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPalmRejectionDelayZonePreviewStaticsImpl: Sized {
    fn CreateForVisual(&self, inputpanelvisual: &::core::option::Option<super::super::super::Composition::Visual>, inputpanelrect: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>;
    fn CreateForVisualWithViewportClip(&self, inputpanelvisual: &::core::option::Option<super::super::super::Composition::Visual>, inputpanelrect: &super::super::super::super::Foundation::Rect, viewportvisual: &::core::option::Option<super::super::super::Composition::Visual>, viewportrect: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPalmRejectionDelayZonePreviewStatics {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPalmRejectionDelayZonePreviewStaticsVtbl {
    pub const fn new<Impl: IPalmRejectionDelayZonePreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPalmRejectionDelayZonePreviewStaticsVtbl {
        unsafe extern "system" fn CreateForVisual<Impl: IPalmRejectionDelayZonePreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputpanelvisual: ::windows::core::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForVisual(&*(&inputpanelvisual as *const <super::super::super::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::Visual as ::windows::core::DefaultType>::DefaultType), &*(&inputpanelrect as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForVisualWithViewportClip<Impl: IPalmRejectionDelayZonePreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputpanelvisual: ::windows::core::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: ::windows::core::RawPtr, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForVisualWithViewportClip(
                &*(&inputpanelvisual as *const <super::super::super::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::Visual as ::windows::core::DefaultType>::DefaultType),
                &*(&inputpanelrect as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                &*(&viewportvisual as *const <super::super::super::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::Visual as ::windows::core::DefaultType>::DefaultType),
                &*(&viewportrect as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPalmRejectionDelayZonePreviewStatics>, base.5, CreateForVisual::<Impl, OFFSET>, CreateForVisualWithViewportClip::<Impl, OFFSET>)
    }
}
