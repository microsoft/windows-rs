#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisInkBulletImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisInkBulletVtbl {
    pub const fn new<Impl: IInkAnalysisInkBulletImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisInkBulletVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisInkBulletImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisInkBullet>, base.5, RecognizedText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisInkDrawingImpl: Sized + IInkAnalysisNodeImpl {
    fn DrawingKind(&self) -> ::windows::core::Result<InkAnalysisDrawingKind>;
    fn Center(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Points(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisInkDrawingVtbl {
    pub const fn new<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisInkDrawingVtbl {
        unsafe extern "system" fn DrawingKind<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisDrawingKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawingKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Center<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Points<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Points() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisInkDrawing>, base.5, DrawingKind::<Impl, OFFSET>, Center::<Impl, OFFSET>, Points::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisInkWordImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TextAlternates(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisInkWordVtbl {
    pub const fn new<Impl: IInkAnalysisInkWordImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisInkWordVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisInkWordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextAlternates<Impl: IInkAnalysisInkWordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisInkWord>, base.5, RecognizedText::<Impl, OFFSET>, TextAlternates::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisLineImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndentLevel(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisLine";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisLineVtbl {
    pub const fn new<Impl: IInkAnalysisLineImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisLineVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisLineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndentLevel<Impl: IInkAnalysisLineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisLine>, base.5, RecognizedText::<Impl, OFFSET>, IndentLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisListItemImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisListItemVtbl {
    pub const fn new<Impl: IInkAnalysisListItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisListItemVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisListItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisListItem>, base.5, RecognizedText::<Impl, OFFSET>)
    }
}
pub trait IInkAnalysisNodeImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
impl ::windows::core::RuntimeName for IInkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
}
impl IInkAnalysisNodeVtbl {
    pub const fn new<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisNodeVtbl {
        unsafe extern "system" fn Id<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRect<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotatedBoundingRect<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotatedBoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeIds<Impl: IInkAnalysisNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisNode>, base.5, Id::<Impl, OFFSET>, Kind::<Impl, OFFSET>, BoundingRect::<Impl, OFFSET>, RotatedBoundingRect::<Impl, OFFSET>, Children::<Impl, OFFSET>, Parent::<Impl, OFFSET>, GetStrokeIds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisParagraphImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisParagraphVtbl {
    pub const fn new<Impl: IInkAnalysisParagraphImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisParagraphVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisParagraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisParagraph>, base.5, RecognizedText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<InkAnalysisStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisResult";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisResultVtbl {
    pub const fn new<Impl: IInkAnalysisResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisResultVtbl {
        unsafe extern "system" fn Status<Impl: IInkAnalysisResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisResult>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisRootImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisRootVtbl {
    pub const fn new<Impl: IInkAnalysisRootImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisRootVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNodes<Impl: IInkAnalysisRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodekind: InkAnalysisNodeKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNodes(nodekind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisRoot>, base.5, RecognizedText::<Impl, OFFSET>, FindNodes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisWritingRegionImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalysisWritingRegionVtbl {
    pub const fn new<Impl: IInkAnalysisWritingRegionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalysisWritingRegionVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisWritingRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalysisWritingRegion>, base.5, RecognizedText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalyzerImpl: Sized {
    fn AnalysisRoot(&self) -> ::windows::core::Result<InkAnalysisRoot>;
    fn IsAnalyzing(&self) -> ::windows::core::Result<bool>;
    fn AddDataForStroke(&self, stroke: &::core::option::Option<super::InkStroke>) -> ::windows::core::Result<()>;
    fn AddDataForStrokes(&self, strokes: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>) -> ::windows::core::Result<()>;
    fn ClearDataForAllStrokes(&self) -> ::windows::core::Result<()>;
    fn RemoveDataForStroke(&self, strokeid: u32) -> ::windows::core::Result<()>;
    fn RemoveDataForStrokes(&self, strokeids: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<()>;
    fn ReplaceDataForStroke(&self, stroke: &::core::option::Option<super::InkStroke>) -> ::windows::core::Result<()>;
    fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::Result<()>;
    fn AnalyzeAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzer";
}
#[cfg(feature = "implement_exclusive")]
impl IInkAnalyzerVtbl {
    pub const fn new<Impl: IInkAnalyzerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalyzerVtbl {
        unsafe extern "system" fn AnalysisRoot<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnalysisRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAnalyzing<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAnalyzing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDataForStroke<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddDataForStroke(&*(&stroke as *const <super::InkStroke as ::windows::core::Abi>::Abi as *const <super::InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddDataForStrokes<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddDataForStrokes(&*(&strokes as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkStroke> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkStroke> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearDataForAllStrokes<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearDataForAllStrokes().into()
        }
        unsafe extern "system" fn RemoveDataForStroke<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDataForStroke(strokeid).into()
        }
        unsafe extern "system" fn RemoveDataForStrokes<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeids: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDataForStrokes(&*(&strokeids as *const <super::super::super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReplaceDataForStroke<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReplaceDataForStroke(&*(&stroke as *const <super::InkStroke as ::windows::core::Abi>::Abi as *const <super::InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStrokeDataKind<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrokeDataKind(strokeid, strokekind).into()
        }
        unsafe extern "system" fn AnalyzeAsync<Impl: IInkAnalyzerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnalyzeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalyzer>, base.5, AnalysisRoot::<Impl, OFFSET>, IsAnalyzing::<Impl, OFFSET>, AddDataForStroke::<Impl, OFFSET>, AddDataForStrokes::<Impl, OFFSET>, ClearDataForAllStrokes::<Impl, OFFSET>, RemoveDataForStroke::<Impl, OFFSET>, RemoveDataForStrokes::<Impl, OFFSET>, ReplaceDataForStroke::<Impl, OFFSET>, SetStrokeDataKind::<Impl, OFFSET>, AnalyzeAsync::<Impl, OFFSET>)
    }
}
pub trait IInkAnalyzerFactoryImpl: Sized {
    fn CreateAnalyzer(&self) -> ::windows::core::Result<InkAnalyzer>;
}
impl ::windows::core::RuntimeName for IInkAnalyzerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
}
impl IInkAnalyzerFactoryVtbl {
    pub const fn new<Impl: IInkAnalyzerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkAnalyzerFactoryVtbl {
        unsafe extern "system" fn CreateAnalyzer<Impl: IInkAnalyzerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAnalyzer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkAnalyzerFactory>, base.5, CreateAnalyzer::<Impl, OFFSET>)
    }
}
