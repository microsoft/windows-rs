pub trait ICommandBarElement_Impl: Sized {
    fn IsCompact(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCompact(&mut self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandBarElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ICommandBarElement";
}
impl ICommandBarElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarElement_Vtbl {
        unsafe extern "system" fn IsCompact<Impl: ICommandBarElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCompact<Impl: ICommandBarElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCompact(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarElement, BASE_OFFSET>(),
            IsCompact: IsCompact::<Impl, IMPL_OFFSET>,
            SetIsCompact: SetIsCompact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarElement as ::windows::core::Interface>::IID
    }
}
pub trait ICommandBarElement2_Impl: Sized {
    fn IsInOverflow(&mut self) -> ::windows::core::Result<bool>;
    fn DynamicOverflowOrder(&mut self) -> ::windows::core::Result<i32>;
    fn SetDynamicOverflowOrder(&mut self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandBarElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ICommandBarElement2";
}
impl ICommandBarElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarElement2_Vtbl {
        unsafe extern "system" fn IsInOverflow<Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInOverflow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicOverflowOrder<Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicOverflowOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicOverflowOrder<Impl: ICommandBarElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDynamicOverflowOrder(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarElement2, BASE_OFFSET>(),
            IsInOverflow: IsInOverflow::<Impl, IMPL_OFFSET>,
            DynamicOverflowOrder: DynamicOverflowOrder::<Impl, IMPL_OFFSET>,
            SetDynamicOverflowOrder: SetDynamicOverflowOrder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IInsertionPanel_Impl: Sized {
    fn GetInsertionIndexes(&mut self, position: &super::super::super::Foundation::Point, first: &mut i32, second: &mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInsertionPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IInsertionPanel";
}
#[cfg(feature = "Foundation")]
impl IInsertionPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInsertionPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInsertionPanel_Vtbl {
        unsafe extern "system" fn GetInsertionIndexes<Impl: IInsertionPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, first: *mut i32, second: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInsertionIndexes(&*(&position as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInsertionPanel, BASE_OFFSET>(),
            GetInsertionIndexes: GetInsertionIndexes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInsertionPanel as ::windows::core::Interface>::IID
    }
}
pub trait IItemContainerMapping_Impl: Sized {
    fn ItemFromContainer(&mut self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn IndexFromContainer(&mut self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn ContainerFromIndex(&mut self, index: i32) -> ::windows::core::Result<super::DependencyObject>;
}
impl ::windows::core::RuntimeName for IItemContainerMapping {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IItemContainerMapping";
}
impl IItemContainerMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemContainerMapping_Vtbl {
        unsafe extern "system" fn ItemFromContainer<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemFromContainer(&*(&container as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromItem<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerFromItem(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexFromContainer<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexFromContainer(&*(&container as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromIndex<Impl: IItemContainerMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerFromIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemContainerMapping, BASE_OFFSET>(),
            ItemFromContainer: ItemFromContainer::<Impl, IMPL_OFFSET>,
            ContainerFromItem: ContainerFromItem::<Impl, IMPL_OFFSET>,
            IndexFromContainer: IndexFromContainer::<Impl, IMPL_OFFSET>,
            ContainerFromIndex: ContainerFromIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait INavigate_Impl: Sized {
    fn Navigate(&mut self, sourcepagetype: &super::Interop::TypeName) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for INavigate {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.INavigate";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl INavigate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigate_Vtbl {
        unsafe extern "system" fn Navigate<Impl: INavigate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(&*(&sourcepagetype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INavigate, BASE_OFFSET>(), Navigate: Navigate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigate as ::windows::core::Interface>::IID
    }
}
pub trait IScrollAnchorProvider_Impl: Sized {
    fn CurrentAnchor(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn RegisterAnchorCandidate(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn UnregisterAnchorCandidate(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollAnchorProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IScrollAnchorProvider";
}
impl IScrollAnchorProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollAnchorProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollAnchorProvider_Vtbl {
        unsafe extern "system" fn CurrentAnchor<Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAnchorCandidate<Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterAnchorCandidate(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterAnchorCandidate<Impl: IScrollAnchorProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAnchorCandidate(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollAnchorProvider, BASE_OFFSET>(),
            CurrentAnchor: CurrentAnchor::<Impl, IMPL_OFFSET>,
            RegisterAnchorCandidate: RegisterAnchorCandidate::<Impl, IMPL_OFFSET>,
            UnregisterAnchorCandidate: UnregisterAnchorCandidate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollAnchorProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISemanticZoomInformation_Impl: Sized {
    fn SemanticZoomOwner(&mut self) -> ::windows::core::Result<SemanticZoom>;
    fn SetSemanticZoomOwner(&mut self, value: &::core::option::Option<SemanticZoom>) -> ::windows::core::Result<()>;
    fn IsActiveView(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsActiveView(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomedInView(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsZoomedInView(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InitializeViewChange(&mut self) -> ::windows::core::Result<()>;
    fn CompleteViewChange(&mut self) -> ::windows::core::Result<()>;
    fn MakeVisible(&mut self, item: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeFrom(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeTo(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeFrom(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeTo(&mut self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISemanticZoomInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ISemanticZoomInformation";
}
impl ISemanticZoomInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticZoomInformation_Vtbl {
        unsafe extern "system" fn SemanticZoomOwner<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SemanticZoomOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSemanticZoomOwner<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSemanticZoomOwner(&*(&value as *const <SemanticZoom as ::windows::core::Abi>::Abi as *const <SemanticZoom as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsActiveView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActiveView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsActiveView(value).into()
        }
        unsafe extern "system" fn IsZoomedInView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsZoomedInView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZoomedInView<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsZoomedInView(value).into()
        }
        unsafe extern "system" fn InitializeViewChange<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeViewChange().into()
        }
        unsafe extern "system" fn CompleteViewChange<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteViewChange().into()
        }
        unsafe extern "system" fn MakeVisible<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeVisible(&*(&item as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartViewChangeFrom<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartViewChangeFrom(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartViewChangeTo<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartViewChangeTo(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompleteViewChangeFrom<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteViewChangeFrom(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompleteViewChangeTo<Impl: ISemanticZoomInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteViewChangeTo(&*(&source as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <SemanticZoomLocation as ::windows::core::Abi>::Abi as *const <SemanticZoomLocation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticZoomInformation, BASE_OFFSET>(),
            SemanticZoomOwner: SemanticZoomOwner::<Impl, IMPL_OFFSET>,
            SetSemanticZoomOwner: SetSemanticZoomOwner::<Impl, IMPL_OFFSET>,
            IsActiveView: IsActiveView::<Impl, IMPL_OFFSET>,
            SetIsActiveView: SetIsActiveView::<Impl, IMPL_OFFSET>,
            IsZoomedInView: IsZoomedInView::<Impl, IMPL_OFFSET>,
            SetIsZoomedInView: SetIsZoomedInView::<Impl, IMPL_OFFSET>,
            InitializeViewChange: InitializeViewChange::<Impl, IMPL_OFFSET>,
            CompleteViewChange: CompleteViewChange::<Impl, IMPL_OFFSET>,
            MakeVisible: MakeVisible::<Impl, IMPL_OFFSET>,
            StartViewChangeFrom: StartViewChangeFrom::<Impl, IMPL_OFFSET>,
            StartViewChangeTo: StartViewChangeTo::<Impl, IMPL_OFFSET>,
            CompleteViewChangeFrom: CompleteViewChangeFrom::<Impl, IMPL_OFFSET>,
            CompleteViewChangeTo: CompleteViewChangeTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticZoomInformation as ::windows::core::Interface>::IID
    }
}
