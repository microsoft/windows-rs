#[cfg(feature = "Foundation_Collections")]
pub trait IInkAnalysisNode_Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IInkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
}
#[cfg(feature = "Foundation_Collections")]
impl IInkAnalysisNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>() -> IInkAnalysisNode_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotatedBoundingRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RotatedBoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeIds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeIds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisNode, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Kind: Kind::<Identity, Impl, OFFSET>,
            BoundingRect: BoundingRect::<Identity, Impl, OFFSET>,
            RotatedBoundingRect: RotatedBoundingRect::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            GetStrokeIds: GetStrokeIds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisNode as ::windows::core::Interface>::IID
    }
}
pub trait IInkAnalyzerFactory_Impl: Sized {
    fn CreateAnalyzer(&self) -> ::windows::core::Result<InkAnalyzer>;
}
impl ::windows::core::RuntimeName for IInkAnalyzerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
}
impl IInkAnalyzerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalyzerFactory_Impl, const OFFSET: isize>() -> IInkAnalyzerFactory_Vtbl {
        unsafe extern "system" fn CreateAnalyzer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalyzerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAnalyzer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalyzerFactory, OFFSET>(),
            CreateAnalyzer: CreateAnalyzer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalyzerFactory as ::windows::core::Interface>::IID
    }
}
