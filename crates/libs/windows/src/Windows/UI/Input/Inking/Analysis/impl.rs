#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisInkBullet_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisInkBullet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisInkBullet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisInkBullet_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisInkBullet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisInkBullet, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisInkBullet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisInkDrawing_Impl: Sized + IInkAnalysisNode_Impl {
    fn DrawingKind(&mut self) -> ::windows::core::Result<InkAnalysisDrawingKind>;
    fn Center(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Points(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisInkDrawing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisInkDrawing_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisInkDrawing_Vtbl {
        unsafe extern "system" fn DrawingKind<Impl: IInkAnalysisInkDrawing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisDrawingKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawingKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Center<Impl: IInkAnalysisInkDrawing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Points<Impl: IInkAnalysisInkDrawing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisInkDrawing, BASE_OFFSET>(),
            DrawingKind: DrawingKind::<Impl, IMPL_OFFSET>,
            Center: Center::<Impl, IMPL_OFFSET>,
            Points: Points::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisInkDrawing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisInkWord_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TextAlternates(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisInkWord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisInkWord_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisInkWord_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisInkWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextAlternates<Impl: IInkAnalysisInkWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisInkWord, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
            TextAlternates: TextAlternates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisInkWord as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisLine_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndentLevel(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisLine";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisLine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisLine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisLine_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisLine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndentLevel<Impl: IInkAnalysisLine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisLine, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
            IndentLevel: IndentLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisLine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisListItem_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisListItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisListItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisListItem_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisListItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisListItem, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisListItem as ::windows::core::Interface>::IID
    }
}
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisParagraph_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisParagraph_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisParagraph_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisParagraph_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisParagraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisParagraph, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisParagraph as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<InkAnalysisStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisResult";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IInkAnalysisResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisRoot_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FindNodes(&mut self, nodekind: InkAnalysisNodeKind) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisRoot_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisRoot_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNodes<Impl: IInkAnalysisRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodekind: InkAnalysisNodeKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNodes(nodekind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisRoot, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
            FindNodes: FindNodes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisWritingRegion_Impl: Sized + IInkAnalysisNode_Impl {
    fn RecognizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisWritingRegion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisWritingRegion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisWritingRegion_Vtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisWritingRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalysisWritingRegion, BASE_OFFSET>(),
            RecognizedText: RecognizedText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisWritingRegion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalyzer_Impl: Sized {
    fn AnalysisRoot(&mut self) -> ::windows::core::Result<InkAnalysisRoot>;
    fn IsAnalyzing(&mut self) -> ::windows::core::Result<bool>;
    fn AddDataForStroke(&mut self, stroke: &::core::option::Option<super::InkStroke>) -> ::windows::core::Result<()>;
    fn AddDataForStrokes(&mut self, strokes: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>) -> ::windows::core::Result<()>;
    fn ClearDataForAllStrokes(&mut self) -> ::windows::core::Result<()>;
    fn RemoveDataForStroke(&mut self, strokeid: u32) -> ::windows::core::Result<()>;
    fn RemoveDataForStrokes(&mut self, strokeids: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<()>;
    fn ReplaceDataForStroke(&mut self, stroke: &::core::option::Option<super::InkStroke>) -> ::windows::core::Result<()>;
    fn SetStrokeDataKind(&mut self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::Result<()>;
    fn AnalyzeAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalyzer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalyzer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalyzer_Vtbl {
        unsafe extern "system" fn AnalysisRoot<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalysisRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAnalyzing<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnalyzing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDataForStroke<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDataForStroke(&*(&stroke as *const <super::InkStroke as ::windows::core::Abi>::Abi as *const <super::InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddDataForStrokes<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDataForStrokes(&*(&strokes as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkStroke> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkStroke> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearDataForAllStrokes<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDataForAllStrokes().into()
        }
        unsafe extern "system" fn RemoveDataForStroke<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataForStroke(strokeid).into()
        }
        unsafe extern "system" fn RemoveDataForStrokes<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeids: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataForStrokes(&*(&strokeids as *const <super::super::super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReplaceDataForStroke<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceDataForStroke(&*(&stroke as *const <super::InkStroke as ::windows::core::Abi>::Abi as *const <super::InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStrokeDataKind<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDataKind(strokeid, strokekind).into()
        }
        unsafe extern "system" fn AnalyzeAsync<Impl: IInkAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkAnalyzer, BASE_OFFSET>(),
            AnalysisRoot: AnalysisRoot::<Impl, IMPL_OFFSET>,
            IsAnalyzing: IsAnalyzing::<Impl, IMPL_OFFSET>,
            AddDataForStroke: AddDataForStroke::<Impl, IMPL_OFFSET>,
            AddDataForStrokes: AddDataForStrokes::<Impl, IMPL_OFFSET>,
            ClearDataForAllStrokes: ClearDataForAllStrokes::<Impl, IMPL_OFFSET>,
            RemoveDataForStroke: RemoveDataForStroke::<Impl, IMPL_OFFSET>,
            RemoveDataForStrokes: RemoveDataForStrokes::<Impl, IMPL_OFFSET>,
            ReplaceDataForStroke: ReplaceDataForStroke::<Impl, IMPL_OFFSET>,
            SetStrokeDataKind: SetStrokeDataKind::<Impl, IMPL_OFFSET>,
            AnalyzeAsync: AnalyzeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalyzer as ::windows::core::Interface>::IID
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
