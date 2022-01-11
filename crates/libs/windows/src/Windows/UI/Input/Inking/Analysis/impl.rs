#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisInkBulletImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisInkBulletVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisInkBulletImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisInkBulletVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisInkBulletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisInkBullet>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisInkBullet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisInkDrawingImpl: Sized + IInkAnalysisNodeImpl {
    fn DrawingKind(&self) -> ::windows::core::Result<InkAnalysisDrawingKind>;
    fn Center(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Points(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisInkDrawingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisInkDrawingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisInkDrawingVtbl {
        unsafe extern "system" fn DrawingKind<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisDrawingKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Center<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Points<Impl: IInkAnalysisInkDrawingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisInkDrawing>, ::windows::core::GetTrustLevel, DrawingKind::<Impl, IMPL_OFFSET>, Center::<Impl, IMPL_OFFSET>, Points::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisInkDrawing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisInkWordImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TextAlternates(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisInkWordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisInkWordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisInkWordVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisInkWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TextAlternates<Impl: IInkAnalysisInkWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisInkWord>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>, TextAlternates::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisInkWord as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisLineImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndentLevel(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisLine";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisLineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisLineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisLineVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndentLevel<Impl: IInkAnalysisLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisLine>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>, IndentLevel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisLine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisListItemImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisListItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisListItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisListItemVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisListItem>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisListItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IInkAnalysisNodeImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IInkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IInkAnalysisNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisNodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisNodeVtbl {
        unsafe extern "system" fn Id<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BoundingRect<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RotatedBoundingRect<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Children<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Parent<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStrokeIds<Impl: IInkAnalysisNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInkAnalysisNode>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            Kind::<Impl, IMPL_OFFSET>,
            BoundingRect::<Impl, IMPL_OFFSET>,
            RotatedBoundingRect::<Impl, IMPL_OFFSET>,
            Children::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            GetStrokeIds::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisParagraphImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisParagraphVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisParagraphImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisParagraphVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisParagraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisParagraph>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisParagraph as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisResultVtbl {
        unsafe extern "system" fn Status<Impl: IInkAnalysisResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisRootImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisRootVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisRootImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisRootVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindNodes<Impl: IInkAnalysisRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodekind: InkAnalysisNodeKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisRoot>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>, FindNodes::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInkAnalysisWritingRegionImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalysisWritingRegionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalysisWritingRegionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalysisWritingRegionVtbl {
        unsafe extern "system" fn RecognizedText<Impl: IInkAnalysisWritingRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalysisWritingRegion>, ::windows::core::GetTrustLevel, RecognizedText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalysisWritingRegion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInkAnalyzerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalyzerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalyzerVtbl {
        unsafe extern "system" fn AnalysisRoot<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAnalyzing<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddDataForStroke<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDataForStroke(&*(&stroke as *const <super::InkStroke as ::windows::core::Abi>::Abi as *const <super::InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddDataForStrokes<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDataForStrokes(&*(&strokes as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkStroke> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::InkStroke> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearDataForAllStrokes<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDataForAllStrokes().into()
        }
        unsafe extern "system" fn RemoveDataForStroke<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataForStroke(strokeid).into()
        }
        unsafe extern "system" fn RemoveDataForStrokes<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeids: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataForStrokes(&*(&strokeids as *const <super::super::super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReplaceDataForStroke<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceDataForStroke(&*(&stroke as *const <super::InkStroke as ::windows::core::Abi>::Abi as *const <super::InkStroke as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStrokeDataKind<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDataKind(strokeid, strokekind).into()
        }
        unsafe extern "system" fn AnalyzeAsync<Impl: IInkAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInkAnalyzer>,
            ::windows::core::GetTrustLevel,
            AnalysisRoot::<Impl, IMPL_OFFSET>,
            IsAnalyzing::<Impl, IMPL_OFFSET>,
            AddDataForStroke::<Impl, IMPL_OFFSET>,
            AddDataForStrokes::<Impl, IMPL_OFFSET>,
            ClearDataForAllStrokes::<Impl, IMPL_OFFSET>,
            RemoveDataForStroke::<Impl, IMPL_OFFSET>,
            RemoveDataForStrokes::<Impl, IMPL_OFFSET>,
            ReplaceDataForStroke::<Impl, IMPL_OFFSET>,
            SetStrokeDataKind::<Impl, IMPL_OFFSET>,
            AnalyzeAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalyzer as ::windows::core::Interface>::IID
    }
}
pub trait IInkAnalyzerFactoryImpl: Sized {
    fn CreateAnalyzer(&self) -> ::windows::core::Result<InkAnalyzer>;
}
impl ::windows::core::RuntimeName for IInkAnalyzerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
}
impl IInkAnalyzerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkAnalyzerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkAnalyzerFactoryVtbl {
        unsafe extern "system" fn CreateAnalyzer<Impl: IInkAnalyzerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkAnalyzerFactory>, ::windows::core::GetTrustLevel, CreateAnalyzer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkAnalyzerFactory as ::windows::core::Interface>::IID
    }
}
