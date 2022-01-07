pub trait IAnnotationProviderImpl: Sized {
    fn AnnotationTypeId(&self) -> ::windows::core::Result<i32>;
    fn AnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateTime(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Target(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IAnnotationProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IAnnotationProvider";
}
impl IAnnotationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProviderImpl, const OFFSET: isize>() -> IAnnotationProviderVtbl {
        unsafe extern "system" fn AnnotationTypeId<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnnotationProvider>, ::windows::core::GetTrustLevel, AnnotationTypeId::<Impl, OFFSET>, AnnotationTypeName::<Impl, OFFSET>, Author::<Impl, OFFSET>, DateTime::<Impl, OFFSET>, Target::<Impl, OFFSET>)
    }
}
pub trait ICustomNavigationProviderImpl: Sized {
    fn NavigateCustom(&self, direction: super::Peers::AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for ICustomNavigationProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ICustomNavigationProvider";
}
impl ICustomNavigationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProviderImpl, const OFFSET: isize>() -> ICustomNavigationProviderVtbl {
        unsafe extern "system" fn NavigateCustom<Impl: ICustomNavigationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: super::Peers::AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigateCustom(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICustomNavigationProvider>, ::windows::core::GetTrustLevel, NavigateCustom::<Impl, OFFSET>)
    }
}
pub trait IDockProviderImpl: Sized {
    fn DockPosition(&self) -> ::windows::core::Result<super::DockPosition>;
    fn SetDockPosition(&self, dockposition: super::DockPosition) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDockProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IDockProvider";
}
impl IDockProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProviderImpl, const OFFSET: isize>() -> IDockProviderVtbl {
        unsafe extern "system" fn DockPosition<Impl: IDockProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDockPosition<Impl: IDockProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: super::DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDockPosition(dockposition).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDockProvider>, ::windows::core::GetTrustLevel, DockPosition::<Impl, OFFSET>, SetDockPosition::<Impl, OFFSET>)
    }
}
pub trait IDragProviderImpl: Sized {
    fn IsGrabbed(&self) -> ::windows::core::Result<bool>;
    fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
    fn GetGrabbedItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for IDragProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IDragProvider";
}
impl IDragProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProviderImpl, const OFFSET: isize>() -> IDragProviderVtbl {
        unsafe extern "system" fn IsGrabbed<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffect<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDragProvider>, ::windows::core::GetTrustLevel, IsGrabbed::<Impl, OFFSET>, DropEffect::<Impl, OFFSET>, DropEffects::<Impl, OFFSET>, GetGrabbedItems::<Impl, OFFSET>)
    }
}
pub trait IDropTargetProviderImpl: Sized {
    fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
impl ::windows::core::RuntimeName for IDropTargetProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IDropTargetProvider";
}
impl IDropTargetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProviderImpl, const OFFSET: isize>() -> IDropTargetProviderVtbl {
        unsafe extern "system" fn DropEffect<Impl: IDropTargetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Impl: IDropTargetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropTargetProvider>, ::windows::core::GetTrustLevel, DropEffect::<Impl, OFFSET>, DropEffects::<Impl, OFFSET>)
    }
}
pub trait IExpandCollapseProviderImpl: Sized {
    fn ExpandCollapseState(&self) -> ::windows::core::Result<super::ExpandCollapseState>;
    fn Collapse(&self) -> ::windows::core::Result<()>;
    fn Expand(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IExpandCollapseProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IExpandCollapseProvider";
}
impl IExpandCollapseProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProviderImpl, const OFFSET: isize>() -> IExpandCollapseProviderVtbl {
        unsafe extern "system" fn ExpandCollapseState<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse().into()
        }
        unsafe extern "system" fn Expand<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Expand().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExpandCollapseProvider>, ::windows::core::GetTrustLevel, ExpandCollapseState::<Impl, OFFSET>, Collapse::<Impl, OFFSET>, Expand::<Impl, OFFSET>)
    }
}
pub trait IGridItemProviderImpl: Sized {
    fn Column(&self) -> ::windows::core::Result<i32>;
    fn ColumnSpan(&self) -> ::windows::core::Result<i32>;
    fn ContainingGrid(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn Row(&self) -> ::windows::core::Result<i32>;
    fn RowSpan(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IGridItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IGridItemProvider";
}
impl IGridItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProviderImpl, const OFFSET: isize>() -> IGridItemProviderVtbl {
        unsafe extern "system" fn Column<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Column() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpan<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGrid<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Row<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Row() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpan<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridItemProvider>, ::windows::core::GetTrustLevel, Column::<Impl, OFFSET>, ColumnSpan::<Impl, OFFSET>, ContainingGrid::<Impl, OFFSET>, Row::<Impl, OFFSET>, RowSpan::<Impl, OFFSET>)
    }
}
pub trait IGridProviderImpl: Sized {
    fn ColumnCount(&self) -> ::windows::core::Result<i32>;
    fn RowCount(&self) -> ::windows::core::Result<i32>;
    fn GetItem(&self, row: i32, column: i32) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IGridProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IGridProvider";
}
impl IGridProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProviderImpl, const OFFSET: isize>() -> IGridProviderVtbl {
        unsafe extern "system" fn ColumnCount<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCount<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(row, column) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridProvider>, ::windows::core::GetTrustLevel, ColumnCount::<Impl, OFFSET>, RowCount::<Impl, OFFSET>, GetItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIRawElementProviderSimpleImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIRawElementProviderSimple {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IIRawElementProviderSimple";
}
#[cfg(feature = "implement_exclusive")]
impl IIRawElementProviderSimpleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIRawElementProviderSimpleImpl, const OFFSET: isize>() -> IIRawElementProviderSimpleVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIRawElementProviderSimple>, ::windows::core::GetTrustLevel)
    }
}
pub trait IInvokeProviderImpl: Sized {
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInvokeProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IInvokeProvider";
}
impl IInvokeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProviderImpl, const OFFSET: isize>() -> IInvokeProviderVtbl {
        unsafe extern "system" fn Invoke<Impl: IInvokeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInvokeProvider>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>)
    }
}
pub trait IItemContainerProviderImpl: Sized {
    fn FindItemByProperty(&self, startafter: &::core::option::Option<IRawElementProviderSimple>, automationproperty: &::core::option::Option<super::AutomationProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IItemContainerProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IItemContainerProvider";
}
impl IItemContainerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProviderImpl, const OFFSET: isize>() -> IItemContainerProviderVtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IItemContainerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startafter: ::windows::core::RawPtr, automationproperty: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByProperty(
                &*(&startafter as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType),
                &*(&automationproperty as *const <super::AutomationProperty as ::windows::core::Abi>::Abi as *const <super::AutomationProperty as ::windows::core::DefaultType>::DefaultType),
                &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemContainerProvider>, ::windows::core::GetTrustLevel, FindItemByProperty::<Impl, OFFSET>)
    }
}
pub trait IMultipleViewProviderImpl: Sized {
    fn CurrentView(&self) -> ::windows::core::Result<i32>;
    fn GetSupportedViews(&self) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn GetViewName(&self, viewid: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrentView(&self, viewid: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMultipleViewProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IMultipleViewProvider";
}
impl IMultipleViewProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProviderImpl, const OFFSET: isize>() -> IMultipleViewProviderVtbl {
        unsafe extern "system" fn CurrentView<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedViews<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewName<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewName(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentView(viewid).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultipleViewProvider>, ::windows::core::GetTrustLevel, CurrentView::<Impl, OFFSET>, GetSupportedViews::<Impl, OFFSET>, GetViewName::<Impl, OFFSET>, SetCurrentView::<Impl, OFFSET>)
    }
}
pub trait IObjectModelProviderImpl: Sized {
    fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IObjectModelProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IObjectModelProvider";
}
impl IObjectModelProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProviderImpl, const OFFSET: isize>() -> IObjectModelProviderVtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IObjectModelProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectModelProvider>, ::windows::core::GetTrustLevel, GetUnderlyingObjectModel::<Impl, OFFSET>)
    }
}
pub trait IRangeValueProviderImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn LargeChange(&self) -> ::windows::core::Result<f64>;
    fn Maximum(&self) -> ::windows::core::Result<f64>;
    fn Minimum(&self) -> ::windows::core::Result<f64>;
    fn SmallChange(&self) -> ::windows::core::Result<f64>;
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRangeValueProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IRangeValueProvider";
}
impl IRangeValueProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProviderImpl, const OFFSET: isize>() -> IRangeValueProviderVtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximum<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimum<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRangeValueProvider>, ::windows::core::GetTrustLevel, IsReadOnly::<Impl, OFFSET>, LargeChange::<Impl, OFFSET>, Maximum::<Impl, OFFSET>, Minimum::<Impl, OFFSET>, SmallChange::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
pub trait IScrollItemProviderImpl: Sized {
    fn ScrollIntoView(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IScrollItemProvider";
}
impl IScrollItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProviderImpl, const OFFSET: isize>() -> IScrollItemProviderVtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IScrollItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollItemProvider>, ::windows::core::GetTrustLevel, ScrollIntoView::<Impl, OFFSET>)
    }
}
pub trait IScrollProviderImpl: Sized {
    fn HorizontallyScrollable(&self) -> ::windows::core::Result<bool>;
    fn HorizontalScrollPercent(&self) -> ::windows::core::Result<f64>;
    fn HorizontalViewSize(&self) -> ::windows::core::Result<f64>;
    fn VerticallyScrollable(&self) -> ::windows::core::Result<bool>;
    fn VerticalScrollPercent(&self) -> ::windows::core::Result<f64>;
    fn VerticalViewSize(&self) -> ::windows::core::Result<f64>;
    fn Scroll(&self, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows::core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IScrollProvider";
}
impl IScrollProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProviderImpl, const OFFSET: isize>() -> IScrollProviderVtbl {
        unsafe extern "system" fn HorizontallyScrollable<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSize<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scroll<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(horizontalamount, verticalamount).into()
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollPercent(horizontalpercent, verticalpercent).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IScrollProvider>,
            ::windows::core::GetTrustLevel,
            HorizontallyScrollable::<Impl, OFFSET>,
            HorizontalScrollPercent::<Impl, OFFSET>,
            HorizontalViewSize::<Impl, OFFSET>,
            VerticallyScrollable::<Impl, OFFSET>,
            VerticalScrollPercent::<Impl, OFFSET>,
            VerticalViewSize::<Impl, OFFSET>,
            Scroll::<Impl, OFFSET>,
            SetScrollPercent::<Impl, OFFSET>,
        )
    }
}
pub trait ISelectionItemProviderImpl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SelectionContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn AddToSelection(&self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows::core::Result<()>;
    fn Select(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISelectionItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISelectionItemProvider";
}
impl ISelectionItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProviderImpl, const OFFSET: isize>() -> ISelectionItemProviderVtbl {
        unsafe extern "system" fn IsSelected<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainer<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSelection<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn Select<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionItemProvider>, ::windows::core::GetTrustLevel, IsSelected::<Impl, OFFSET>, SelectionContainer::<Impl, OFFSET>, AddToSelection::<Impl, OFFSET>, RemoveFromSelection::<Impl, OFFSET>, Select::<Impl, OFFSET>)
    }
}
pub trait ISelectionProviderImpl: Sized {
    fn CanSelectMultiple(&self) -> ::windows::core::Result<bool>;
    fn IsSelectionRequired(&self) -> ::windows::core::Result<bool>;
    fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ISelectionProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISelectionProvider";
}
impl ISelectionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProviderImpl, const OFFSET: isize>() -> ISelectionProviderVtbl {
        unsafe extern "system" fn CanSelectMultiple<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionProvider>, ::windows::core::GetTrustLevel, CanSelectMultiple::<Impl, OFFSET>, IsSelectionRequired::<Impl, OFFSET>, GetSelection::<Impl, OFFSET>)
    }
}
pub trait ISpreadsheetItemProviderImpl: Sized {
    fn Formula(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAnnotationObjects(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetAnnotationTypes(&self) -> ::windows::core::Result<::windows::core::Array<super::AnnotationType>>;
}
impl ::windows::core::RuntimeName for ISpreadsheetItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISpreadsheetItemProvider";
}
impl ISpreadsheetItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>() -> ISpreadsheetItemProviderVtbl {
        unsafe extern "system" fn Formula<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Formula() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationTypes<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpreadsheetItemProvider>, ::windows::core::GetTrustLevel, Formula::<Impl, OFFSET>, GetAnnotationObjects::<Impl, OFFSET>, GetAnnotationTypes::<Impl, OFFSET>)
    }
}
pub trait ISpreadsheetProviderImpl: Sized {
    fn GetItemByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for ISpreadsheetProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISpreadsheetProvider";
}
impl ISpreadsheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProviderImpl, const OFFSET: isize>() -> ISpreadsheetProviderVtbl {
        unsafe extern "system" fn GetItemByName<Impl: ISpreadsheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpreadsheetProvider>, ::windows::core::GetTrustLevel, GetItemByName::<Impl, OFFSET>)
    }
}
pub trait IStylesProviderImpl: Sized {
    fn ExtendedProperties(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FillColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn FillPatternColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn FillPatternStyle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Shape(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StyleId(&self) -> ::windows::core::Result<i32>;
    fn StyleName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStylesProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IStylesProvider";
}
impl IStylesProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProviderImpl, const OFFSET: isize>() -> IStylesProviderVtbl {
        unsafe extern "system" fn ExtendedProperties<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColor<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColor<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyle<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleId<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleName<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStylesProvider>, ::windows::core::GetTrustLevel, ExtendedProperties::<Impl, OFFSET>, FillColor::<Impl, OFFSET>, FillPatternColor::<Impl, OFFSET>, FillPatternStyle::<Impl, OFFSET>, Shape::<Impl, OFFSET>, StyleId::<Impl, OFFSET>, StyleName::<Impl, OFFSET>)
    }
}
pub trait ISynchronizedInputProviderImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn StartListening(&self, inputtype: super::SynchronizedInputType) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISynchronizedInputProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISynchronizedInputProvider";
}
impl ISynchronizedInputProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>() -> ISynchronizedInputProviderVtbl {
        unsafe extern "system" fn Cancel<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn StartListening<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: super::SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListening(inputtype).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizedInputProvider>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, StartListening::<Impl, OFFSET>)
    }
}
pub trait ITableItemProviderImpl: Sized {
    fn GetColumnHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetRowHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ITableItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITableItemProvider";
}
impl ITableItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProviderImpl, const OFFSET: isize>() -> ITableItemProviderVtbl {
        unsafe extern "system" fn GetColumnHeaderItems<Impl: ITableItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowHeaderItems<Impl: ITableItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITableItemProvider>, ::windows::core::GetTrustLevel, GetColumnHeaderItems::<Impl, OFFSET>, GetRowHeaderItems::<Impl, OFFSET>)
    }
}
pub trait ITableProviderImpl: Sized {
    fn RowOrColumnMajor(&self) -> ::windows::core::Result<super::RowOrColumnMajor>;
    fn GetColumnHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetRowHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ITableProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITableProvider";
}
impl ITableProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProviderImpl, const OFFSET: isize>() -> ITableProviderVtbl {
        unsafe extern "system" fn RowOrColumnMajor<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowHeaders<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITableProvider>, ::windows::core::GetTrustLevel, RowOrColumnMajor::<Impl, OFFSET>, GetColumnHeaders::<Impl, OFFSET>, GetRowHeaders::<Impl, OFFSET>)
    }
}
pub trait ITextChildProviderImpl: Sized {
    fn TextContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn TextRange(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
impl ::windows::core::RuntimeName for ITextChildProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextChildProvider";
}
impl ITextChildProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProviderImpl, const OFFSET: isize>() -> ITextChildProviderVtbl {
        unsafe extern "system" fn TextContainer<Impl: ITextChildProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Impl: ITextChildProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextChildProvider>, ::windows::core::GetTrustLevel, TextContainer::<Impl, OFFSET>, TextRange::<Impl, OFFSET>)
    }
}
pub trait ITextEditProviderImpl: Sized + ITextProviderImpl {
    fn GetActiveComposition(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
impl ::windows::core::RuntimeName for ITextEditProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextEditProvider";
}
impl ITextEditProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProviderImpl, const OFFSET: isize>() -> ITextEditProviderVtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: ITextEditProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Impl: ITextEditProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextEditProvider>, ::windows::core::GetTrustLevel, GetActiveComposition::<Impl, OFFSET>, GetConversionTarget::<Impl, OFFSET>)
    }
}
pub trait ITextProviderImpl: Sized {
    fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&self) -> ::windows::core::Result<super::SupportedTextSelection>;
    fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>>;
    fn GetVisibleRanges(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>>;
    fn RangeFromChild(&self, childelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&self, screenlocation: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<ITextRangeProvider>;
}
impl ::windows::core::RuntimeName for ITextProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextProvider";
}
impl ITextProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProviderImpl, const OFFSET: isize>() -> ITextProviderVtbl {
        unsafe extern "system" fn DocumentRange<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromChild(&*(&childelement as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, screenlocation: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(&*(&screenlocation as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextProvider>, ::windows::core::GetTrustLevel, DocumentRange::<Impl, OFFSET>, SupportedTextSelection::<Impl, OFFSET>, GetSelection::<Impl, OFFSET>, GetVisibleRanges::<Impl, OFFSET>, RangeFromChild::<Impl, OFFSET>, RangeFromPoint::<Impl, OFFSET>)
    }
}
pub trait ITextProvider2Impl: Sized + ITextProviderImpl {
    fn RangeFromAnnotation(&self, annotationelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: &mut bool) -> ::windows::core::Result<ITextRangeProvider>;
}
impl ::windows::core::RuntimeName for ITextProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextProvider2";
}
impl ITextProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2Impl, const OFFSET: isize>() -> ITextProvider2Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: ITextProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromAnnotation(&*(&annotationelement as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Impl: ITextProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretRange(::core::mem::transmute_copy(&isactive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextProvider2>, ::windows::core::GetTrustLevel, RangeFromAnnotation::<Impl, OFFSET>, GetCaretRange::<Impl, OFFSET>)
    }
}
pub trait ITextRangeProviderImpl: Sized {
    fn Clone(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn Compare(&self, textrangeprovider: &::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<bool>;
    fn CompareEndpoints(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: &::core::option::Option<ITextRangeProvider>, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, unit: super::Text::TextUnit) -> ::windows::core::Result<()>;
    fn FindAttribute(&self, attributeid: i32, value: &::core::option::Option<::windows::core::IInspectable>, backward: bool) -> ::windows::core::Result<ITextRangeProvider>;
    fn FindText(&self, text: &::windows::core::HSTRING, backward: bool, ignorecase: bool) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&self, attributeid: i32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetBoundingRectangles(&self, returnvalue: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()>;
    fn GetEnclosingElement(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByRange(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: &::core::option::Option<ITextRangeProvider>, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<()>;
    fn Select(&self) -> ::windows::core::Result<()>;
    fn AddToSelection(&self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: bool) -> ::windows::core::Result<()>;
    fn GetChildren(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
impl ::windows::core::RuntimeName for ITextRangeProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextRangeProvider";
}
impl ITextRangeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProviderImpl, const OFFSET: isize>() -> ITextRangeProviderVtbl {
        unsafe extern "system" fn Clone<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrangeprovider: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&textrangeprovider as *const <ITextRangeProvider as ::windows::core::Abi>::Abi as *const <ITextRangeProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: ::windows::core::RawPtr, targetendpoint: super::Text::TextPatternRangeEndpoint, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEndpoints(endpoint, &*(&textrangeprovider as *const <ITextRangeProvider as ::windows::core::Abi>::Abi as *const <ITextRangeProvider as ::windows::core::DefaultType>::DefaultType), targetendpoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: super::Text::TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExpandToEnclosingUnit(unit).into()
        }
        unsafe extern "system" fn FindAttribute<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, value: *mut ::core::ffi::c_void, backward: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAttribute(attributeid, &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), backward) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, backward: bool, ignorecase: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), backward, ignorecase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(attributeid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnValue_array_size: *mut u32, returnvalue: *mut *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBoundingRectangles(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&returnvalue), returnValue_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(maxlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByUnit(endpoint, unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: ::windows::core::RawPtr, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveEndpointByRange(endpoint, &*(&textrangeprovider as *const <ITextRangeProvider as ::windows::core::Abi>::Abi as *const <ITextRangeProvider as ::windows::core::DefaultType>::DefaultType), targetendpoint).into()
        }
        unsafe extern "system" fn Select<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView(aligntotop).into()
        }
        unsafe extern "system" fn GetChildren<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextRangeProvider>,
            ::windows::core::GetTrustLevel,
            Clone::<Impl, OFFSET>,
            Compare::<Impl, OFFSET>,
            CompareEndpoints::<Impl, OFFSET>,
            ExpandToEnclosingUnit::<Impl, OFFSET>,
            FindAttribute::<Impl, OFFSET>,
            FindText::<Impl, OFFSET>,
            GetAttributeValue::<Impl, OFFSET>,
            GetBoundingRectangles::<Impl, OFFSET>,
            GetEnclosingElement::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            MoveEndpointByUnit::<Impl, OFFSET>,
            MoveEndpointByRange::<Impl, OFFSET>,
            Select::<Impl, OFFSET>,
            AddToSelection::<Impl, OFFSET>,
            RemoveFromSelection::<Impl, OFFSET>,
            ScrollIntoView::<Impl, OFFSET>,
            GetChildren::<Impl, OFFSET>,
        )
    }
}
pub trait ITextRangeProvider2Impl: Sized + ITextRangeProviderImpl {
    fn ShowContextMenu(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITextRangeProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextRangeProvider2";
}
impl ITextRangeProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2Impl, const OFFSET: isize>() -> ITextRangeProvider2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: ITextRangeProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextRangeProvider2>, ::windows::core::GetTrustLevel, ShowContextMenu::<Impl, OFFSET>)
    }
}
pub trait IToggleProviderImpl: Sized {
    fn ToggleState(&self) -> ::windows::core::Result<super::ToggleState>;
    fn Toggle(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IToggleProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IToggleProvider";
}
impl IToggleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProviderImpl, const OFFSET: isize>() -> IToggleProviderVtbl {
        unsafe extern "system" fn ToggleState<Impl: IToggleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Toggle<Impl: IToggleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Toggle().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleProvider>, ::windows::core::GetTrustLevel, ToggleState::<Impl, OFFSET>, Toggle::<Impl, OFFSET>)
    }
}
pub trait ITransformProviderImpl: Sized {
    fn CanMove(&self) -> ::windows::core::Result<bool>;
    fn CanResize(&self) -> ::windows::core::Result<bool>;
    fn CanRotate(&self) -> ::windows::core::Result<bool>;
    fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()>;
    fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransformProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITransformProvider";
}
impl ITransformProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProviderImpl, const OFFSET: isize>() -> ITransformProviderVtbl {
        unsafe extern "system" fn CanMove<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResize<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotate<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(x, y).into()
        }
        unsafe extern "system" fn Resize<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(width, height).into()
        }
        unsafe extern "system" fn Rotate<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rotate(degrees).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransformProvider>, ::windows::core::GetTrustLevel, CanMove::<Impl, OFFSET>, CanResize::<Impl, OFFSET>, CanRotate::<Impl, OFFSET>, Move::<Impl, OFFSET>, Resize::<Impl, OFFSET>, Rotate::<Impl, OFFSET>)
    }
}
pub trait ITransformProvider2Impl: Sized + ITransformProviderImpl {
    fn CanZoom(&self) -> ::windows::core::Result<bool>;
    fn ZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn MaxZoom(&self) -> ::windows::core::Result<f64>;
    fn MinZoom(&self) -> ::windows::core::Result<f64>;
    fn Zoom(&self, zoom: f64) -> ::windows::core::Result<()>;
    fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransformProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITransformProvider2";
}
impl ITransformProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2Impl, const OFFSET: isize>() -> ITransformProvider2Vtbl {
        unsafe extern "system" fn CanZoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxZoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinZoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Zoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Zoom(zoom).into()
        }
        unsafe extern "system" fn ZoomByUnit<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: super::ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ZoomByUnit(zoomunit).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransformProvider2>, ::windows::core::GetTrustLevel, CanZoom::<Impl, OFFSET>, ZoomLevel::<Impl, OFFSET>, MaxZoom::<Impl, OFFSET>, MinZoom::<Impl, OFFSET>, Zoom::<Impl, OFFSET>, ZoomByUnit::<Impl, OFFSET>)
    }
}
pub trait IValueProviderImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IValueProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IValueProvider";
}
impl IValueProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProviderImpl, const OFFSET: isize>() -> IValueProviderVtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IValueProvider>, ::windows::core::GetTrustLevel, IsReadOnly::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
pub trait IVirtualizedItemProviderImpl: Sized {
    fn Realize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVirtualizedItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IVirtualizedItemProvider";
}
impl IVirtualizedItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProviderImpl, const OFFSET: isize>() -> IVirtualizedItemProviderVtbl {
        unsafe extern "system" fn Realize<Impl: IVirtualizedItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Realize().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVirtualizedItemProvider>, ::windows::core::GetTrustLevel, Realize::<Impl, OFFSET>)
    }
}
pub trait IWindowProviderImpl: Sized {
    fn IsModal(&self) -> ::windows::core::Result<bool>;
    fn IsTopmost(&self) -> ::windows::core::Result<bool>;
    fn Maximizable(&self) -> ::windows::core::Result<bool>;
    fn Minimizable(&self) -> ::windows::core::Result<bool>;
    fn InteractionState(&self) -> ::windows::core::Result<super::WindowInteractionState>;
    fn VisualState(&self) -> ::windows::core::Result<super::WindowVisualState>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn SetVisualState(&self, state: super::WindowVisualState) -> ::windows::core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IWindowProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IWindowProvider";
}
impl IWindowProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProviderImpl, const OFFSET: isize>() -> IWindowProviderVtbl {
        unsafe extern "system" fn IsModal<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmost<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximizable<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maximizable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimizable<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minimizable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InteractionState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisualState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetVisualState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: super::WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualState(state).into()
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForInputIdle(milliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWindowProvider>,
            ::windows::core::GetTrustLevel,
            IsModal::<Impl, OFFSET>,
            IsTopmost::<Impl, OFFSET>,
            Maximizable::<Impl, OFFSET>,
            Minimizable::<Impl, OFFSET>,
            InteractionState::<Impl, OFFSET>,
            VisualState::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            SetVisualState::<Impl, OFFSET>,
            WaitForInputIdle::<Impl, OFFSET>,
        )
    }
}
