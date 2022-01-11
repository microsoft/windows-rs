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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnnotationProvider>, ::windows::core::GetTrustLevel, AnnotationTypeId::<Impl, IMPL_OFFSET>, AnnotationTypeName::<Impl, IMPL_OFFSET>, Author::<Impl, IMPL_OFFSET>, DateTime::<Impl, IMPL_OFFSET>, Target::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnnotationProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub trait ICustomNavigationProviderImpl: Sized {
    fn NavigateCustom(&self, direction: super::Peers::AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "UI_Xaml_Automation_Peers")]
impl ::windows::core::RuntimeName for ICustomNavigationProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ICustomNavigationProvider";
}
#[cfg(feature = "UI_Xaml_Automation_Peers")]
impl ICustomNavigationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomNavigationProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICustomNavigationProvider>, ::windows::core::GetTrustLevel, NavigateCustom::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomNavigationProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDockProvider>, ::windows::core::GetTrustLevel, DockPosition::<Impl, IMPL_OFFSET>, SetDockPosition::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDockProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDragProvider>, ::windows::core::GetTrustLevel, IsGrabbed::<Impl, IMPL_OFFSET>, DropEffect::<Impl, IMPL_OFFSET>, DropEffects::<Impl, IMPL_OFFSET>, GetGrabbedItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropTargetProvider>, ::windows::core::GetTrustLevel, DropEffect::<Impl, IMPL_OFFSET>, DropEffects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapseProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExpandCollapseProvider>, ::windows::core::GetTrustLevel, ExpandCollapseState::<Impl, IMPL_OFFSET>, Collapse::<Impl, IMPL_OFFSET>, Expand::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpandCollapseProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridItemProvider>, ::windows::core::GetTrustLevel, Column::<Impl, IMPL_OFFSET>, ColumnSpan::<Impl, IMPL_OFFSET>, ContainingGrid::<Impl, IMPL_OFFSET>, Row::<Impl, IMPL_OFFSET>, RowSpan::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridItemProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridProvider>, ::windows::core::GetTrustLevel, ColumnCount::<Impl, IMPL_OFFSET>, RowCount::<Impl, IMPL_OFFSET>, GetItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIRawElementProviderSimpleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIRawElementProviderSimpleVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIRawElementProviderSimple>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIRawElementProviderSimple as ::windows::core::Interface>::IID
    }
}
pub trait IInvokeProviderImpl: Sized {
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInvokeProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IInvokeProvider";
}
impl IInvokeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInvokeProviderVtbl {
        unsafe extern "system" fn Invoke<Impl: IInvokeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInvokeProvider>, ::windows::core::GetTrustLevel, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvokeProvider as ::windows::core::Interface>::IID
    }
}
pub trait IItemContainerProviderImpl: Sized {
    fn FindItemByProperty(&self, startafter: &::core::option::Option<IRawElementProviderSimple>, automationproperty: &::core::option::Option<super::AutomationProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for IItemContainerProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IItemContainerProvider";
}
impl IItemContainerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemContainerProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemContainerProvider>, ::windows::core::GetTrustLevel, FindItemByProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultipleViewProvider>, ::windows::core::GetTrustLevel, CurrentView::<Impl, IMPL_OFFSET>, GetSupportedViews::<Impl, IMPL_OFFSET>, GetViewName::<Impl, IMPL_OFFSET>, SetCurrentView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleViewProvider as ::windows::core::Interface>::IID
    }
}
pub trait IObjectModelProviderImpl: Sized {
    fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IObjectModelProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IObjectModelProvider";
}
impl IObjectModelProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectModelProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectModelProvider>, ::windows::core::GetTrustLevel, GetUnderlyingObjectModel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectModelProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValueProviderVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRangeValueProvider>,
            ::windows::core::GetTrustLevel,
            IsReadOnly::<Impl, IMPL_OFFSET>,
            LargeChange::<Impl, IMPL_OFFSET>,
            Maximum::<Impl, IMPL_OFFSET>,
            Minimum::<Impl, IMPL_OFFSET>,
            SmallChange::<Impl, IMPL_OFFSET>,
            Value::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeValueProvider as ::windows::core::Interface>::IID
    }
}
pub trait IScrollItemProviderImpl: Sized {
    fn ScrollIntoView(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IScrollItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IScrollItemProvider";
}
impl IScrollItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollItemProviderVtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IScrollItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollItemProvider>, ::windows::core::GetTrustLevel, ScrollIntoView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollItemProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollProviderVtbl {
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IScrollProvider>,
            ::windows::core::GetTrustLevel,
            HorizontallyScrollable::<Impl, IMPL_OFFSET>,
            HorizontalScrollPercent::<Impl, IMPL_OFFSET>,
            HorizontalViewSize::<Impl, IMPL_OFFSET>,
            VerticallyScrollable::<Impl, IMPL_OFFSET>,
            VerticalScrollPercent::<Impl, IMPL_OFFSET>,
            VerticalViewSize::<Impl, IMPL_OFFSET>,
            Scroll::<Impl, IMPL_OFFSET>,
            SetScrollPercent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionItemProvider>, ::windows::core::GetTrustLevel, IsSelected::<Impl, IMPL_OFFSET>, SelectionContainer::<Impl, IMPL_OFFSET>, AddToSelection::<Impl, IMPL_OFFSET>, RemoveFromSelection::<Impl, IMPL_OFFSET>, Select::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionItemProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionProvider>, ::windows::core::GetTrustLevel, CanSelectMultiple::<Impl, IMPL_OFFSET>, IsSelectionRequired::<Impl, IMPL_OFFSET>, GetSelection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpreadsheetItemProvider>, ::windows::core::GetTrustLevel, Formula::<Impl, IMPL_OFFSET>, GetAnnotationObjects::<Impl, IMPL_OFFSET>, GetAnnotationTypes::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetItemProvider as ::windows::core::Interface>::IID
    }
}
pub trait ISpreadsheetProviderImpl: Sized {
    fn GetItemByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl ::windows::core::RuntimeName for ISpreadsheetProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ISpreadsheetProvider";
}
impl ISpreadsheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpreadsheetProvider>, ::windows::core::GetTrustLevel, GetItemByName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesProviderVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStylesProvider>,
            ::windows::core::GetTrustLevel,
            ExtendedProperties::<Impl, IMPL_OFFSET>,
            FillColor::<Impl, IMPL_OFFSET>,
            FillPatternColor::<Impl, IMPL_OFFSET>,
            FillPatternStyle::<Impl, IMPL_OFFSET>,
            Shape::<Impl, IMPL_OFFSET>,
            StyleId::<Impl, IMPL_OFFSET>,
            StyleName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylesProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizedInputProviderVtbl {
        unsafe extern "system" fn Cancel<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn StartListening<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: super::SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListening(inputtype).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizedInputProvider>, ::windows::core::GetTrustLevel, Cancel::<Impl, IMPL_OFFSET>, StartListening::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizedInputProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITableItemProvider>, ::windows::core::GetTrustLevel, GetColumnHeaderItems::<Impl, IMPL_OFFSET>, GetRowHeaderItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableItemProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITableProvider>, ::windows::core::GetTrustLevel, RowOrColumnMajor::<Impl, IMPL_OFFSET>, GetColumnHeaders::<Impl, IMPL_OFFSET>, GetRowHeaders::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextChildProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextChildProvider>, ::windows::core::GetTrustLevel, TextContainer::<Impl, IMPL_OFFSET>, TextRange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextChildProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ITextEditProviderImpl: Sized + ITextProviderImpl {
    fn GetActiveComposition(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ITextEditProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextEditProvider";
}
#[cfg(feature = "Foundation")]
impl ITextEditProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextEditProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextEditProvider>, ::windows::core::GetTrustLevel, GetActiveComposition::<Impl, IMPL_OFFSET>, GetConversionTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextEditProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ITextProviderImpl: Sized {
    fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&self) -> ::windows::core::Result<super::SupportedTextSelection>;
    fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>>;
    fn GetVisibleRanges(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>>;
    fn RangeFromChild(&self, childelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&self, screenlocation: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ITextProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextProvider";
}
#[cfg(feature = "Foundation")]
impl ITextProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextProviderVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextProvider>,
            ::windows::core::GetTrustLevel,
            DocumentRange::<Impl, IMPL_OFFSET>,
            SupportedTextSelection::<Impl, IMPL_OFFSET>,
            GetSelection::<Impl, IMPL_OFFSET>,
            GetVisibleRanges::<Impl, IMPL_OFFSET>,
            RangeFromChild::<Impl, IMPL_OFFSET>,
            RangeFromPoint::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ITextProvider2Impl: Sized + ITextProviderImpl {
    fn RangeFromAnnotation(&self, annotationelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: &mut bool) -> ::windows::core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ITextProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextProvider2";
}
#[cfg(feature = "Foundation")]
impl ITextProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextProvider2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextProvider2>, ::windows::core::GetTrustLevel, RangeFromAnnotation::<Impl, IMPL_OFFSET>, GetCaretRange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
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
#[cfg(feature = "UI_Xaml_Automation_Text")]
impl ::windows::core::RuntimeName for ITextRangeProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextRangeProvider";
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
impl ITextRangeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeProviderVtbl {
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextRangeProvider>,
            ::windows::core::GetTrustLevel,
            Clone::<Impl, IMPL_OFFSET>,
            Compare::<Impl, IMPL_OFFSET>,
            CompareEndpoints::<Impl, IMPL_OFFSET>,
            ExpandToEnclosingUnit::<Impl, IMPL_OFFSET>,
            FindAttribute::<Impl, IMPL_OFFSET>,
            FindText::<Impl, IMPL_OFFSET>,
            GetAttributeValue::<Impl, IMPL_OFFSET>,
            GetBoundingRectangles::<Impl, IMPL_OFFSET>,
            GetEnclosingElement::<Impl, IMPL_OFFSET>,
            GetText::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            MoveEndpointByUnit::<Impl, IMPL_OFFSET>,
            MoveEndpointByRange::<Impl, IMPL_OFFSET>,
            Select::<Impl, IMPL_OFFSET>,
            AddToSelection::<Impl, IMPL_OFFSET>,
            RemoveFromSelection::<Impl, IMPL_OFFSET>,
            ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetChildren::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub trait ITextRangeProvider2Impl: Sized + ITextRangeProviderImpl {
    fn ShowContextMenu(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
impl ::windows::core::RuntimeName for ITextRangeProvider2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.ITextRangeProvider2";
}
#[cfg(feature = "UI_Xaml_Automation_Text")]
impl ITextRangeProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeProvider2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: ITextRangeProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextRangeProvider2>, ::windows::core::GetTrustLevel, ShowContextMenu::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleProvider>, ::windows::core::GetTrustLevel, ToggleState::<Impl, IMPL_OFFSET>, Toggle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransformProvider>, ::windows::core::GetTrustLevel, CanMove::<Impl, IMPL_OFFSET>, CanResize::<Impl, IMPL_OFFSET>, CanRotate::<Impl, IMPL_OFFSET>, Move::<Impl, IMPL_OFFSET>, Resize::<Impl, IMPL_OFFSET>, Rotate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformProvider2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransformProvider2>, ::windows::core::GetTrustLevel, CanZoom::<Impl, IMPL_OFFSET>, ZoomLevel::<Impl, IMPL_OFFSET>, MaxZoom::<Impl, IMPL_OFFSET>, MinZoom::<Impl, IMPL_OFFSET>, Zoom::<Impl, IMPL_OFFSET>, ZoomByUnit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IValueProvider>, ::windows::core::GetTrustLevel, IsReadOnly::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueProvider as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualizedItemProviderImpl: Sized {
    fn Realize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVirtualizedItemProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IVirtualizedItemProvider";
}
impl IVirtualizedItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualizedItemProviderVtbl {
        unsafe extern "system" fn Realize<Impl: IVirtualizedItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Realize().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVirtualizedItemProvider>, ::windows::core::GetTrustLevel, Realize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizedItemProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowProviderVtbl {
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWindowProvider>,
            ::windows::core::GetTrustLevel,
            IsModal::<Impl, IMPL_OFFSET>,
            IsTopmost::<Impl, IMPL_OFFSET>,
            Maximizable::<Impl, IMPL_OFFSET>,
            Minimizable::<Impl, IMPL_OFFSET>,
            InteractionState::<Impl, IMPL_OFFSET>,
            VisualState::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            SetVisualState::<Impl, IMPL_OFFSET>,
            WaitForInputIdle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowProvider as ::windows::core::Interface>::IID
    }
}
