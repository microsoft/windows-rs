#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IScrollSnapPointsInfo_Impl: Sized {
    fn AreHorizontalSnapPointsRegular(&mut self) -> ::windows::core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&mut self) -> ::windows::core::Result<bool>;
    fn HorizontalSnapPointsChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetIrregularSnapPoints(&mut self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(&mut self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IScrollSnapPointsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollSnapPointsInfo_Vtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreHorizontalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreVerticalSnapPointsRegular<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreVerticalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalSnapPointsChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHorizontalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalSnapPointsChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVerticalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIrregularSnapPoints(orientation, alignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularSnapPoints<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegularSnapPoints(orientation, alignment, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollSnapPointsInfo, BASE_OFFSET>(),
            AreHorizontalSnapPointsRegular: AreHorizontalSnapPointsRegular::<Impl, IMPL_OFFSET>,
            AreVerticalSnapPointsRegular: AreVerticalSnapPointsRegular::<Impl, IMPL_OFFSET>,
            HorizontalSnapPointsChanged: HorizontalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            RemoveHorizontalSnapPointsChanged: RemoveHorizontalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            VerticalSnapPointsChanged: VerticalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            RemoveVerticalSnapPointsChanged: RemoveVerticalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            GetIrregularSnapPoints: GetIrregularSnapPoints::<Impl, IMPL_OFFSET>,
            GetRegularSnapPoints: GetRegularSnapPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollSnapPointsInfo as ::windows::core::Interface>::IID
    }
}
