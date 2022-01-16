#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IInkAnalysisNode_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Kind(&mut self) -> ::windows::core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(&mut self) -> ::windows::core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IInkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IInkAnalysisNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisNode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisNode_Vtbl {
        unsafe extern "system" fn Id<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRect<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotatedBoundingRect<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotatedBoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeIds<Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisNode, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            BoundingRect: BoundingRect::<Impl, IMPL_OFFSET>,
            RotatedBoundingRect: RotatedBoundingRect::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            GetStrokeIds: GetStrokeIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisNode as ::windows::core::Interface>::IID
    }
}
pub trait IInkAnalyzerFactory_Impl: Sized {
    fn CreateAnalyzer(&mut self) -> ::windows::core::Result<InkAnalyzer>;
}
impl ::windows::core::RuntimeName for IInkAnalyzerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
}
impl IInkAnalyzerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalyzerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalyzerFactory_Vtbl {
        unsafe extern "system" fn CreateAnalyzer<Impl: IInkAnalyzerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnalyzer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalyzerFactory, BASE_OFFSET>(),
            CreateAnalyzer: CreateAnalyzer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalyzerFactory as ::windows::core::Interface>::IID
    }
}
